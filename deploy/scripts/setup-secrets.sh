#!/bin/bash
set -e

# Setup script for GitHub secrets and environment variables
# This script helps configure the deployment pipeline

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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

print_header() {
    echo -e "${GREEN}================================${NC}"
    echo -e "${GREEN}$1${NC}"
    echo -e "${GREEN}================================${NC}"
}

# Function to generate SSH key pair
generate_ssh_key() {
    print_header "SSH Key Generation"
    
    local key_path="$HOME/.ssh/ffball_deploy"
    
    if [ -f "$key_path" ]; then
        print_warning "SSH key already exists at $key_path"
        read -p "Do you want to overwrite it? (y/N): " -r
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            print_status "Using existing SSH key"
            return 0
        fi
    fi
    
    print_status "Generating new SSH key pair..."
    ssh-keygen -t rsa -b 4096 -C "ffball-deploy" -f "$key_path" -N ""
    
    print_success "SSH key pair generated successfully"
    print_status "Public key location: ${key_path}.pub"
    print_status "Private key location: $key_path"
}

# Function to display GitHub secrets
show_github_secrets() {
    print_header "GitHub Secrets Configuration"
    
    echo "Please add the following secrets to your GitHub repository:"
    echo "Go to: Settings → Secrets and variables → Actions → Repository secrets"
    echo ""
    
    # AWS credentials
    echo "1. AWS_ACCESS_KEY_ID"
    echo "   Value: Your AWS access key ID"
    echo ""
    
    echo "2. AWS_SECRET_ACCESS_KEY"
    echo "   Value: Your AWS secret access key"
    echo ""
    
    # SSH keys
    if [ -f "$HOME/.ssh/ffball_deploy.pub" ]; then
        echo "3. SSH_PUBLIC_KEY"
        echo "   Value:"
        cat "$HOME/.ssh/ffball_deploy.pub"
        echo ""
    else
        echo "3. SSH_PUBLIC_KEY"
        echo "   Value: [Run this script first to generate SSH keys]"
        echo ""
    fi
    
    if [ -f "$HOME/.ssh/ffball_deploy" ]; then
        echo "4. SSH_PRIVATE_KEY"
        echo "   Value:"
        cat "$HOME/.ssh/ffball_deploy"
        echo ""
    else
        echo "4. SSH_PRIVATE_KEY"
        echo "   Value: [Run this script first to generate SSH keys]"
        echo ""
    fi
}

# Function to display GitHub variables
show_github_variables() {
    print_header "GitHub Variables Configuration"
    
    echo "Please add the following variables to your GitHub repository:"
    echo "Go to: Settings → Secrets and variables → Actions → Repository variables"
    echo ""
    
    echo "1. AWS_REGION"
    echo "   Value: us-east-1 (or your preferred region)"
    echo ""
    
    echo "2. DOMAIN_NAME (optional)"
    echo "   Value: your-domain.com"
    echo ""
    
    echo "3. ROUTE53_ZONE_ID (optional, required if DOMAIN_NAME is set)"
    echo "   Value: Z1234567890ABC"
    echo ""
}

# Function to display environment setup
show_environment_setup() {
    print_header "Environment Setup"
    
    echo "For manual deployments, set the following environment variables:"
    echo ""
    
    if [ -f "$HOME/.ssh/ffball_deploy.pub" ]; then
        echo "export SSH_PUBLIC_KEY=\"$(cat $HOME/.ssh/ffball_deploy.pub)\""
    else
        echo "export SSH_PUBLIC_KEY=\"your-public-key-here\""
    fi
    
    echo "export AWS_REGION=\"us-east-1\""
    echo "export DOMAIN_NAME=\"your-domain.com\"  # Optional"
    echo "export ROUTE53_ZONE_ID=\"Z1234567890ABC\"  # Optional"
    echo ""
    
    echo "You can add these to your ~/.bashrc or ~/.zshrc file for persistence."
}

# Function to validate AWS credentials
validate_aws_credentials() {
    print_header "AWS Credentials Validation"
    
    if ! command -v aws &> /dev/null; then
        print_error "AWS CLI is not installed. Please install it first."
        return 1
    fi
    
    print_status "Checking AWS credentials..."
    
    if aws sts get-caller-identity &> /dev/null; then
        local account_id=$(aws sts get-caller-identity --query Account --output text)
        local user_arn=$(aws sts get-caller-identity --query Arn --output text)
        
        print_success "AWS credentials are valid"
        print_status "Account ID: $account_id"
        print_status "User ARN: $user_arn"
        
        # Check required permissions
        print_status "Checking required permissions..."
        
        local required_permissions=(
            "ec2:*"
            "vpc:*"
            "iam:*"
            "route53:*"
        )
        
        print_warning "Please ensure your AWS user has the following permissions:"
        for perm in "${required_permissions[@]}"; do
            echo "  - $perm"
        done
        
    else
        print_error "AWS credentials are not configured or invalid"
        print_status "Please run 'aws configure' to set up your credentials"
        return 1
    fi
}

# Function to create deployment configuration
create_deployment_config() {
    print_header "Creating Deployment Configuration"
    
    local config_file="deploy/terraform/terraform.tfvars"
    
    if [ -f "$config_file" ]; then
        print_warning "Configuration file already exists at $config_file"
        read -p "Do you want to overwrite it? (y/N): " -r
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            print_status "Using existing configuration"
            return 0
        fi
    fi
    
    print_status "Creating Terraform configuration..."
    
    # Get user input
    read -p "AWS Region [us-east-1]: " aws_region
    aws_region=${aws_region:-us-east-1}
    
    read -p "Environment [production]: " environment
    environment=${environment:-production}
    
    read -p "Instance Type [t3.medium]: " instance_type
    instance_type=${instance_type:-t3.medium}
    
    read -p "Domain Name (optional): " domain_name
    
    if [ -n "$domain_name" ]; then
        read -p "Route53 Zone ID: " route53_zone_id
    fi
    
    # Create configuration file
    cat > "$config_file" << EOF
# AWS Configuration
aws_region = "$aws_region"

# Project Configuration
project_name = "ffball"
environment  = "$environment"

# Network Configuration
vpc_cidr           = "10.0.0.0/16"
public_subnet_cidr = "10.0.1.0/24"

# EC2 Configuration
instance_type      = "$instance_type"
root_volume_size   = 20

# SSH Configuration
ssh_public_key = "$(cat $HOME/.ssh/ffball_deploy.pub 2>/dev/null || echo "your-public-key-here")"

# Application Configuration
git_repo   = "https://github.com/mengeling/fantasy-football-draft-board-v2.git"
git_branch = "main"

EOF
    
    if [ -n "$domain_name" ]; then
        echo "# Domain Configuration" >> "$config_file"
        echo "domain_name      = \"$domain_name\"" >> "$config_file"
        echo "route53_zone_id  = \"$route53_zone_id\"" >> "$config_file"
    fi
    
    print_success "Configuration file created at $config_file"
}

# Function to show deployment instructions
show_deployment_instructions() {
    print_header "Deployment Instructions"
    
    echo "Now you can deploy using one of these methods:"
    echo ""
    
    echo "1. GitHub Actions (Recommended):"
    echo "   - Go to Actions tab in your GitHub repository"
    echo "   - Find the 'Deploy' workflow"
    echo "   - Click 'Run workflow'"
    echo "   - Choose environment and branch"
    echo "   - Click 'Run workflow' button"
    echo ""
    
    echo "2. Manual deployment:"
    echo "   - Run: chmod +x deploy/scripts/deploy.sh"
    echo "   - Run: ./deploy/scripts/deploy.sh"
    echo "   - Or with options: ./deploy/scripts/deploy.sh -e production -b main"
    echo ""
    
    echo "3. Local development:"
    echo "   - Install Nix: curl -L https://nixos.org/nix/install | sh"
    echo "   - Run: nix develop"
    echo "   - Run: docker-compose up"
    echo ""
}

# Main menu
show_menu() {
    print_header "Fantasy Football Draft Board - Setup Assistant"
    
    echo "Choose an option:"
    echo "1. Generate SSH keys"
    echo "2. Show GitHub secrets configuration"
    echo "3. Show GitHub variables configuration"
    echo "4. Show environment setup"
    echo "5. Validate AWS credentials"
    echo "6. Create deployment configuration"
    echo "7. Show deployment instructions"
    echo "8. Run all setup steps"
    echo "9. Exit"
    echo ""
    
    read -p "Enter your choice [1-9]: " choice
    
    case $choice in
        1)
            generate_ssh_key
            ;;
        2)
            show_github_secrets
            ;;
        3)
            show_github_variables
            ;;
        4)
            show_environment_setup
            ;;
        5)
            validate_aws_credentials
            ;;
        6)
            create_deployment_config
            ;;
        7)
            show_deployment_instructions
            ;;
        8)
            generate_ssh_key
            validate_aws_credentials
            create_deployment_config
            show_github_secrets
            show_github_variables
            show_environment_setup
            show_deployment_instructions
            ;;
        9)
            print_success "Setup complete!"
            exit 0
            ;;
        *)
            print_error "Invalid choice. Please try again."
            show_menu
            ;;
    esac
    
    echo ""
    read -p "Press Enter to continue..."
    show_menu
}

# Run the main menu
show_menu