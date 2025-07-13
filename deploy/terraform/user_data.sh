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
    lsb-release

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