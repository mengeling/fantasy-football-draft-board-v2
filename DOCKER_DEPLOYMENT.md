# Docker Deployment Guide

This guide explains how to deploy your Fantasy Football Draft Board using Docker, which simplifies the deployment process and eliminates most manual dependency installation.

## 🐳 Why Docker?

### Benefits:

- **Consistent Environment**: Same setup across development and production
- **No Manual Dependencies**: All dependencies are containerized
- **Easy Scaling**: Simple to add more instances
- **Isolation**: Each service runs in its own container
- **Version Control**: Exact versions of all dependencies
- **Rollback**: Easy to revert to previous versions

### Architecture:

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Nginx Proxy   │    │   Frontend      │    │   Backend       │
│   (Port 80/443) │◄──►│   (Svelte)      │    │   (Rust)        │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 │
                    ┌─────────────────┐
                    │   PostgreSQL    │
                    │   Database      │
                    └─────────────────┘
```

## 🚀 Quick Start

### Prerequisites

- Ubuntu EC2 instance
- Docker and Docker Compose (automatically installed by script)

### One-Command Setup

```bash
# Clone the repository
git clone git@github.com:mengeling/fantasy-football-draft-board-v2.git
cd fantasy-football-draft-board-v2

# Run the Docker setup script
chmod +x scripts/docker-setup.sh
./scripts/docker-setup.sh yourdomain.com
```

That's it! The script will:

1. Install Docker and Docker Compose
2. Build all containers
3. Start all services
4. Set up SSL certificates (if DNS is configured)

## 📋 Manual Setup Steps

If you prefer to set up manually:

### 1. Install Docker

```bash
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

# Add user to docker group
sudo usermod -aG docker $USER
newgrp docker
```

### 2. Install Docker Compose

```bash
# Install Docker Compose
sudo curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
```

### 3. Configure Domain (Optional)

If you have a domain:

```bash
# Update nginx configuration
sed -i "s/yourdomain.com/YOUR_DOMAIN/g" nginx/nginx.conf
sed -i "s/dev.yourdomain.com/dev.YOUR_DOMAIN/g" nginx/nginx.conf

# Update docker-compose.yml
sed -i "s/yourdomain.com/YOUR_DOMAIN/g" docker-compose.yml
sed -i "s/dev.yourdomain.com/dev.YOUR_DOMAIN/g" docker-compose.yml
sed -i "s/admin@yourdomain.com/admin@YOUR_DOMAIN/g" docker-compose.yml
```

### 4. Start Services

```bash
# Build and start all services
docker-compose up -d --build

# Check status
docker-compose ps
```

### 5. Set Up SSL (If Domain Configured)

```bash
# Generate SSL certificates
docker-compose --profile ssl run --rm certbot

# Restart nginx to use certificates
docker-compose restart nginx
```

## 🔧 Service Management

### View Services

```bash
# List all containers
docker-compose ps

# View logs
docker-compose logs -f

# View logs for specific service
docker-compose logs -f backend
```

### Update Services

```bash
# Pull latest code
git pull origin main

# Rebuild and restart services
docker-compose up -d --build

# Or update specific services
docker-compose up -d --build backend frontend
```

### Stop Services

```bash
# Stop all services
docker-compose down

# Stop and remove volumes (⚠️ deletes data)
docker-compose down -v
```

### Restart Services

```bash
# Restart all services
docker-compose restart

# Restart specific service
docker-compose restart backend
```

## 📊 Monitoring

### Health Check

```bash
# Run comprehensive health check
./scripts/docker-health-check.sh
```

### Resource Usage

```bash
# View container resource usage
docker stats

# View disk usage
docker system df
```

### Logs

```bash
# View all logs
docker-compose logs

# View logs for specific service
docker-compose logs backend

# Follow logs in real-time
docker-compose logs -f nginx
```

## 🔒 SSL Certificate Management

### Automatic Renewal

```bash
# Set up automatic renewal (add to crontab)
echo "0 12 * * * cd /home/ubuntu/fantasy-football-draft-board-v2 && docker-compose --profile ssl run --rm certbot renew" | crontab -
```

### Manual Renewal

```bash
# Renew certificates manually
docker-compose --profile ssl run --rm certbot renew

# Restart nginx after renewal
docker-compose restart nginx
```

### Check Certificate Status

```bash
# View certificate information
docker-compose exec nginx openssl x509 -in /etc/letsencrypt/live/yourdomain.com/cert.pem -text -noout
```

## 🗄️ Database Management

### Access Database

```bash
# Connect to production database
docker-compose exec postgres psql -U ffball -d ffball_prod

# Connect to development database
docker-compose exec postgres-dev psql -U ffball -d ffball_dev
```

### Backup Database

```bash
# Backup production database
docker-compose exec postgres pg_dump -U ffball ffball_prod > backup_prod.sql

# Backup development database
docker-compose exec postgres-dev pg_dump -U ffball ffball_dev > backup_dev.sql
```

### Restore Database

```bash
# Restore production database
docker-compose exec -T postgres psql -U ffball -d ffball_prod < backup_prod.sql

# Restore development database
docker-compose exec -T postgres-dev psql -U ffball -d ffball_dev < backup_dev.sql
```

## 🔄 CI/CD Integration

The GitHub Actions workflow has been updated to use Docker:

### Development Deployment

- Triggers on push to `develop` branch
- Updates `backend-dev` and `frontend-dev` containers
- No downtime deployment

### Production Deployment

- Triggers on push to `main` branch
- Updates `backend` and `frontend` containers
- No downtime deployment

## 🛠️ Troubleshooting

### Common Issues

1. **Port Already in Use**

   ```bash
   # Check what's using the port
   sudo netstat -tulpn | grep :80

   # Stop conflicting services
   sudo systemctl stop nginx
   sudo systemctl stop apache2
   ```

2. **Container Won't Start**

   ```bash
   # Check container logs
   docker-compose logs backend

   # Check container status
   docker-compose ps
   ```

3. **Database Connection Issues**

   ```bash
   # Check database container
   docker-compose logs postgres

   # Test database connection
   docker-compose exec postgres pg_isready -U ffball
   ```

4. **SSL Certificate Issues**

   ```bash
   # Check certificate validity
   openssl s_client -connect yourdomain.com:443 -servername yourdomain.com

   # Regenerate certificates
   docker-compose --profile ssl run --rm certbot
   ```

### Debugging Commands

```bash
# Enter a running container
docker-compose exec backend bash

# View container details
docker inspect ffball-backend

# Check container health
docker inspect --format='{{.State.Health.Status}}' ffball-backend

# View nginx configuration
docker-compose exec nginx nginx -t
```

## 📈 Scaling

### Add More Backend Instances

```bash
# Scale backend to 3 instances
docker-compose up -d --scale backend=3

# Update nginx configuration for load balancing
```

### Add More Frontend Instances

```bash
# Scale frontend to 2 instances
docker-compose up -d --scale frontend=2
```

## 🔧 Customization

### Environment Variables

Create `.env` file for custom configuration:

```bash
# Database
POSTGRES_PASSWORD=your_secure_password
DATABASE_URL=postgresql://ffball:your_secure_password@postgres:5432/ffball_prod

# Backend
RUST_LOG=debug
PORT=8080

# Frontend
NODE_ENV=production
```

### Custom Nginx Configuration

Edit `nginx/nginx.conf` for custom routing, caching, or security settings.

### Custom Docker Images

Modify `backend/Dockerfile` or `frontend/Dockerfile` for custom builds.

## 💰 Cost Optimization

### Resource Limits

Add resource limits to `docker-compose.yml`:

```yaml
services:
  backend:
    deploy:
      resources:
        limits:
          cpus: "0.5"
          memory: 512M
        reservations:
          cpus: "0.25"
          memory: 256M
```

### Cleanup

```bash
# Remove unused images
docker image prune -a

# Remove unused volumes
docker volume prune

# Remove unused networks
docker network prune

# Clean everything
docker system prune -a
```

## 🎯 Next Steps

1. **Set up monitoring** with Prometheus/Grafana
2. **Configure backups** for databases
3. **Set up logging** aggregation
4. **Add load balancing** for high traffic
5. **Implement blue-green deployments**

Your Fantasy Football Draft Board is now running in a fully containerized environment with easy deployment and management!
