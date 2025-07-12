#!/bin/bash

# Health check script for both environments
# Run this to check the status of all services

set -e

echo "🔍 Health Check Report"
echo "======================"
echo ""

# Check systemd services
echo "📊 Service Status:"
echo "------------------"
echo "Production Backend:"
sudo systemctl is-active ffball && echo "✅ Running" || echo "❌ Stopped"

echo "Development Backend:"
sudo systemctl is-active ffball-dev && echo "✅ Running" || echo "❌ Stopped"

echo "Nginx:"
sudo systemctl is-active nginx && echo "✅ Running" || echo "❌ Stopped"

echo "PostgreSQL:"
sudo systemctl is-active postgresql && echo "✅ Running" || echo "❌ Stopped"

echo ""

# Check database connections
echo "🗄️  Database Status:"
echo "-------------------"
echo "Production Database:"
PGPASSWORD=ffball psql -U ffball -d ffball_prod -c "SELECT 1;" > /dev/null 2>&1 && echo "✅ Connected" || echo "❌ Connection Failed"

echo "Development Database:"
PGPASSWORD=ffball psql -U ffball -d ffball_dev -c "SELECT 1;" > /dev/null 2>&1 && echo "✅ Connected" || echo "❌ Connection Failed"

echo ""

# Check web endpoints
echo "🌐 Web Endpoints:"
echo "----------------"
PROD_IP=$(curl -s http://169.254.169.254/latest/meta-data/public-ipv4)

# Check if domain is configured
if [ -f "/etc/letsencrypt/live" ]; then
    # Get domain from SSL certificate
    DOMAIN=$(sudo find /etc/letsencrypt/live -maxdepth 1 -type d -name "*.com" -o -name "*.net" -o -name "*.org" | head -1 | xargs basename 2>/dev/null || echo "")
    
    if [ ! -z "$DOMAIN" ]; then
        echo "Production Frontend (HTTPS):"
        curl -s -o /dev/null -w "%{http_code}" "https://$DOMAIN" | grep -q "200" && echo "✅ Responding" || echo "❌ Not Responding"
        
        echo "Development Frontend (HTTPS):"
        curl -s -o /dev/null -w "%{http_code}" "https://dev.$DOMAIN" | grep -q "200" && echo "✅ Responding" || echo "❌ Not Responding"
        
        echo "Production API (HTTPS):"
        curl -s -o /dev/null -w "%{http_code}" "https://$DOMAIN/api/players" | grep -q "200\|404" && echo "✅ Responding" || echo "❌ Not Responding"
        
        echo "Development API (HTTPS):"
        curl -s -o /dev/null -w "%{http_code}" "https://dev.$DOMAIN/api/players" | grep -q "200\|404" && echo "✅ Responding" || echo "❌ Not Responding"
        
        echo ""
        echo "🔒 SSL Certificate Status:"
        echo "--------------------------"
        echo "Production Certificate:"
        openssl s_client -connect $DOMAIN:443 -servername $DOMAIN < /dev/null 2>/dev/null | openssl x509 -noout -dates | head -1 | grep -q "notAfter" && echo "✅ Valid" || echo "❌ Invalid/Expired"
        
        echo "Development Certificate:"
        openssl s_client -connect dev.$DOMAIN:443 -servername dev.$DOMAIN < /dev/null 2>/dev/null | openssl x509 -noout -dates | head -1 | grep -q "notAfter" && echo "✅ Valid" || echo "❌ Invalid/Expired"
    else
        echo "Production Frontend (HTTP):"
        curl -s -o /dev/null -w "%{http_code}" "http://$PROD_IP" | grep -q "200" && echo "✅ Responding" || echo "❌ Not Responding"
        
        echo "Development Frontend (HTTP):"
        curl -s -o /dev/null -w "%{http_code}" "http://dev.$PROD_IP" | grep -q "200" && echo "✅ Responding" || echo "❌ Not Responding"
        
        echo "Production API (HTTP):"
        curl -s -o /dev/null -w "%{http_code}" "http://$PROD_IP/api/players" | grep -q "200\|404" && echo "✅ Responding" || echo "❌ Not Responding"
        
        echo "Development API (HTTP):"
        curl -s -o /dev/null -w "%{http_code}" "http://dev.$PROD_IP/api/players" | grep -q "200\|404" && echo "✅ Responding" || echo "❌ Not Responding"
        
        echo ""
        echo "⚠️  SSL not configured. Run: ./scripts/setup-ssl.sh yourdomain.com"
    fi
else
    echo "Production Frontend (HTTP):"
    curl -s -o /dev/null -w "%{http_code}" "http://$PROD_IP" | grep -q "200" && echo "✅ Responding" || echo "❌ Not Responding"
    
    echo "Development Frontend (HTTP):"
    curl -s -o /dev/null -w "%{http_code}" "http://dev.$PROD_IP" | grep -q "200" && echo "✅ Responding" || echo "❌ Not Responding"
    
    echo "Production API (HTTP):"
    curl -s -o /dev/null -w "%{http_code}" "http://$PROD_IP/api/players" | grep -q "200\|404" && echo "✅ Responding" || echo "❌ Not Responding"
    
    echo "Development API (HTTP):"
    curl -s -o /dev/null -w "%{http_code}" "http://dev.$PROD_IP/api/players" | grep -q "200\|404" && echo "✅ Responding" || echo "❌ Not Responding"
    
    echo ""
    echo "⚠️  SSL not configured. Run: ./scripts/setup-ssl.sh yourdomain.com"
fi

echo ""

# Check disk space
echo "💾 Disk Usage:"
echo "-------------"
df -h / | tail -1 | awk '{print "Root: " $5 " used (" $3 "/" $2 ")"}'
df -h /var/www | tail -1 | awk '{print "Web: " $5 " used (" $3 "/" $2 ")"}'

echo ""

# Check recent logs
echo "📝 Recent Logs (last 5 lines):"
echo "------------------------------"
echo "Production Backend:"
sudo journalctl -u ffball -n 5 --no-pager | tail -5

echo ""
echo "Development Backend:"
sudo journalctl -u ffball-dev -n 5 --no-pager | tail -5

echo ""
echo "✅ Health check complete!" 