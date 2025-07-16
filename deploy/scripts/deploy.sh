#!/bin/bash
set -e

# Fantasy Football Draft Board Deployment Script
# This script provides one-button deployment functionality

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
ENVIRONMENT="production"
GIT_BRANCH="main"
FORCE_RECREATE=false
SKIP_INFRASTRUCTURE=false
SKIP_APPLICATION=false
AWS_REGION="us-east-1"

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to show usage
show_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  -e, --environment    Environment to deploy to (default: production)"
    echo "  -b, --branch         Git branch to deploy (default: main)"
    echo "  -f, --force          Force recreate infrastructure"
    echo "  -s, --skip-infra     Skip infrastructure deployment"
    echo "  -a, --skip-app       Skip application deployment"
    echo "  -r, --region         AWS region (default: us-east-1)"
    echo "  -h, --help           Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0                           # Deploy main branch to production"
    echo "  $0 -e staging -b develop     # Deploy develop branch to staging"
    echo "  $0 -f                        # Force recreate infrastructure"
    echo "  $0 -s                        # Skip infrastructure, deploy app only"
}

# Function to check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    # Check if required tools are installed
    local tools=("terraform" "aws" "docker" "git" "curl" "ssh")
    
    for tool in "${tools[@]}"; do
        if ! command -v "$tool" &> /dev/null; then
            print_error "$tool is not installed or not in PATH"
            exit 1
        fi
    done
    
    # Check if AWS credentials are configured
    if ! aws sts get-caller-identity &> /dev/null; then
        print_error "AWS credentials not configured. Run 'aws configure' first."
        exit 1
    fi
    
    # Check if SSH key exists
    if [ ! -f "$HOME/.ssh/id_rsa" ]; then
        print_error "SSH private key not found at $HOME/.ssh/id_rsa"
        exit 1
    fi
    
    print_success "Prerequisites check passed"
}

# Function to validate environment variables
validate_environment() {
    print_status "Validating environment..."
    
    # Required environment variables
    local required_vars=("SSH_PUBLIC_KEY")
    
    for var in "${required_vars[@]}"; do
        if [ -z "${!var}" ]; then
            print_error "Required environment variable $var is not set"
            exit 1
        fi
    done
    
    print_success "Environment validation passed"
}

# Function to deploy infrastructure
deploy_infrastructure() {
    print_status "Deploying infrastructure..."
    
    cd deploy/terraform
    
    # Initialize Terraform
    terraform init
    
    # Create terraform variables file
    cat > terraform.tfvars << EOF
aws_region = "$AWS_REGION"
environment = "$ENVIRONMENT"
git_branch = "$GIT_BRANCH"
ssh_public_key = "$SSH_PUBLIC_KEY"
domain_name = "${DOMAIN_NAME:-}"
route53_zone_id = "${ROUTE53_ZONE_ID:-}"
EOF
    
    # Plan
    terraform plan -var-file=terraform.tfvars -out=tfplan
    
    # Apply
    if [ "$FORCE_RECREATE" = true ]; then
        terraform apply -auto-approve -replace=aws_instance.web tfplan
    else
        terraform apply -auto-approve tfplan
    fi
    
    # Export outputs
    export PUBLIC_IP=$(terraform output -raw public_ip)
    export SSH_COMMAND=$(terraform output -raw ssh_command)
    export APPLICATION_URL=$(terraform output -raw application_url)
    
    print_success "Infrastructure deployed successfully"
    print_status "Public IP: $PUBLIC_IP"
    print_status "SSH Command: $SSH_COMMAND"
    print_status "Application URL: $APPLICATION_URL"
    
    cd ../..
}

# Function to wait for instance to be ready
wait_for_instance() {
    print_status "Waiting for instance to be ready..."
    
    local max_attempts=30
    local attempt=1
    
    while [ $attempt -le $max_attempts ]; do
        if ssh -o ConnectTimeout=5 -o StrictHostKeyChecking=no ubuntu@$PUBLIC_IP "echo 'SSH is ready'" &> /dev/null; then
            print_success "Instance is ready"
            return 0
        fi
        
        print_status "Waiting for SSH connection... ($attempt/$max_attempts)"
        sleep 10
        ((attempt++))
    done
    
    print_error "Instance did not become ready within expected time"
    exit 1
}

# Function to deploy application
deploy_application() {
    print_status "Deploying application..."
    
    # Add host to known_hosts
    ssh-keyscan -H $PUBLIC_IP >> ~/.ssh/known_hosts 2>/dev/null
    
    # Deploy application
    ssh ubuntu@$PUBLIC_IP << EOF
        set -e
        
        echo "Starting application deployment..."
        
        # Navigate to app directory
        cd /home/ubuntu/app
        
        # Pull latest changes
        git fetch origin
        git checkout $GIT_BRANCH
        git pull origin $GIT_BRANCH
        
        # Stop existing services
        docker-compose down || true
        
        # Clean up unused images and containers
        docker system prune -f
        
        # Build and start services
        docker-compose build --no-cache
        docker-compose up -d
        
        # Wait for services to be healthy
        echo "Waiting for services to be healthy..."
        for i in {1..30}; do
            if curl -f http://localhost/health > /dev/null 2>&1; then
                echo "Application is healthy"
                break
            fi
            echo "Waiting for health check... (\$i/30)"
            sleep 10
        done
        
        # Verify all services are running
        docker-compose ps
        
        echo "Application deployment completed successfully!"
EOF
    
    print_success "Application deployed successfully"
}

# Function to run health checks
run_health_checks() {
    print_status "Running health checks..."
    
    # Wait a bit for the application to fully start
    sleep 30
    
    # Check if application is accessible
    if curl -f "$APPLICATION_URL/health" &> /dev/null; then
        print_success "Application health check passed"
    else
        print_error "Application health check failed"
        exit 1
    fi
    
    # Additional checks
    print_status "Running additional checks..."
    
    # Check if all containers are running
    ssh ubuntu@$PUBLIC_IP "docker-compose ps" | grep -q "Up" || {
        print_error "Not all containers are running"
        exit 1
    }
    
    print_success "All health checks passed"
}

# Function to display deployment summary
show_deployment_summary() {
    print_success "🚀 Deployment completed successfully!"
    echo ""
    echo "Deployment Summary:"
    echo "==================="
    echo "Environment: $ENVIRONMENT"
    echo "Git Branch: $GIT_BRANCH"
    echo "Public IP: $PUBLIC_IP"
    echo "Application URL: $APPLICATION_URL"
    echo "SSH Command: $SSH_COMMAND"
    echo ""
    echo "Next steps:"
    echo "- Test the application at: $APPLICATION_URL"
    echo "- Monitor logs: ssh ubuntu@$PUBLIC_IP 'docker-compose logs -f'"
    echo "- Check container status: ssh ubuntu@$PUBLIC_IP 'docker-compose ps'"
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -e|--environment)
            ENVIRONMENT="$2"
            shift 2
            ;;
        -b|--branch)
            GIT_BRANCH="$2"
            shift 2
            ;;
        -f|--force)
            FORCE_RECREATE=true
            shift
            ;;
        -s|--skip-infra)
            SKIP_INFRASTRUCTURE=true
            shift
            ;;
        -a|--skip-app)
            SKIP_APPLICATION=true
            shift
            ;;
        -r|--region)
            AWS_REGION="$2"
            shift 2
            ;;
        -h|--help)
            show_usage
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

# Main execution
main() {
    print_status "Starting deployment process..."
    print_status "Environment: $ENVIRONMENT"
    print_status "Git Branch: $GIT_BRANCH"
    print_status "Force Recreate: $FORCE_RECREATE"
    print_status "Skip Infrastructure: $SKIP_INFRASTRUCTURE"
    print_status "Skip Application: $SKIP_APPLICATION"
    
    # Check prerequisites
    check_prerequisites
    
    # Validate environment
    validate_environment
    
    # Deploy infrastructure
    if [ "$SKIP_INFRASTRUCTURE" = false ]; then
        deploy_infrastructure
    else
        # If skipping infrastructure, we need to get the outputs
        cd deploy/terraform
        export PUBLIC_IP=$(terraform output -raw public_ip)
        export SSH_COMMAND=$(terraform output -raw ssh_command)
        export APPLICATION_URL=$(terraform output -raw application_url)
        cd ../..
        print_status "Skipping infrastructure deployment"
    fi
    
    # Wait for instance to be ready
    wait_for_instance
    
    # Deploy application
    if [ "$SKIP_APPLICATION" = false ]; then
        deploy_application
        run_health_checks
    else
        print_status "Skipping application deployment"
    fi
    
    # Show deployment summary
    show_deployment_summary
}

# Run main function
main "$@"