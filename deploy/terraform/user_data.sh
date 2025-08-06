#!/bin/bash
set -e

# Install Nix (multi-user mode for better tool access)
export NIX_INSTALLER_NO_MODIFY_PROFILE=1
export HOME=/root
export NIX_CONFIG="experimental-features = nix-command flakes"
export NIXPKGS_ALLOW_UNFREE=1
curl -L https://nixos.org/nix/install | sh -s -- --daemon --yes

mkdir -p /home/ubuntu/app
cd /home/ubuntu/app
git config --global --add safe.directory /home/ubuntu/app
git clone ${git_repo} .
git checkout ${git_branch}
chown -R ubuntu:ubuntu /home/ubuntu/app

. /etc/profile.d/nix.sh
nix profile install --impure .#system-tools

NGINX_STORE_PATH=$(readlink -f /nix/var/nix/profiles/default/bin/nginx | sed 's|/bin/nginx||')
if [ -n "$NGINX_STORE_PATH" ]; then
    cp /home/ubuntu/app/deploy/nginx/ffball.conf "$NGINX_STORE_PATH/conf/nginx.conf"
fi

# Create required Nginx directories
mkdir -p /var/log/nginx
mkdir -p /var/cache/nginx
mkdir -p /var/run

# Set up SSL certificates (if domain is provided)
if [ -n "${domain_name}" ]; then
    echo "Setting up SSL certificates for ${domain_name}..."
    
    # Stop Nginx temporarily to free port 80 for Certbot standalone mode
    echo "Stopping Nginx temporarily for SSL certificate generation..."
    systemctl stop nginx || true
    
    # Obtain SSL certificate using standalone mode (no Nginx needed)
    echo "Obtaining SSL certificate..."
    certbot certonly --standalone -d ${domain_name} --non-interactive --agree-tos --email admin@${domain_name} --dry-run
    echo "SSL certificates obtained successfully!"
    
    nginx -t
    systemctl enable nginx
    systemctl start nginx
    systemctl status nginx --no-pager -l
    
    echo "Nginx with SSL configured successfully!"
    
    systemctl enable certbot.timer
    systemctl start certbot.timer
fi

cp /home/ubuntu/app/deploy/systemd/ffball.service /etc/systemd/system/

mkdir -p /home/ubuntu/logs
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

echo "User data script completed successfully!"