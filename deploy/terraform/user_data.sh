#!/bin/bash
set -e

# Update system
apt-get update && apt-get upgrade -y

# Install essential packages
apt-get install -y \
    curl \
    wget \
    git \
    unzip \
    software-properties-common \
    apt-transport-https \
    ca-certificates \
    gnupg \
    lsb-release \
    nginx \
    certbot \
    python3-certbot-nginx

# Install Docker
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null
apt-get update
apt-get install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin

# Add ubuntu user to docker group
usermod -aG docker ubuntu

# Start and enable Docker
systemctl start docker
systemctl enable docker

# Install Docker Compose
curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
chmod +x /usr/local/bin/docker-compose

# Install AWS CLI
curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
unzip awscliv2.zip
./aws/install
rm -rf aws awscliv2.zip

# Set up environment for ubuntu user
export HOME=/home/ubuntu

# Install Just command runner
curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to /usr/local/bin

# Install Nix
curl -L https://nixos.org/nix/install | sh -s -- --daemon

# Create app directory
mkdir -p /home/ubuntu/app
chown ubuntu:ubuntu /home/ubuntu/app

# Clone repository
cd /home/ubuntu/app
git clone ${git_repo} .
git checkout ${git_branch}

# Set ownership
chown -R ubuntu:ubuntu /home/ubuntu/app

# Configure Nginx for SSL (if domain is provided)
if [ -n "${domain_name}" ]; then
    # Create Nginx configuration for SSL
    cat > /etc/nginx/sites-available/ffball << 'EOF'
server {
    listen 80;
    server_name ${domain_name};
    
    # Redirect HTTP to HTTPS
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name ${domain_name};
    
    ssl_certificate /etc/letsencrypt/live/${domain_name}/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/${domain_name}/privkey.pem;
    
    # SSL configuration
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
    
    location / {
        proxy_pass http://localhost:80;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
EOF

    # Enable the site
    ln -sf /etc/nginx/sites-available/ffball /etc/nginx/sites-enabled/
    rm -f /etc/nginx/sites-enabled/default
    
    # Test Nginx configuration
    nginx -t
    
    # Start Nginx
    systemctl enable nginx
    systemctl start nginx
    
    # Create SSL certificate setup script
    cat > /home/ubuntu/setup_ssl.sh << 'EOF'
#!/bin/bash
set -e

DOMAIN_NAME="${domain_name}"

if [ -z "$DOMAIN_NAME" ]; then
    echo "No domain name provided, skipping SSL setup"
    exit 0
fi

echo "Setting up SSL certificate for $DOMAIN_NAME..."

# Wait for DNS to propagate
echo "Waiting for DNS propagation..."
for i in {1..30}; do
    if nslookup $DOMAIN_NAME > /dev/null 2>&1; then
        echo "DNS is ready"
        break
    fi
    echo "Waiting for DNS... ($i/30)"
    sleep 60
done

# Obtain SSL certificate
echo "Obtaining SSL certificate..."
certbot --nginx -d $DOMAIN_NAME --non-interactive --agree-tos --email admin@$DOMAIN_NAME

# Set up automatic renewal
echo "Setting up automatic renewal..."
(crontab -l 2>/dev/null; echo "0 12 * * * /usr/bin/certbot renew --quiet") | crontab -

echo "SSL certificate setup completed!"
EOF

    chmod +x /home/ubuntu/setup_ssl.sh
    chown ubuntu:ubuntu /home/ubuntu/setup_ssl.sh
    
    # Run SSL setup in background (after deployment)
    (sleep 300 && /home/ubuntu/setup_ssl.sh) &
fi

# Create systemd service for the application
cat > /etc/systemd/system/ffball.service << 'EOF'
[Unit]
Description=Fantasy Football Draft Board
After=network.target docker.service
Requires=docker.service

[Service]
Type=oneshot
RemainAfterExit=yes
User=ubuntu
Group=ubuntu
WorkingDirectory=/home/ubuntu/app
ExecStart=/usr/local/bin/docker-compose up -d
ExecStop=/usr/local/bin/docker-compose down
TimeoutStartSec=0

[Install]
WantedBy=multi-user.target
EOF

# Create deployment script
cat > /home/ubuntu/deploy.sh << 'EOF'
#!/bin/bash
set -e

cd /home/ubuntu/app

# Pull latest changes
git pull origin ${git_branch}

# Rebuild and restart containers
docker-compose down
docker-compose build --no-cache
docker-compose up -d

# Wait for services to be healthy
sleep 30

# Verify deployment
curl -f http://localhost/health || exit 1

echo "Deployment completed successfully!"
EOF

chmod +x /home/ubuntu/deploy.sh
chown ubuntu:ubuntu /home/ubuntu/deploy.sh

# Create log directory
mkdir -p /home/ubuntu/logs
chown ubuntu:ubuntu /home/ubuntu/logs

# Set up log rotation
cat > /etc/logrotate.d/ffball << 'EOF'
/home/ubuntu/logs/*.log {
    daily
    rotate 7
    compress
    delaycompress
    missingok
    notifempty
    copytruncate
    su ubuntu ubuntu
}
EOF

# Enable and start the service
systemctl daemon-reload
systemctl enable ffball
systemctl start ffball

# Create a simple health check script
cat > /home/ubuntu/health_check.sh << 'EOF'
#!/bin/bash
curl -f http://localhost/health > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "Application is healthy"
    exit 0
else
    echo "Application is unhealthy"
    exit 1
fi
EOF

chmod +x /home/ubuntu/health_check.sh
chown ubuntu:ubuntu /home/ubuntu/health_check.sh

# Set up CloudWatch agent (optional)
wget https://s3.amazonaws.com/amazoncloudwatch-agent/ubuntu/amd64/latest/amazon-cloudwatch-agent.deb
dpkg -i amazon-cloudwatch-agent.deb
rm amazon-cloudwatch-agent.deb

echo "User data script completed successfully!"