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
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg --batch --yes
echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null
apt-get update
apt-get install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin

# Add ubuntu user to docker group
usermod -aG docker ubuntu

# Start and enable Docker
systemctl start docker
systemctl enable docker

# Docker Compose is included in docker-compose-plugin package

# Install AWS CLI
curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
unzip awscliv2.zip
./aws/install
rm -rf aws awscliv2.zip

# Debug environment
echo "Current HOME: $HOME"
echo "Current USER: $USER"
echo "Current PWD: $PWD"

# Set up environment for ubuntu user
export HOME=/home/ubuntu

# Install Just command runner
curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to /usr/local/bin

# Install Nix (single-user mode for root) - SKIPPING FOR NOW
# Create nixbld group if it doesn't exist
# groupadd -f nixbld
# sh <(curl -L https://nixos.org/nix/install) --no-daemon --yes
echo "Skipping Nix installation for now"

# Create app directory
mkdir -p /home/ubuntu/app

# Clone repository
cd /home/ubuntu/app
git config --global --add safe.directory /home/ubuntu/app
git clone ${git_repo} .
git checkout ${git_branch}

# Set ownership
chown -R ubuntu:ubuntu /home/ubuntu/app

# Set up SSL certificates first (if domain is provided)
if [ -n "${domain_name}" ]; then
    echo "Setting up SSL certificates for ${domain_name}..."
    
    # Stop Nginx temporarily to free port 80 for Certbot standalone mode
    echo "Stopping Nginx temporarily for SSL certificate generation..."
    systemctl stop nginx || true
    
    # Obtain SSL certificate using standalone mode (no Nginx needed)
    echo "Obtaining SSL certificate..."
    certbot certonly --standalone -d ${domain_name} --non-interactive --agree-tos --email admin@${domain_name}
    
    echo "SSL certificates obtained successfully!"
    
    # Configure Nginx with SSL
    echo "Configuring Nginx with SSL..."
    
    # Copy Nginx config from repository and substitute domain name
    sed "s/\${domain_name}/${domain_name}/g" /home/ubuntu/app/deploy/nginx/ffball.conf > /etc/nginx/sites-available/ffball

    # Enable the site
    ln -sf /etc/nginx/sites-available/ffball /etc/nginx/sites-enabled/
    rm -f /etc/nginx/sites-enabled/default
    
    # Test Nginx configuration
    nginx -t
    
    # Start Nginx
    systemctl enable nginx
    systemctl start nginx
    
    echo "Nginx with SSL configured successfully!"
    
    # Enable Certbot's built-in renewal timer
    systemctl enable certbot.timer
    systemctl start certbot.timer
fi

# Copy systemd service from repository
cp /home/ubuntu/app/deploy/systemd/ffball.service /etc/systemd/system/

# Create log directory
mkdir -p /home/ubuntu/logs

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

# Enable the service for future deployments (but don't start it yet)
systemctl daemon-reload
systemctl enable ffball



# Set up CloudWatch agent (optional)
# wget https://s3.amazonaws.com/amazoncloudwatch-agent/ubuntu/amd64/latest/amazon-cloudwatch-agent.deb
# dpkg -i amazon-cloudwatch-agent.deb
# rm amazon-cloudwatch-agent.deb

echo "User data script completed successfully!"