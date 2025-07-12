# Fantasy Football Draft Board - Task Runner
# Run `just --list` to see all available commands

# Default recipe to run when no arguments are provided
default:
    @just --list

# Show help and available commands
help:
    @echo "Fantasy Football Draft Board - Available Commands"
    @echo "================================================"
    @echo ""
    @just --list

# =============================================================================
# DEPLOYMENT COMMANDS
# =============================================================================

# Deploy the entire application (Docker mode)
deploy domain="":
    #!/usr/bin/env bash
    set -e
    
    echo "🚀 Deploying Fantasy Football Draft Board..."
    
    if [ -n "{{domain}}" ]; then
        echo "🌐 Domain: {{domain}}"
        # Update configuration files with domain
        sed -i "s/yourdomain.com/{{domain}}/g" nginx/nginx.conf
        sed -i "s/dev.yourdomain.com/dev.{{domain}}/g" nginx/nginx.conf
        sed -i "s/yourdomain.com/{{domain}}/g" docker-compose.yml
        sed -i "s/dev.yourdomain.com/dev.{{domain}}/g" docker-compose.yml
        sed -i "s/admin@yourdomain.com/admin@{{domain}}/g" docker-compose.yml
        echo "✅ Configuration updated for domain: {{domain}}"
    fi
    
    # Create necessary directories
    mkdir -p nginx/certbot nginx/ssl
    sudo chown -R $USER:$USER nginx/
    
    # Deploy with Docker Compose
    docker-compose up -d --build
    
    # Wait for services to be ready
    echo "⏳ Waiting for services to be ready..."
    sleep 30
    
    # Set up SSL if domain is provided
    if [ -n "{{domain}}" ]; then
        echo "🔒 Setting up SSL certificates..."
        if nslookup {{domain}} > /dev/null 2>&1; then
            docker-compose --profile ssl run --rm certbot
            docker-compose restart nginx
            echo "✅ SSL certificates generated"
        else
            echo "⚠️  DNS not configured yet. SSL setup skipped."
            echo "   Please configure DNS and run: just ssl"
        fi
    fi
    
    echo "✅ Deployment complete!"
    just status

# Deploy manually (systemd services)
deploy-manual:
    #!/usr/bin/env bash
    set -e
    
    echo "🔧 Deploying manually with systemd services..."
    
    # Check if scripts exist
    if [ ! -f "scripts/initial-setup.sh" ]; then
        echo "❌ Manual setup scripts not found"
        exit 1
    fi
    
    # Run manual setup
    chmod +x scripts/*.sh
    ./scripts/initial-setup.sh
    
    echo "✅ Manual deployment complete!"

# Deploy only development environment
deploy-dev:
    docker-compose up -d --build backend-dev frontend-dev postgres-dev
    echo "✅ Development environment deployed!"

# Deploy only production environment
deploy-prod:
    docker-compose up -d --build backend frontend postgres
    echo "✅ Production environment deployed!"

# =============================================================================
# SSL & DOMAIN COMMANDS
# =============================================================================

# Set up SSL certificates
ssl domain:
    #!/usr/bin/env bash
    set -e
    
    if [ -z "{{domain}}" ]; then
        echo "❌ Domain is required. Usage: just ssl yourdomain.com"
        exit 1
    fi
    
    echo "🔒 Setting up SSL certificates for {{domain}}..."
    
    # Check if DNS is configured
    if ! nslookup {{domain}} > /dev/null 2>&1; then
        echo "❌ DNS not configured for {{domain}}"
        echo "   Please configure DNS records first:"
        echo "   A record: {{domain}} → $(curl -s http://169.254.169.254/latest/meta-data/public-ipv4)"
        echo "   A record: dev.{{domain}} → $(curl -s http://169.254.169.254/latest/meta-data/public-ipv4)"
        exit 1
    fi
    
    # Generate certificates
    docker-compose --profile ssl run --rm certbot
    
    # Restart nginx to use certificates
    docker-compose restart nginx
    
    echo "✅ SSL certificates generated for {{domain}}"

# Renew SSL certificates
ssl-renew:
    docker-compose --profile ssl run --rm certbot renew
    docker-compose restart nginx
    echo "✅ SSL certificates renewed"

# Set up domain configuration
setup-domain domain:
    #!/usr/bin/env bash
    set -e
    
    if [ -z "{{domain}}" ]; then
        echo "❌ Domain is required. Usage: just setup-domain yourdomain.com"
        exit 1
    fi
    
    echo "🌐 Setting up domain configuration for {{domain}}..."
    
    # Get EC2 IP
    EC2_IP=$(curl -s http://169.254.169.254/latest/meta-data/public-ipv4)
    
    echo "📋 DNS Configuration Required:"
    echo "   A record: {{domain}} → $EC2_IP"
    echo "   A record: dev.{{domain}} → $EC2_IP"
    echo ""
    echo "⏳ DNS changes can take 5-60 minutes to propagate."
    echo ""
    echo "After DNS is configured, run: just ssl {{domain}}"

# =============================================================================
# MONITORING & HEALTH CHECKS
# =============================================================================

# Check status of all services
status:
    @echo "📊 Service Status"
    @echo "================"
    @docker-compose ps
    @echo ""
    @echo "🏥 Health Check"
    @echo "=============="
    @just health

# Run comprehensive health check
health:
    #!/usr/bin/env bash
    set -e
    
    echo "🔍 Health Check Report"
    echo "======================"
    echo ""
    
    # Check if Docker is running
    if ! docker info > /dev/null 2>&1; then
        echo "❌ Docker is not running"
        exit 1
    fi
    
    # Check container status
    echo "📊 Container Status:"
    echo "-------------------"
    docker-compose ps
    echo ""
    
    # Check container health
    echo "🏥 Container Health:"
    echo "-------------------"
    for service in backend backend-dev frontend frontend-dev postgres postgres-dev nginx; do
        if docker-compose ps $service | grep -q "Up"; then
            HEALTH_STATUS=$(docker inspect --format='{{.State.Health.Status}}' ffball-$service 2>/dev/null || echo "no-health-check")
            if [ "$HEALTH_STATUS" = "healthy" ]; then
                echo "✅ $service: Healthy"
            elif [ "$HEALTH_STATUS" = "no-health-check" ]; then
                echo "✅ $service: Running (no health check)"
            else
                echo "⚠️  $service: Running but unhealthy ($HEALTH_STATUS)"
            fi
        else
            echo "❌ $service: Not running"
        fi
    done
    echo ""
    
    # Check web endpoints
    echo "🌐 Web Endpoints:"
    echo "----------------"
    EC2_IP=$(curl -s http://169.254.169.254/latest/meta-data/public-ipv4)
    
    echo "Production Frontend:"
    if curl -s -o /dev/null -w "%{http_code}" "http://$EC2_IP" | grep -q "200"; then
        echo "✅ Responding"
    else
        echo "❌ Not responding"
    fi
    
    echo "Development Frontend:"
    if curl -s -o /dev/null -w "%{http_code}" "http://dev.$EC2_IP" | grep -q "200"; then
        echo "✅ Responding"
    else
        echo "❌ Not responding"
    fi
    
    echo "Production API:"
    if curl -s -o /dev/null -w "%{http_code}" "http://$EC2_IP/api/players" | grep -q "200\|404"; then
        echo "✅ Responding"
    else
        echo "❌ Not responding"
    fi
    
    echo "Development API:"
    if curl -s -o /dev/null -w "%{http_code}" "http://dev.$EC2_IP/api/players" | grep -q "200\|404"; then
        echo "✅ Responding"
    else
        echo "❌ Not responding"
    fi
    echo ""
    
    # Check SSL if configured
    if [ -d "nginx/certbot" ] && [ "$(ls -A nginx/certbot 2>/dev/null)" ]; then
        echo "🔒 SSL Certificate Status:"
        echo "--------------------------"
        DOMAIN=$(find nginx/certbot -maxdepth 1 -type d -name "*.com" -o -name "*.net" -o -name "*.org" | head -1 | xargs basename 2>/dev/null || echo "")
        if [ ! -z "$DOMAIN" ]; then
            echo "Production Certificate:"
            if openssl s_client -connect $DOMAIN:443 -servername $DOMAIN < /dev/null 2>/dev/null | openssl x509 -noout -dates | head -1 | grep -q "notAfter"; then
                echo "✅ Valid"
            else
                echo "❌ Invalid/Expired"
            fi
        fi
    fi
    echo ""
    echo "✅ Health check complete!"

# Watch logs in real-time
logs service="":
    if [ -n "{{service}}" ]; then
        docker-compose logs -f {{service}}
    else
        docker-compose logs -f
    fi

# Show recent logs
logs-recent service="":
    if [ -n "{{service}}" ]; then
        docker-compose logs --tail=50 {{service}}
    else
        docker-compose logs --tail=50
    fi

# =============================================================================
# DATABASE COMMANDS
# =============================================================================

# Connect to production database
db-prod:
    docker-compose exec postgres psql -U ffball -d ffball_prod

# Connect to development database
db-dev:
    docker-compose exec postgres-dev psql -U ffball -d ffball_dev

# Backup production database
backup-prod:
    docker-compose exec postgres pg_dump -U ffball ffball_prod > backup_prod_$(date +%Y%m%d_%H%M%S).sql
    echo "✅ Production database backed up"

# Backup development database
backup-dev:
    docker-compose exec postgres-dev pg_dump -U ffball ffball_dev > backup_dev_$(date +%Y%m%d_%H%M%S).sql
    echo "✅ Development database backed up"

# Backup all databases
backup:
    just backup-prod
    just backup-dev

# Restore production database
restore-prod file:
    docker-compose exec -T postgres psql -U ffball -d ffball_prod < {{file}}
    echo "✅ Production database restored from {{file}}"

# Restore development database
restore-dev file:
    docker-compose exec -T postgres-dev psql -U ffball -d ffball_dev < {{file}}
    echo "✅ Development database restored from {{file}}"

# =============================================================================
# MAINTENANCE COMMANDS
# =============================================================================

# Update and rebuild all services
update:
    git pull origin main
    docker-compose up -d --build
    echo "✅ Services updated and rebuilt"

# Update specific service
update-service service:
    docker-compose up -d --build {{service}}
    echo "✅ {{service}} updated and rebuilt"

# Restart all services
restart:
    docker-compose restart
    echo "✅ All services restarted"

# Restart specific service
restart-service service:
    docker-compose restart {{service}}
    echo "✅ {{service}} restarted"

# Stop all services
stop:
    docker-compose down
    echo "✅ All services stopped"

# Start all services
start:
    docker-compose up -d
    echo "✅ All services started"

# Remove all containers and volumes (⚠️ destructive)
clean:
    @echo "⚠️  This will remove all containers and data!"
    @read -p "Are you sure? (y/N): " -n 1 -r; \
    echo; \
    if [[ $$REPLY =~ ^[Yy]$$ ]]; then \
        docker-compose down -v; \
        echo "✅ All containers and volumes removed"; \
    else \
        echo "❌ Cleanup cancelled"; \
    fi

# Clean up Docker system
cleanup:
    docker system prune -a --volumes
    echo "✅ Docker system cleaned up"

# =============================================================================
# DEVELOPMENT COMMANDS
# =============================================================================

# Build backend
build-backend:
    cd backend && cargo build --release
    echo "✅ Backend built"

# Build frontend
build-frontend:
    cd frontend && npm ci && npm run build
    echo "✅ Frontend built"

# Run backend tests
test-backend:
    cd backend && cargo test
    echo "✅ Backend tests passed"

# Run frontend tests
test-frontend:
    cd frontend && npm run lint
    echo "✅ Frontend tests passed"

# Run all tests
test:
    just test-backend
    just test-frontend

# Development mode (with hot reload)
dev:
    @echo "🚀 Starting development mode..."
    @echo "Backend: http://localhost:8080"
    @echo "Frontend: http://localhost:3000"
    @echo "Development: http://localhost:3001"
    @echo ""
    @echo "Press Ctrl+C to stop"
    docker-compose up

# =============================================================================
# UTILITY COMMANDS
# =============================================================================

# Show system information
info:
    @echo "🖥️  System Information"
    @echo "===================="
    @echo "EC2 IP: $(curl -s http://169.254.169.254/latest/meta-data/public-ipv4)"
    @echo "Docker version: $(docker --version)"
    @echo "Docker Compose version: $(docker-compose --version)"
    @echo "Disk usage:"
    @df -h / | tail -1
    @echo ""
    @echo "🐳 Docker usage:"
    @docker system df

# Show environment URLs
urls:
    #!/usr/bin/env bash
    EC2_IP=$(curl -s http://169.254.169.254/latest/meta-data/public-ipv4)
    
    echo "🌐 Environment URLs"
    echo "=================="
    echo "Production: http://$EC2_IP"
    echo "Development: http://dev.$EC2_IP"
    echo ""
    
    # Check if SSL is configured
    if [ -d "nginx/certbot" ] && [ "$(ls -A nginx/certbot 2>/dev/null)" ]; then
        DOMAIN=$(find nginx/certbot -maxdepth 1 -type d -name "*.com" -o -name "*.net" -o -name "*.org" | head -1 | xargs basename 2>/dev/null || echo "")
        if [ ! -z "$DOMAIN" ]; then
            echo "🌐 Secure URLs (if DNS configured):"
            echo "Production: https://$DOMAIN"
            echo "Development: https://dev.$DOMAIN"
        fi
    fi

# Show configuration
config:
    @echo "⚙️  Configuration"
    @echo "================"
    @echo "Docker Compose:"
    @docker-compose config
    @echo ""
    @echo "Nginx Configuration:"
    @docker-compose exec nginx nginx -t 2>/dev/null || echo "Nginx not running"

# =============================================================================
# CI/CD COMMANDS
# =============================================================================

# Deploy to development (for CI/CD)
ci-deploy-dev:
    git pull origin develop
    docker-compose up -d --build backend-dev frontend-dev
    sleep 30
    docker-compose ps backend-dev frontend-dev

# Deploy to production (for CI/CD)
ci-deploy-prod:
    git pull origin main
    docker-compose up -d --build backend frontend
    sleep 30
    docker-compose ps backend frontend

# =============================================================================
# NIX DEVELOPMENT ENVIRONMENT
# =============================================================================

# Enter Nix development shell
nix-shell: ## Enter Nix development environment
    @echo "🚀 Entering Nix development shell..."
    nix-shell

# Enter Nix development shell with flakes
nix-flake: ## Enter Nix development shell using flakes
    @echo "🚀 Entering Nix development shell (flakes)..."
    nix develop

# Enter production Nix shell
nix-prod: ## Enter Nix production environment
    @echo "🏭 Entering Nix production shell..."
    nix develop .#production

# Build backend with Nix
nix-build-backend: ## Build backend using Nix
    @echo "🔨 Building backend with Nix..."
    nix build .#backend

# Build frontend with Nix
nix-build-frontend: ## Build frontend using Nix
    @echo "🔨 Building frontend with Nix..."
    nix build .#frontend

# Build all packages with Nix
nix-build-all: ## Build all packages using Nix
    @echo "🔨 Building all packages with Nix..."
    nix build

# Build Docker images with Nix
nix-docker-backend: ## Build backend Docker image with Nix
    @echo "🐳 Building backend Docker image with Nix..."
    nix build .#backend-docker

nix-docker-frontend: ## Build frontend Docker image with Nix
    @echo "🐳 Building frontend Docker image with Nix..."
    nix build .#frontend-docker

# Update Nix dependencies
nix-update: ## Update Nix flake inputs
    @echo "🔄 Updating Nix flake inputs..."
    nix flake update

# Clean Nix store
nix-clean: ## Clean Nix store and garbage collect
    @echo "🧹 Cleaning Nix store..."
    nix store gc
    nix store optimise

# =============================================================================
# DEVELOPMENT
# =============================================================================

# Start backend development server
dev-backend: ## Start backend development server
    @echo "🚀 Starting backend development server..."
    cd backend && cargo run

# Start frontend development server
dev-frontend: ## Start frontend development server
    @echo "🚀 Starting frontend development server..."
    cd frontend && pnpm run dev

# Start both development servers
dev: ## Start both backend and frontend development servers
    @echo "🚀 Starting development servers..."
    @just dev-backend & just dev-frontend

# =============================================================================
# BUILDING
# =============================================================================

# Build backend
build-backend: ## Build backend for production
    @echo "🔨 Building backend..."
    cd backend && cargo build --release

# Build frontend
build-frontend: ## Build frontend for production
    @echo "🔨 Building frontend..."
    cd frontend && pnpm run build

# Build both backend and frontend
build: ## Build both backend and frontend
    @echo "🔨 Building project..."
    @just build-backend
    @just build-frontend

# =============================================================================
# TESTING
# =============================================================================

# Test backend
test-backend: ## Run backend tests
    @echo "🧪 Running backend tests..."
    cd backend && cargo test

# Test frontend
test-frontend: ## Run frontend tests
    @echo "🧪 Running frontend tests..."
    cd frontend && pnpm run test

# Test everything
test: ## Run all tests
    @echo "🧪 Running all tests..."
    @just test-backend
    @just test-frontend

# =============================================================================
# DOCKER
# =============================================================================

# Build Docker images
docker-build: ## Build all Docker images
    @echo "🐳 Building Docker images..."
    docker-compose build

# Start Docker services
docker-up: ## Start all Docker services
    @echo "🐳 Starting Docker services..."
    docker-compose up -d

# Stop Docker services
docker-down: ## Stop all Docker services
    @echo "🐳 Stopping Docker services..."
    docker-compose down

# View Docker logs
docker-logs: ## View Docker logs
    @echo "📝 Showing Docker logs..."
    docker-compose logs -f

# =============================================================================
# DATABASE
# =============================================================================

# Setup development database
db-setup: ## Setup development database
    @echo "🗄️ Setting up development database..."
    @just db-create-dev
    @just db-migrate-dev

# Create development database
db-create-dev: ## Create development database
    @echo "🗄️ Creating development database..."
    createdb -U ffball ffball_dev || true

# Reset development database
db-reset: ## Reset development database
    @echo "🗄️ Resetting development database..."
    dropdb -U ffball ffball_dev || true
    @just db-create-dev
    @just db-migrate-dev

# Run database migrations (development)
db-migrate-dev: ## Run database migrations for development
    @echo "🗄️ Running development migrations..."
    cd backend && DATABASE_URL="postgresql://ffball:ffball@localhost:5432/ffball_dev" cargo run --bin migrate

# =============================================================================
# DEPLOYMENT
# =============================================================================

# Deploy to production
deploy: ## Deploy to production
    @echo "🚀 Deploying to production..."
    @just build
    @just docker-build
    @just docker-up

# Deploy with Nix
deploy-nix: ## Deploy using Nix builds
    @echo "🚀 Deploying with Nix..."
    @just nix-build-all
    @just docker-up

# =============================================================================
# MONITORING
# =============================================================================

# Health check
health-check: ## Check system health
    @echo "🔍 Running health check..."
    ./scripts/health-check.sh

# View logs
logs: ## View application logs
    @echo "📝 Showing application logs..."
    sudo journalctl -u ffball -f

# =============================================================================
# MAINTENANCE
# =============================================================================

# Backup database
backup: ## Backup production database
    @echo "💾 Backing up database..."
    pg_dump -U ffball ffball_prod > backup_$(date +%Y%m%d_%H%M%S).sql

# Restore database
restore file: ## Restore database from backup file
    @echo "💾 Restoring database from {{file}}..."
    psql -U ffball ffball_prod < {{file}}

# Update dependencies
update-deps: ## Update all dependencies
    @echo "🔄 Updating dependencies..."
    cd backend && cargo update
    cd frontend && pnpm update

# =============================================================================
# UTILITIES
# =============================================================================

# Format code
format: ## Format all code
    @echo "🎨 Formatting code..."
    cd backend && cargo fmt
    cd frontend && pnpm run format

# Lint code
lint: ## Lint all code
    @echo "🔍 Linting code..."
    cd backend && cargo clippy
    cd frontend && pnpm run lint

# Clean build artifacts
clean: ## Clean build artifacts
    @echo "🧹 Cleaning build artifacts..."
    cd backend && cargo clean
    cd frontend && rm -rf node_modules dist

# Show project status
status: ## Show project status
    @echo "📊 Project Status"
    @echo "================="
    @echo "Backend:"
    @cd backend && cargo check --quiet && echo "✅ Backend compiles" || echo "❌ Backend has errors"
    @echo "Frontend:"
    @cd frontend && pnpm run check --silent && echo "✅ Frontend checks pass" || echo "❌ Frontend has errors"
    @echo "Docker:"
    @docker-compose ps --quiet && echo "✅ Docker services running" || echo "❌ Docker services stopped"

# Show help
help: ## Show this help message
    @echo "Fantasy Football Draft Board - Available Commands"
    @echo "================================================"
    @just --list

# =============================================================================
# CI/CD INTEGRATION
# =============================================================================

# CI build
ci-build: ## Build for CI/CD
    @echo "🔨 CI Build..."
    @just test
    @just build
    @just docker-build

# CI test
ci-test: ## Test for CI/CD
    @echo "🧪 CI Test..."
    @just test-backend
    @just test-frontend

# CI deploy
ci-deploy: ## Deploy for CI/CD
    @echo "🚀 CI Deploy..."
    @just deploy-nix 