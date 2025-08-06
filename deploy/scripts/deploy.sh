#!/bin/bash
set -e

# Fantasy Football Draft Board Deployment Script
# This script provides one-button deployment functionality

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

ENVIRONMENT="production"
GIT_BRANCH="main"

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

show_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  -e, --environment    Environment to deploy to (default: production)"
    echo "  -b, --branch         Git branch to deploy (default: main)"
    echo "  -h, --help           Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0                           # Deploy application"
    echo "  $0 -e staging -b develop     # Deploy with specific environment and branch"
    echo ""
    echo "Note: This script should be run from the app directory on EC2."
}

check_prerequisites() {
    print_status "Checking prerequisites..."
    
    local tools=("nix" "git" "curl" "docker" "docker-compose")
    for tool in "${tools[@]}"; do
        if ! command -v "$tool" &> /dev/null; then
            print_error "$tool is not installed or not in PATH"
            exit 1
        fi
    done
    
    print_success "Prerequisites check passed"
}

deploy_application() {    
    set -e
    echo "Starting application deployment..."
    
    if ! docker info &> /dev/null; then
        print_status "Docker daemon not running, starting it..."
        sudo /nix/var/nix/profiles/default/bin/dockerd --host=unix:///var/run/docker.sock &
        sleep 5
    fi
    
    print_status "Building Docker images with Nix..."
    nix build .#backendImage && docker load < result
    nix build .#frontendImage && docker load < result
    
    print_status "Deploying with Docker Compose..."
    docker-compose down || true 
    docker system prune -f || true
    docker-compose up -d
    docker-compose ps    
    print_success "Application deployed successfully!"
}

show_deployment_summary() {
    print_success "🚀 Deployment completed successfully!"
    echo ""
    echo "Deployment Summary:"
    echo "==================="
    echo "Environment: $ENVIRONMENT"
    echo "Git Branch: $GIT_BRANCH"
    echo ""
    echo "Next steps:"
    echo "- Monitor logs: docker-compose logs -f"
    echo "- Check container status: docker-compose ps"
}

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

main() {
    print_status "Starting application deployment..."
    print_status "Environment: $ENVIRONMENT"
    print_status "Git Branch: $GIT_BRANCH"
    check_prerequisites
    deploy_application
    show_deployment_summary
}

main "$@"
