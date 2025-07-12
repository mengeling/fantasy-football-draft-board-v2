#!/bin/bash

# Setup script for dev and prod environments on the same EC2 instance
# This script should be run on the EC2 instance

set -e

echo "Setting up dev and prod environments..."

# Create directories for static files
sudo mkdir -p /var/www/dev
sudo mkdir -p /var/www/prod
sudo chown -R www-data:www-data /var/www/dev
sudo chown -R www-data:www-data /var/www/prod

# Create dev database
sudo -u postgres psql -c "CREATE DATABASE ffball_dev;" || echo "Database ffball_dev already exists"
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE ffball_dev TO ffball;" || echo "Privileges already granted"

# Create prod database (if not exists)
sudo -u postgres psql -c "CREATE DATABASE ffball_prod;" || echo "Database ffball_prod already exists"
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE ffball_prod TO ffball;" || echo "Privileges already granted"

# Create dev backend service
sudo tee /etc/systemd/system/ffball-dev.service > /dev/null <<EOF
[Unit]
Description=Fantasy Football Backend (Dev)
After=network.target postgresql.service

[Service]
Type=simple
User=ubuntu
Group=ubuntu
Environment="RUST_LOG=info"
Environment="RUST_BACKTRACE=1"
Environment="DATABASE_URL=postgresql://ffball:ffball@localhost/ffball_dev"
Environment="PORT=8081"
WorkingDirectory=/home/ubuntu/fantasy-football-draft-board-v2/backend
ExecStart=/home/ubuntu/fantasy-football-draft-board-v2/backend/target/release/backend
Restart=always
RestartSec=5
StandardOutput=append:/home/ubuntu/ffball-dev.log
StandardError=append:/home/ubuntu/ffball-dev.log

[Install]
WantedBy=multi-user.target
EOF

# Update prod backend service to use prod database
sudo tee /etc/systemd/system/ffball.service > /dev/null <<EOF
[Unit]
Description=Fantasy Football Backend (Prod)
After=network.target postgresql.service

[Service]
Type=simple
User=ubuntu
Group=ubuntu
Environment="RUST_LOG=info"
Environment="RUST_BACKTRACE=1"
Environment="DATABASE_URL=postgresql://ffball:ffball@localhost/ffball_prod"
Environment="PORT=8080"
WorkingDirectory=/home/ubuntu/fantasy-football-draft-board-v2/backend
ExecStart=/home/ubuntu/fantasy-football-draft-board-v2/backend/target/release/backend
Restart=always
RestartSec=5
StandardOutput=append:/home/ubuntu/ffball.log
StandardError=append:/home/ubuntu/ffball.log

[Install]
WantedBy=multi-user.target
EOF

# Create log files
sudo touch /home/ubuntu/ffball-dev.log
sudo touch /home/ubuntu/ffball.log
sudo chown ubuntu:ubuntu /home/ubuntu/ffball-dev.log
sudo chown ubuntu:ubuntu /home/ubuntu/ffball.log

# Create nginx configuration for both environments
sudo tee /etc/nginx/sites-available/ffball-multi-env > /dev/null <<EOF
# Production environment (main domain)
server {
    listen 80;
    server_name 100.29.78.245;  # Replace with your domain or IP

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
        proxy_cache_bypass \$http_upgrade;
    }
}

# Development environment (subdomain)
server {
    listen 80;
    server_name dev.100.29.78.245;  # Replace with your dev subdomain

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
        proxy_cache_bypass \$http_upgrade;
    }
}
EOF

# Enable the new configuration
sudo ln -sf /etc/nginx/sites-available/ffball-multi-env /etc/nginx/sites-enabled/
sudo rm -f /etc/nginx/sites-enabled/default

# Test nginx configuration
sudo nginx -t

# Reload systemd and restart services
sudo systemctl daemon-reload
sudo systemctl enable ffball-dev
sudo systemctl enable ffball
sudo systemctl restart ffball-dev
sudo systemctl restart ffball
sudo systemctl reload nginx

echo "Environment setup complete!"
echo "Production: http://100.29.78.245"
echo "Development: http://dev.100.29.78.245" 