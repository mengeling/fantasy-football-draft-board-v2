# CI/CD Deployment Guide

This guide explains how to set up and use the new CI/CD pipeline with separate development and production environments on the same EC2 instance.

## Architecture Overview

- **Production**: Main domain/IP (port 80 frontend, 8080 backend)
- **Development**: Subdomain (port 80 frontend, 8081 backend)
- **Databases**: Separate `ffball_prod` and `ffball_dev` databases
- **Services**: Separate systemd services for each environment

## Prerequisites

1. GitHub repository with SSH access configured on EC2
2. GitHub Actions secrets configured (see below)
3. EC2 instance with all dependencies installed (Node.js, Rust, PostgreSQL, Nginx)

## GitHub Actions Setup

### 1. Configure Repository Secrets

Go to your GitHub repository → Settings → Secrets and variables → Actions, and add:

- `EC2_HOST`: Your EC2 public IP address
- `EC2_USERNAME`: `ubuntu` (or your EC2 username)
- `EC2_SSH_KEY`: Your private SSH key for EC2 access

### 2. Create Environments

Create two environments in GitHub:

1. **development**: For testing changes
2. **production**: For live deployment

Go to Settings → Environments and create both with appropriate protection rules.

## Server Setup

### 1. Initial Environment Setup

SSH into your EC2 instance and run:

```bash
cd /home/ubuntu/fantasy-football-draft-board-v2
chmod +x scripts/setup-environments.sh
./scripts/setup-environments.sh
```

### 2. Setup Development Database

```bash
chmod +x scripts/setup-dev-db.sh
./scripts/setup-dev-db.sh
```

### 3. Initial Deployment

```bash
# Build and deploy to both environments
cd backend
cargo build --release

cd ../frontend
npm ci
npm run build

# Deploy to production
sudo cp -r build/* /var/www/prod/
sudo chown -R www-data:www-data /var/www/prod/

# Deploy to development
sudo cp -r build/* /var/www/dev/
sudo chown -R www-data:www-data /var/www/dev/

# Restart services
sudo systemctl restart ffball
sudo systemctl restart ffball-dev
sudo systemctl reload nginx
```

## Workflow

### Development Workflow

1. Create a feature branch from `develop`
2. Make your changes
3. Push to your feature branch
4. Create a pull request to `develop`
5. Once merged, GitHub Actions will automatically deploy to the dev environment

### Production Workflow

1. Create a pull request from `develop` to `main`
2. Review and merge the pull request
3. GitHub Actions will automatically deploy to production

## Environment URLs

- **Production**: `http://100.29.78.245` (or your domain)
- **Development**: `http://dev.100.29.78.245` (or your dev subdomain)

## Monitoring

### Service Status

```bash
# Check service status
sudo systemctl status ffball      # Production backend
sudo systemctl status ffball-dev  # Development backend
sudo systemctl status nginx       # Web server

# View logs
sudo journalctl -u ffball -f      # Production logs
sudo journalctl -u ffball-dev -f  # Development logs
tail -f /home/ubuntu/ffball.log   # Production log file
tail -f /home/ubuntu/ffball-dev.log # Development log file
```

### Database Management

```bash
# Connect to production database
PGPASSWORD=ffball psql -U ffball -d ffball_prod

# Connect to development database
PGPASSWORD=ffball psql -U ffball -d ffball_dev

# List databases
sudo -u postgres psql -l
```

## Manual Deployment

If you need to deploy manually:

### Development

```bash
cd /home/ubuntu/fantasy-football-draft-board-v2
git pull origin develop

# Backend
cd backend
cargo build --release
sudo systemctl restart ffball-dev

# Frontend
cd ../frontend
npm ci
npm run build
sudo cp -r build/* /var/www/dev/
sudo chown -R www-data:www-data /var/www/dev/
```

### Production

```bash
cd /home/ubuntu/fantasy-football-draft-board-v2
git pull origin main

# Backend
cd backend
cargo build --release
sudo systemctl restart ffball

# Frontend
cd ../frontend
npm ci
npm run build
sudo cp -r build/* /var/www/prod/
sudo chown -R www-data:www-data /var/www/prod/
```

## Troubleshooting

### Common Issues

1. **Service won't start**: Check logs with `sudo journalctl -u ffball-dev -n 50`
2. **Database connection issues**: Verify DATABASE_URL environment variables
3. **Nginx errors**: Check `/var/log/nginx/error.log`
4. **Permission issues**: Ensure proper ownership of `/var/www/` directories

### Rollback

To rollback to a previous version:

```bash
# Check git log for commit hash
git log --oneline -10

# Reset to previous commit
git reset --hard <commit-hash>
git push --force origin develop  # or main for production

# Rebuild and redeploy
# (GitHub Actions will handle this automatically)
```

## Security Considerations

1. **Environment Variables**: Keep sensitive data in GitHub secrets
2. **Database Access**: Use separate databases for dev/prod
3. **Firewall**: Ensure only necessary ports are open
4. **SSL**: Consider adding SSL certificates for both environments

## Scaling Considerations

If you need to scale beyond a single EC2 instance:

1. Use a load balancer (AWS ALB/ELB)
2. Deploy to multiple EC2 instances
3. Use a managed database service (RDS)
4. Consider containerization with Docker
