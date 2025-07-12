#!/bin/bash

# Docker Setup Script for Fantasy Football Draft Board
# This script sets up the entire application using Docker

set -e

echo "🐳 Setting up Fantasy Football Draft Board with Docker"
echo "======================================================"
echo ""

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed. Installing Docker..."
    
    # Update package list
    sudo apt-get update
    
    # Install prerequisites
    sudo apt-get install -y \
        apt-transport-https \
        ca-certificates \
        curl \
        gnupg \
        lsb-release
    
    # Add Docker's official GPG key
    curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
    
    # Add Docker repository
    echo \
        "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu \
        $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
    
    # Install Docker
    sudo apt-get update
    sudo apt-get install -y docker-ce docker-ce-cli containerd.io
    
    # Add current user to docker group
    sudo usermod -aG docker $USER
    
    echo "✅ Docker installed successfully!"
    echo "⚠️  Please log out and log back in for group changes to take effect."
    echo "   Or run: newgrp docker"
    exit 0
fi

# Check if Docker Compose is installed
if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose is not installed. Installing Docker Compose..."
    
    # Install Docker Compose
    sudo curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
    sudo chmod +x /usr/local/bin/docker-compose
    
    echo "✅ Docker Compose installed successfully!"
fi

# Create necessary directories
echo "📁 Creating necessary directories..."
mkdir -p nginx/certbot
mkdir -p nginx/ssl

# Set proper permissions
sudo chown -R $USER:$USER nginx/

echo "✅ Directories created!"

# Check if domain is provided
if [ -z "$1" ]; then
    echo ""
    echo "📋 Usage: $0 <your-domain.com>"
    echo "Example: $0 fantasyfootball.com"
    echo ""
    echo "This will:"
    echo "1. Update nginx configuration with your domain"
    echo "2. Build and start all containers"
    echo "3. Set up SSL certificates (if DNS is configured)"
    echo ""
    echo "If you don't have a domain yet, you can run without one:"
    echo "$0"
    echo ""
    read -p "Do you want to continue without a domain? (y/n): " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
    DOMAIN=""
else
    DOMAIN=$1
    echo "🌐 Domain: $DOMAIN"
    echo "🔧 Updating nginx configuration..."
    
    # Update nginx configuration with domain
    sed -i "s/yourdomain.com/$DOMAIN/g" nginx/nginx.conf
    sed -i "s/dev.yourdomain.com/dev.$DOMAIN/g" nginx/nginx.conf
    
    # Update docker-compose.yml with domain
    sed -i "s/yourdomain.com/$DOMAIN/g" docker-compose.yml
    sed -i "s/dev.yourdomain.com/dev.$DOMAIN/g" docker-compose.yml
    sed -i "s/admin@yourdomain.com/admin@$DOMAIN/g" docker-compose.yml
    
    echo "✅ Nginx configuration updated!"
fi

# Build and start containers
echo ""
echo "🔨 Building and starting containers..."
docker-compose up -d --build

# Wait for containers to be ready
echo ""
echo "⏳ Waiting for containers to be ready..."
sleep 30

# Check container status
echo ""
echo "📊 Container Status:"
docker-compose ps

# Check if containers are healthy
echo ""
echo "🏥 Health Check:"
for service in backend backend-dev frontend frontend-dev postgres postgres-dev nginx; do
    if docker-compose ps $service | grep -q "Up"; then
        echo "✅ $service: Running"
    else
        echo "❌ $service: Not running"
    fi
done

# Set up SSL certificates if domain is provided
if [ ! -z "$DOMAIN" ]; then
    echo ""
    echo "🔒 Setting up SSL certificates..."
    
    # Check if DNS is configured
    if nslookup $DOMAIN > /dev/null 2>&1; then
        echo "✅ DNS appears to be configured. Setting up SSL..."
        
        # Run certbot to generate certificates
        docker-compose --profile ssl run --rm certbot
        
        # Reload nginx to use SSL certificates
        docker-compose restart nginx
        
        echo "✅ SSL certificates generated!"
        echo ""
        echo "🌐 Your secure environments:"
        echo "   Production: https://$DOMAIN"
        echo "   Development: https://dev.$DOMAIN"
    else
        echo "⚠️  DNS not configured yet. SSL setup skipped."
        echo "   Please configure DNS and run: docker-compose --profile ssl run --rm certbot"
    fi
else
    echo ""
    echo "🌐 Your environments (HTTP only):"
    echo "   Production: http://$(curl -s http://169.254.169.254/latest/meta-data/public-ipv4)"
    echo "   Development: http://dev.$(curl -s http://169.254.169.254/latest/meta-data/public-ipv4)"
fi

echo ""
echo "✅ Docker setup complete!"
echo ""
echo "📋 Useful commands:"
echo "   View logs: docker-compose logs -f"
echo "   Stop services: docker-compose down"
echo "   Restart services: docker-compose restart"
echo "   Update and rebuild: docker-compose up -d --build"
echo "   View container status: docker-compose ps"
echo ""
echo "🔧 SSL Certificate Renewal:"
echo "   docker-compose --profile ssl run --rm certbot renew"
echo ""
echo "📊 Monitoring:"
echo "   ./scripts/docker-health-check.sh" 