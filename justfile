# Fantasy Football Draft Board - Justfile
# Convenient commands for development and deployment

# Default recipe - show help
default:
    @echo "Fantasy Football Draft Board - Nix-based Development & Deployment"
    @echo "================================================================"
    @echo ""
    @just --list
    @echo ""
    @echo "Quick Start:"
    @echo "  just nix-deploy    - Deploy with Nix-built images"
    @echo "  just nix-shell     - Enter Nix development environment"
    @echo "  just help-deploy   - Show deployment help"

# Development commands
dev: # Start development environment
    @echo "Starting development environment..."
    docker-compose -f docker-compose.yml -f docker-compose.override.yml up

build: # Build all components
    @echo "Building all components..."
    cd backend && cargo build --release
    cd frontend && npm run build

test: # Run all tests
    @echo "Running tests..."
    cd backend && cargo test
    cd frontend && npm test

lint: # Run linting
    @echo "Running linting..."
    cd backend && cargo clippy -- -D warnings
    cd frontend && npm run lint

format: # Format code
    @echo "Formatting code..."
    cd backend && cargo fmt
    cd frontend && npm run format

clean: # Clean build artifacts
    @echo "Cleaning build artifacts..."
    cd backend && cargo clean
    cd frontend && rm -rf build node_modules
    docker system prune -f

# Deployment commands
deploy: # Deploy to production
    @echo "Deploying to production..."
    ./deploy/scripts/deploy.sh

deploy-github: # Deploy via GitHub Actions workflow dispatch
    @echo "To deploy via GitHub Actions:"
    @echo "1. Go to: https://github.com/$(git remote get-url origin | sed 's/.*github.com[:/]\([^/]*\/[^/]*\).*/\1/')/actions/workflows/deploy.yml"
    @echo "2. Click 'Run workflow'"
    @echo "3. Select your branch and environment"
    @echo "4. Click 'Run workflow'"

setup: # Run setup assistant
    @echo "Running setup assistant..."
    ./deploy/scripts/setup-secrets.sh

infra: # Deploy infrastructure only
    @echo "Deploying infrastructure only..."
    ./deploy/scripts/deploy.sh -a

app: # Deploy application only
    @echo "Deploying application only..."
    ./deploy/scripts/deploy.sh -s

# Nix-based deployment commands
nix-build-images: # Build Docker images with Nix
    @echo "Building Docker images with Nix..."
    nix build --extra-experimental-features 'nix-command flakes' .#backendImage .#frontendImage

nix-load-images: # Load Nix-built images into Docker
    @echo "Loading Nix-built images into Docker..."
    docker load < result-1
    docker load < result-2

nix-deploy: nix-build-images nix-load-images # Deploy with Nix-built images
    @echo "Deploying with Nix-built Docker images..."
    docker-compose down || true
    docker system prune -f
    docker-compose up -d
    docker-compose ps

# Utility commands
logs: # View application logs
    @echo "Viewing application logs..."
    docker-compose logs -f

health: # Check application health
    @echo "Checking application health..."
    curl -f http://localhost/health || echo "Application is not healthy"

backup: # Create backup
    @echo "Creating backup..."
    @echo "Backup functionality not implemented yet"

# Nix commands (if Nix is available)
nix-shell: # Start Nix development shell
    @echo "Starting Nix development shell..."
    nix develop

nix-build: # Build with Nix
    @echo "Building with Nix..."
    nix build --extra-experimental-features 'nix-command flakes'



# Infrastructure commands
tf-init: # Initialize Terraform
    @echo "Initializing Terraform..."
    cd deploy/terraform && terraform init

tf-init-no-backend: # Initialize Terraform without backend
    @echo "Initializing Terraform without backend..."
    cd deploy/terraform && terraform init -backend=false

tf-plan: # Plan Terraform changes
    @echo "Planning Terraform changes..."
    cd deploy/terraform && terraform plan

tf-apply: # Apply Terraform changes
    @echo "Applying Terraform changes..."
    cd deploy/terraform && terraform apply

tf-destroy: # Destroy Terraform resources
    @echo "Destroying Terraform resources..."
    cd deploy/terraform && terraform destroy

tf-format: # Format Terraform code
    @echo "Formatting Terraform code..."
    cd deploy/terraform && terraform fmt -check

tf-validate: # Validate Terraform configuration
    @echo "Validating Terraform configuration..."
    cd deploy/terraform && terraform validate

tf-backend: # Set up Terraform S3 backend
    @echo "Setting up Terraform S3 backend..."
    cd deploy/scripts && ./setup-terraform-backend.sh

tf-setup: tf-backend tf-init # Complete Terraform setup
    @echo "Terraform setup complete!"

# Database commands
db-setup: # Set up database
    @echo "Setting up database..."
    docker-compose up -d postgres
    sleep 5
    cd backend && ./src/scripts/setup_db.sh

db-reset: # Reset database
    @echo "Resetting database..."
    docker-compose down
    docker volume rm fantasy-football-draft-board-v2_postgres_data
    just db-setup

db-backup: # Create database backup
    @echo "Creating database backup..."
    docker-compose exec postgres pg_dump -U ffball -d ffball > backup_$(date +%Y%m%d_%H%M%S).sql

# CI/CD commands
ci-test: lint test build # Run CI tests
    @echo "Running CI tests..."

ci-build: nix-build-images # Build for CI with Nix
    @echo "Building for CI with Nix..."

# Security commands
security-scan: # Run security scan
    @echo "Running security scan..."
    cd backend && cargo audit
    cd frontend && npm audit

# Performance commands
perf-test: # Run performance tests
    @echo "Running performance tests..."
    @echo "Performance testing not implemented yet"

# Monitoring commands
monitor: # Start monitoring
    @echo "Starting monitoring..."
    docker-compose exec backend htop

# Quick commands for common tasks
quick-deploy: build nix-deploy # Quick deployment with Nix
quick-test: lint test # Quick test
quick-clean: clean # Quick clean

# Help commands
help-deploy: # Show deployment help
    @echo "Deployment Help:"
    @echo "==============="
    @echo "just deploy      - Full deployment (infrastructure + application)"
    @echo "just nix-deploy  - Deploy with Nix-built Docker images"
    @echo "just infra       - Deploy infrastructure only"
    @echo "just app         - Deploy application only"
    @echo "just setup       - Run interactive setup"
    @echo ""
    @echo "Nix Commands:"
    @echo "  just nix-build-images  - Build Docker images with Nix"
    @echo "  just nix-load-images   - Load images into Docker"
    @echo "  just nix-deploy        - Complete Nix deployment"
    @echo ""
    @echo "Environment variables:"
    @echo "  ENVIRONMENT=production|staging"
    @echo "  GIT_BRANCH=main|develop"
    @echo "  FORCE_RECREATE=true|false"

help-dev: # Show development help
    @echo "Development Help:"
    @echo "================"
    @echo "just dev         - Start development environment with hot reload"
    @echo "just build       - Build all components"
    @echo "just test        - Run all tests"
    @echo "just lint        - Run linting"
    @echo "just format      - Format code"
    @echo "just clean       - Clean build artifacts"

# Advanced commands with parameters
deploy-env env: # Deploy to specific environment
    @echo "Deploying to {{env}}..."
    ENVIRONMENT={{env}} ./deploy/scripts/deploy.sh

build-backend: # Build only backend
    @echo "Building backend..."
    cd backend && cargo build --release

build-frontend: # Build only frontend
    @echo "Building frontend..."
    cd frontend && npm run build

test-backend: # Test only backend
    @echo "Testing backend..."
    cd backend && cargo test

test-frontend: # Test only frontend
    @echo "Testing frontend..."
    cd frontend && npm test

# Database commands with parameters
db-backup-named name: # Create named database backup
    @echo "Creating database backup: {{name}}..."
    docker-compose exec postgres pg_dump -U ffball -d ffball > backup_{{name}}.sql

# Docker commands with parameters
docker-build-tag tag: # Build Docker images with specific tag
    @echo "Building Docker images with tag: {{tag}}..."
    docker build -t ffball-backend:{{tag}} -f Dockerfile.backend .
    docker build -t ffball-frontend:{{tag}} -f Dockerfile.frontend .

# Utility functions
timestamp:
    #!/usr/bin/env bash
    date +%Y%m%d_%H%M%S 