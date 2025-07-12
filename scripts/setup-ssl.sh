#!/bin/bash

# SSL Setup Script for Fantasy Football Draft Board
# This script sets up SSL certificates using Let's Encrypt

set -e

echo "🔒 Setting up SSL certificates..."

# Check if domain is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <your-domain.com>"
    echo "Example: $0 fantasyfootball.example.com"
    exit 1
fi

DOMAIN=$1
DEV_SUBDOMAIN="dev.$DOMAIN"

echo "Domain: $DOMAIN"
echo "Development subdomain: $DEV_SUBDOMAIN"

# Install certbot if not already installed
if ! command -v certbot &> /dev/null; then
    echo "Installing certbot..."
    sudo apt update
    sudo apt install -y certbot python3-certbot-nginx
fi

# Create nginx configuration for SSL
echo "Creating nginx configuration for SSL..."

sudo tee /etc/nginx/sites-available/ffball-ssl > /dev/null <<EOF
# Production environment (main domain)
server {
    listen 80;
    server_name $DOMAIN;
    return 301 https://\$server_name\$request_uri;
}

server {
    listen 443 ssl http2;
    server_name $DOMAIN;

    # SSL configuration
    ssl_certificate /etc/letsencrypt/live/$DOMAIN/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/$DOMAIN/privkey.pem;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-RSA-AES256-GCM-SHA512:DHE-RSA-AES256-GCM-SHA512:ECDHE-RSA-AES256-GCM-SHA384:DHE-RSA-AES256-GCM-SHA384;
    ssl_prefer_server_ciphers off;
    ssl_session_cache shared:SSL:10m;
    ssl_session_timeout 10m;

    # Security headers
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
    add_header X-Frame-Options DENY always;
    add_header X-Content-Type-Options nosniff always;
    add_header X-XSS-Protection "1; mode=block" always;

    # Frontend - serve static files
    root /var/www/prod;
    index index.html;

    location / {
        try_files \$uri \$uri/ /index.html;
    }

    # Backend API
    location /api/ {
        proxy_pass http://127.0.0.1:8080/;
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
        proxy_cache_bypass \$http_upgrade;
    }
}

# Development environment (subdomain)
server {
    listen 80;
    server_name $DEV_SUBDOMAIN;
    return 301 https://\$server_name\$request_uri;
}

server {
    listen 443 ssl http2;
    server_name $DEV_SUBDOMAIN;

    # SSL configuration
    ssl_certificate /etc/letsencrypt/live/$DEV_SUBDOMAIN/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/$DEV_SUBDOMAIN/privkey.pem;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-RSA-AES256-GCM-SHA512:DHE-RSA-AES256-GCM-SHA512:ECDHE-RSA-AES256-GCM-SHA384:DHE-RSA-AES256-GCM-SHA384;
    ssl_prefer_server_ciphers off;
    ssl_session_cache shared:SSL:10m;
    ssl_session_timeout 10m;

    # Security headers
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
    add_header X-Frame-Options DENY always;
    add_header X-Content-Type-Options nosniff always;
    add_header X-XSS-Protection "1; mode=block" always;

    # Frontend - serve static files
    root /var/www/dev;
    index index.html;

    location / {
        try_files \$uri \$uri/ /index.html;
    }

    # Backend API
    location /api/ {
        proxy_pass http://127.0.0.1:8081/;
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
        proxy_cache_bypass \$http_upgrade;
    }
}
EOF

# Temporarily enable HTTP-only configuration for certificate generation
echo "Creating temporary HTTP configuration for certificate generation..."

sudo tee /etc/nginx/sites-available/ffball-temp > /dev/null <<EOF
# Temporary configuration for certificate generation
server {
    listen 80;
    server_name $DOMAIN $DEV_SUBDOMAIN;

    location /.well-known/acme-challenge/ {
        root /var/www/certbot;
    }

    location / {
        return 301 https://\$server_name\$request_uri;
    }
}
EOF

# Create certbot webroot directory
sudo mkdir -p /var/www/certbot

# Enable temporary configuration
sudo ln -sf /etc/nginx/sites-available/ffball-temp /etc/nginx/sites-enabled/
sudo rm -f /etc/nginx/sites-enabled/ffball-multi-env
sudo nginx -t
sudo systemctl reload nginx

# Generate certificates
echo "Generating SSL certificates..."

# Production certificate
echo "Generating certificate for $DOMAIN..."
sudo certbot certonly --webroot \
    --webroot-path=/var/www/certbot \
    --email admin@$DOMAIN \
    --agree-tos \
    --no-eff-email \
    -d $DOMAIN

# Development certificate
echo "Generating certificate for $DEV_SUBDOMAIN..."
sudo certbot certonly --webroot \
    --webroot-path=/var/www/certbot \
    --email admin@$DOMAIN \
    --agree-tos \
    --no-eff-email \
    -d $DEV_SUBDOMAIN

# Enable SSL configuration
echo "Enabling SSL configuration..."
sudo ln -sf /etc/nginx/sites-available/ffball-ssl /etc/nginx/sites-enabled/
sudo rm -f /etc/nginx/sites-enabled/ffball-temp

# Test nginx configuration
sudo nginx -t

# Reload nginx
sudo systemctl reload nginx

# Set up automatic renewal
echo "Setting up automatic certificate renewal..."
sudo crontab -l 2>/dev/null | { cat; echo "0 12 * * * /usr/bin/certbot renew --quiet"; } | sudo crontab -

echo ""
echo "✅ SSL setup complete!"
echo ""
echo "🌐 Your secure environments:"
echo "   Production: https://$DOMAIN"
echo "   Development: https://$DEV_SUBDOMAIN"
echo ""
echo "📋 Next steps:"
echo "1. Update your DNS records to point to your EC2 IP:"
echo "   A record: $DOMAIN → $(curl -s http://169.254.169.254/latest/meta-data/public-ipv4)"
echo "   A record: $DEV_SUBDOMAIN → $(curl -s http://169.254.169.254/latest/meta-data/public-ipv4)"
echo ""
echo "2. Update GitHub Actions secrets with new URLs"
echo ""
echo "3. Test your environments:"
echo "   curl -I https://$DOMAIN"
echo "   curl -I https://$DEV_SUBDOMAIN" 