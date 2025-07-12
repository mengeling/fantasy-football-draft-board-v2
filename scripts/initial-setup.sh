#!/bin/bash

# Initial setup script for CI/CD environment
# Run this on your EC2 instance to set up the new environment

set -e

echo "Setting up CI/CD environment..."

# Check if we're in the right directory
if [ ! -f "backend/Cargo.toml" ] || [ ! -f "frontend/package.json" ]; then
    echo "Error: Please run this script from the project root directory"
    exit 1
fi

# Create scripts directory if it doesn't exist
mkdir -p scripts

# Make scripts executable
chmod +x scripts/setup-environments.sh
chmod +x scripts/setup-dev-db.sh

# Run environment setup
echo "Setting up environments..."
./scripts/setup-environments.sh

# Setup development database
echo "Setting up development database..."
./scripts/setup-dev-db.sh

# Build and deploy initial versions
echo "Building and deploying initial versions..."

# Backend
cd backend
cargo build --release
cd ..

# Frontend
cd frontend
npm ci
npm run build
cd ..

# Deploy to both environments
echo "Deploying to production..."
sudo cp -r frontend/build/* /var/www/prod/
sudo chown -R www-data:www-data /var/www/prod/

echo "Deploying to development..."
sudo cp -r frontend/build/* /var/www/dev/
sudo chown -R www-data:www-data /var/www/dev/

# Restart services
echo "Restarting services..."
sudo systemctl restart ffball
sudo systemctl restart ffball-dev
sudo systemctl reload nginx

echo ""
echo "✅ CI/CD environment setup complete!"
echo ""
echo "📋 Next steps:"
echo "1. Configure GitHub Actions secrets:"
echo "   - EC2_HOST: $(curl -s http://169.254.169.254/latest/meta-data/public-ipv4)"
echo "   - EC2_USERNAME: ubuntu"
echo "   - EC2_SSH_KEY: Your private SSH key"
echo ""
echo "2. Create GitHub environments:"
echo "   - development"
echo "   - production"
echo ""
echo "3. Create a 'develop' branch:"
echo "   git checkout -b develop"
echo "   git push -u origin develop"
echo ""
echo "🌐 Your environments:"
echo "   Production: http://$(curl -s http://169.254.169.254/latest/meta-data/public-ipv4)"
echo "   Development: http://dev.$(curl -s http://169.254.169.254/latest/meta-data/public-ipv4)"
echo ""
echo "📊 Check service status:"
echo "   sudo systemctl status ffball"
echo "   sudo systemctl status ffball-dev"
echo "   sudo systemctl status nginx" 