#!/bin/bash

# Master Deployment Script for Fantasy Football Draft Board
# This script handles both manual and Docker deployments

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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
    echo "Fantasy Football Draft Board - Deployment Script"
    echo "================================================"
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  -m, --mode MODE        Deployment mode: docker|manual (default: docker)"
    echo "  -d, --domain DOMAIN    Domain name for SSL setup"
    echo "  -e, --environment ENV  Environment: dev|prod|both (default: both)"
    echo "  -s, --ssl              Set up SSL certificates"
    echo "  -h, --health           Run health check after deployment"
    echo "  --help                 Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0                                    # Deploy with Docker (both environments)"
    echo "  $0 -m docker -d example.com          # Deploy with Docker + SSL"
    echo "  $0 -m manual -e prod                 # Manual deployment (production only)"
    echo "  $0 -h                                 # Run health check"
    echo ""
}

# Function to check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    # Check if we're on Ubuntu/Debian
    if ! command -v apt-get &> /dev/null; then
        print_error "This script is designed for Ubuntu/Debian systems"
        exit 1
    fi
    
    # Check if we're in the right directory
    if [ ! -f "docker-compose.yml" ] || [ ! -f "backend/Cargo.toml" ]; then
        print_error "Please run this script from the project root directory"
        exit 1
    fi
    
    print_success "Prerequisites check passed"
}

# Function to install Docker
install_docker() {
    print_status "Installing Docker..."
    
    if command -v docker &> /dev/null; then
        print_success "Docker is already installed"
        return
    fi
    
    # Install Docker
    sudo apt-get update
    sudo apt-get install -y \
        apt-transport-https \
        ca-certificates \
        curl \
        gnupg \
        lsb-release
    
    curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
    
    echo \
        "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu \
        $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
    
    sudo apt-get update
    sudo apt-get install -y docker-ce docker-ce-cli containerd.io
    
    # Add user to docker group
    sudo usermod -aG docker $USER
    
    print_success "Docker installed successfully"
    print_warning "Please log out and log back in for group changes to take effect"
}

# Function to install Docker Compose
install_docker_compose() {
    print_status "Installing Docker Compose..."
    
    if command -v docker-compose &> /dev/null; then
        print_success "Docker Compose is already installed"
        return
    fi
    
    sudo curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
    sudo chmod +x /usr/local/bin/docker-compose
    
    print_success "Docker Compose installed successfully"
}

# Function to deploy with Docker
deploy_docker() {
    print_status "Deploying with Docker..."
    
    # Create necessary directories
    mkdir -p nginx/certbot
    mkdir -p nginx/ssl
    sudo chown -R $USER:$USER nginx/
    
    # Update configuration if domain is provided
    if [ ! -z "$DOMAIN" ]; then
        print_status "Updating configuration for domain: $DOMAIN"
        sed -i "s/yourdomain.com/$DOMAIN/g" nginx/nginx.conf
        sed -i "s/dev.yourdomain.com/dev.$DOMAIN/g" nginx/nginx.conf
        sed -i "s/yourdomain.com/$DOMAIN/g" docker-compose.yml
        sed -i "s/dev.yourdomain.com/dev.$DOMAIN/g" docker-compose.yml
        sed -i "s/admin@yourdomain.com/admin@$DOMAIN/g" docker-compose.yml
    fi
    
    # Build and start containers
    print_status "Building and starting containers..."
    docker-compose up -d --build
    
    # Wait for services to be ready
    print_status "Waiting for services to be ready..."
    sleep 30
    
    # Set up SSL if requested
    if [ "$SETUP_SSL" = true ] && [ ! -z "$DOMAIN" ]; then
        print_status "Setting up SSL certificates..."
        if nslookup $DOMAIN > /dev/null 2>&1; then
            docker-compose --profile ssl run --rm certbot
            docker-compose restart nginx
            print_success "SSL certificates generated"
        else
            print_warning "DNS not configured yet. SSL setup skipped."
            print_warning "Please configure DNS and run: docker-compose --profile ssl run --rm certbot"
        fi
    fi
    
    print_success "Docker deployment complete"
}

# Function to deploy manually
deploy_manual() {
    print_status "Deploying manually..."
    
    # This would call the existing manual setup scripts
    if [ -f "scripts/initial-setup.sh" ]; then
        ./scripts/initial-setup.sh
    else
        print_error "Manual setup scripts not found"
        exit 1
    fi
    
    print_success "Manual deployment complete"
}

# Function to run health check
run_health_check() {
    print_status "Running health check..."
    
    if [ "$MODE" = "docker" ]; then
        if [ -f "scripts/docker-health-check.sh" ]; then
            ./scripts/docker-health-check.sh
        else
            print_warning "Docker health check script not found"
        fi
    else
        if [ -f "scripts/health-check.sh" ]; then
            ./scripts/health-check.sh
        else
            print_warning "Health check script not found"
        fi
    fi
}

# Function to show deployment info
show_deployment_info() {
    echo ""
    echo "🎉 Deployment Complete!"
    echo "======================"
    echo ""
    
    if [ "$MODE" = "docker" ]; then
        echo "🐳 Docker Deployment:"
        echo "   Status: docker-compose ps"
        echo "   Logs: docker-compose logs -f"
        echo "   Update: docker-compose up -d --build"
        echo ""
        
        if [ ! -z "$DOMAIN" ]; then
            echo "🌐 Your environments:"
            echo "   Production: https://$DOMAIN"
            echo "   Development: https://dev.$DOMAIN"
        else
            EC2_IP=$(curl -s http://169.254.169.254/latest/meta-data/public-ipv4)
            echo "🌐 Your environments:"
            echo "   Production: http://$EC2_IP"
            echo "   Development: http://dev.$EC2_IP"
        fi
    else
        echo "🔧 Manual Deployment:"
        echo "   Status: sudo systemctl status ffball ffball-dev"
        echo "   Logs: sudo journalctl -u ffball -f"
        echo ""
        
        if [ ! -z "$DOMAIN" ]; then
            echo "🌐 Your environments:"
            echo "   Production: https://$DOMAIN"
            echo "   Development: https://dev.$DOMAIN"
        else
            EC2_IP=$(curl -s http://169.254.169.254/latest/meta-data/public-ipv4)
            echo "🌐 Your environments:"
            echo "   Production: http://$EC2_IP"
            echo "   Development: http://dev.$EC2_IP"
        fi
    fi
    
    echo ""
    echo "📊 Monitoring:"
    echo "   Health check: $0 -h"
    echo "   Documentation: README.md"
}

# Parse command line arguments
MODE="docker"
DOMAIN=""
ENVIRONMENT="both"
SETUP_SSL=false
RUN_HEALTH_CHECK=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -m|--mode)
            MODE="$2"
            shift 2
            ;;
        -d|--domain)
            DOMAIN="$2"
            shift 2
            ;;
        -e|--environment)
            ENVIRONMENT="$2"
            shift 2
            ;;
        -s|--ssl)
            SETUP_SSL=true
            shift
            ;;
        -h|--health)
            RUN_HEALTH_CHECK=true
            shift
            ;;
        --help)
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
    echo "🚀 Fantasy Football Draft Board - Deployment"
    echo "============================================="
    echo ""
    
    # Show usage if health check only
    if [ "$RUN_HEALTH_CHECK" = true ]; then
        run_health_check
        exit 0
    fi
    
    # Check prerequisites
    check_prerequisites
    
    # Install Docker if needed
    if [ "$MODE" = "docker" ]; then
        install_docker
        install_docker_compose
    fi
    
    # Deploy based on mode
    if [ "$MODE" = "docker" ]; then
        deploy_docker
    elif [ "$MODE" = "manual" ]; then
        deploy_manual
    else
        print_error "Invalid mode: $MODE. Use 'docker' or 'manual'"
        exit 1
    fi
    
    # Run health check if requested
    if [ "$RUN_HEALTH_CHECK" = true ]; then
        run_health_check
    fi
    
    # Show deployment info
    show_deployment_info
}

# Run main function
main "$@" 