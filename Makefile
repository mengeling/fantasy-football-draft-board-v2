# Fantasy Football Draft Board - Makefile
# Convenient commands for development and deployment

.PHONY: help dev build test clean deploy setup lint format docker-build docker-push

# Default target
help:
	@echo "Fantasy Football Draft Board - Available Commands"
	@echo "================================================"
	@echo "Development:"
	@echo "  make dev         - Start development environment"
	@echo "  make build       - Build all components"
	@echo "  make test        - Run all tests"
	@echo "  make lint        - Run linting"
	@echo "  make format      - Format code"
	@echo "  make clean       - Clean build artifacts"
	@echo ""
	@echo "Deployment:"
	@echo "  make deploy      - Deploy to production"
	@echo "  make setup       - Run setup assistant"
	@echo "  make infra       - Deploy infrastructure only"
	@echo "  make app         - Deploy application only"
	@echo ""
	@echo "Docker:"
	@echo "  make docker-build - Build Docker images"
	@echo "  make docker-push  - Push Docker images"
	@echo "  make docker-dev   - Start Docker development environment"
	@echo ""
	@echo "Utilities:"
	@echo "  make logs        - View application logs"
	@echo "  make health      - Check application health"
	@echo "  make backup      - Create backup"

# Development commands
dev:
	@echo "Starting development environment..."
	docker-compose -f docker-compose.yml -f docker-compose.override.yml up

build:
	@echo "Building all components..."
	cd backend && cargo build --release
	cd frontend && npm run build

test:
	@echo "Running tests..."
	cd backend && cargo test
	cd frontend && npm test

lint:
	@echo "Running linting..."
	cd backend && cargo clippy -- -D warnings
	cd frontend && npm run lint

format:
	@echo "Formatting code..."
	cd backend && cargo fmt
	cd frontend && npm run format

clean:
	@echo "Cleaning build artifacts..."
	cd backend && cargo clean
	cd frontend && rm -rf build node_modules
	docker system prune -f

# Deployment commands
deploy:
	@echo "Deploying to production..."
	./deploy/scripts/deploy.sh

setup:
	@echo "Running setup assistant..."
	./deploy/scripts/setup-secrets.sh

infra:
	@echo "Deploying infrastructure only..."
	./deploy/scripts/deploy.sh -a

app:
	@echo "Deploying application only..."
	./deploy/scripts/deploy.sh -s

# Docker commands
docker-build:
	@echo "Building Docker images..."
	docker build -t ffball-backend -f Dockerfile.backend .
	docker build -t ffball-frontend -f Dockerfile.frontend .

docker-push:
	@echo "Pushing Docker images..."
	docker push ffball-backend
	docker push ffball-frontend

docker-dev:
	@echo "Starting Docker development environment..."
	docker-compose up

# Utility commands
logs:
	@echo "Viewing application logs..."
	docker-compose logs -f

health:
	@echo "Checking application health..."
	curl -f http://localhost/health || echo "Application is not healthy"

backup:
	@echo "Creating backup..."
	@echo "Backup functionality not implemented yet"

# Nix commands (if Nix is available)
nix-shell:
	@echo "Starting Nix development shell..."
	nix develop

nix-build:
	@echo "Building with Nix..."
	nix build

# Infrastructure commands
tf-init:
	@echo "Initializing Terraform..."
	cd deploy/terraform && terraform init

tf-plan:
	@echo "Planning Terraform changes..."
	cd deploy/terraform && terraform plan

tf-apply:
	@echo "Applying Terraform changes..."
	cd deploy/terraform && terraform apply

tf-destroy:
	@echo "Destroying Terraform resources..."
	cd deploy/terraform && terraform destroy

tf-backend:
	@echo "Setting up Terraform S3 backend..."
	cd deploy/scripts && ./setup-terraform-backend.sh

tf-setup: tf-backend tf-init
	@echo "Terraform setup complete!"

# Database commands
db-setup:
	@echo "Setting up database..."
	docker-compose up -d postgres
	sleep 5
	cd backend && ./src/scripts/setup_db.sh

db-reset:
	@echo "Resetting database..."
	docker-compose down
	docker volume rm fantasy-football-draft-board-v2_postgres_data
	make db-setup

db-backup:
	@echo "Creating database backup..."
	docker-compose exec postgres pg_dump -U ffball -d ffball > backup_$(shell date +%Y%m%d_%H%M%S).sql

# CI/CD commands
ci-test:
	@echo "Running CI tests..."
	make lint
	make test
	make build

ci-build:
	@echo "Building for CI..."
	make docker-build

# Security commands
security-scan:
	@echo "Running security scan..."
	cd backend && cargo audit
	cd frontend && npm audit

# Performance commands
perf-test:
	@echo "Running performance tests..."
	@echo "Performance testing not implemented yet"

# Monitoring commands
monitor:
	@echo "Starting monitoring..."
	docker-compose exec backend htop

# Quick commands for common tasks
quick-deploy: build docker-build deploy
quick-test: lint test
quick-clean: clean

# Help for specific targets
help-deploy:
	@echo "Deployment Help:"
	@echo "==============="
	@echo "make deploy      - Full deployment (infrastructure + application)"
	@echo "make infra       - Deploy infrastructure only"
	@echo "make app         - Deploy application only"
	@echo "make setup       - Run interactive setup"
	@echo ""
	@echo "Environment variables:"
	@echo "  ENVIRONMENT=production|staging"
	@echo "  GIT_BRANCH=main|develop"
	@echo "  FORCE_RECREATE=true|false"

help-dev:
	@echo "Development Help:"
	@echo "================"
	@echo "make dev         - Start development environment with hot reload"
	@echo "make build       - Build all components"
	@echo "make test        - Run all tests"
	@echo "make lint        - Run linting"
	@echo "make format      - Format code"
	@echo "make clean       - Clean build artifacts"