#!/bin/bash

# Docker Health Check Script for Fantasy Football Draft Board

set -e

echo "🐳 Docker Health Check Report"
echo "============================="
echo ""

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker is not running"
    exit 1
fi

# Check if docker-compose is available
if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose is not installed"
    exit 1
fi

# Check container status
echo "📊 Container Status:"
echo "-------------------"
docker-compose ps

echo ""

# Check container health
echo "🏥 Container Health:"
echo "-------------------"
for service in backend backend-dev frontend frontend-dev postgres postgres-dev nginx; do
    if docker-compose ps $service | grep -q "Up"; then
        # Check if container is healthy
        HEALTH_STATUS=$(docker inspect --format='{{.State.Health.Status}}' ffball-$service 2>/dev/null || echo "no-health-check")
        if [ "$HEALTH_STATUS" = "healthy" ]; then
            echo "✅ $service: Healthy"
        elif [ "$HEALTH_STATUS" = "no-health-check" ]; then
            echo "✅ $service: Running (no health check)"
        else
            echo "⚠️  $service: Running but unhealthy ($HEALTH_STATUS)"
        fi
    else
        echo "❌ $service: Not running"
    fi
done

echo ""

# Check database connections
echo "🗄️  Database Status:"
echo "-------------------"
echo "Production Database:"
if docker-compose exec -T postgres pg_isready -U ffball -d ffball_prod > /dev/null 2>&1; then
    echo "✅ Connected"
else
    echo "❌ Connection failed"
fi

echo "Development Database:"
if docker-compose exec -T postgres-dev pg_isready -U ffball -d ffball_dev > /dev/null 2>&1; then
    echo "✅ Connected"
else
    echo "❌ Connection failed"
fi

echo ""

# Check web endpoints
echo "🌐 Web Endpoints:"
echo "----------------"
EC2_IP=$(curl -s http://169.254.169.254/latest/meta-data/public-ipv4)

echo "Production Frontend:"
if curl -s -o /dev/null -w "%{http_code}" "http://$EC2_IP" | grep -q "200"; then
    echo "✅ Responding"
else
    echo "❌ Not responding"
fi

echo "Development Frontend:"
if curl -s -o /dev/null -w "%{http_code}" "http://dev.$EC2_IP" | grep -q "200"; then
    echo "✅ Responding"
else
    echo "❌ Not responding"
fi

echo "Production API:"
if curl -s -o /dev/null -w "%{http_code}" "http://$EC2_IP/api/players" | grep -q "200\|404"; then
    echo "✅ Responding"
else
    echo "❌ Not responding"
fi

echo "Development API:"
if curl -s -o /dev/null -w "%{http_code}" "http://dev.$EC2_IP/api/players" | grep -q "200\|404"; then
    echo "✅ Responding"
else
    echo "❌ Not responding"
fi

echo ""

# Check SSL certificates if they exist
if [ -d "nginx/certbot" ] && [ "$(ls -A nginx/certbot 2>/dev/null)" ]; then
    echo "🔒 SSL Certificate Status:"
    echo "--------------------------"
    
    # Find domain from certbot directory
    DOMAIN=$(find nginx/certbot -maxdepth 1 -type d -name "*.com" -o -name "*.net" -o -name "*.org" | head -1 | xargs basename 2>/dev/null || echo "")
    
    if [ ! -z "$DOMAIN" ]; then
        echo "Production Certificate:"
        if openssl s_client -connect $DOMAIN:443 -servername $DOMAIN < /dev/null 2>/dev/null | openssl x509 -noout -dates | head -1 | grep -q "notAfter"; then
            echo "✅ Valid"
        else
            echo "❌ Invalid/Expired"
        fi
        
        echo "Development Certificate:"
        if openssl s_client -connect dev.$DOMAIN:443 -servername dev.$DOMAIN < /dev/null 2>/dev/null | openssl x509 -noout -dates | head -1 | grep -q "notAfter"; then
            echo "✅ Valid"
        else
            echo "❌ Invalid/Expired"
        fi
    fi
fi

echo ""

# Check disk usage
echo "💾 Disk Usage:"
echo "-------------"
df -h / | tail -1 | awk '{print "Root: " $5 " used (" $3 "/" $2 ")"}'

# Check Docker disk usage
echo "Docker:"
docker system df --format "table {{.Type}}\t{{.TotalCount}}\t{{.Size}}\t{{.Reclaimable}}"

echo ""

# Check recent logs
echo "📝 Recent Logs (last 3 lines per service):"
echo "------------------------------------------"
for service in backend backend-dev frontend frontend-dev nginx; do
    echo "$service:"
    docker-compose logs --tail=3 $service 2>/dev/null || echo "  No logs available"
    echo ""
done

echo "✅ Docker health check complete!" 