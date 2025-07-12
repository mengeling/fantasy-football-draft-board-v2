#!/bin/bash

# Domain Setup Helper Script
# This script helps you set up your domain configuration

set -e

echo "🌐 Domain Setup Helper"
echo "======================"
echo ""

# Check if domain is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <your-domain.com>"
    echo "Example: $0 fantasyfootball.com"
    echo ""
    echo "This script will help you:"
    echo "1. Get your EC2 IP address"
    echo "2. Show DNS configuration instructions"
    echo "3. Test DNS propagation"
    echo "4. Set up SSL certificates"
    exit 1
fi

DOMAIN=$1
DEV_SUBDOMAIN="dev.$DOMAIN"

echo "Domain: $DOMAIN"
echo "Development subdomain: $DEV_SUBDOMAIN"
echo ""

# Get EC2 IP address
echo "📋 Your EC2 IP Address:"
echo "----------------------"
EC2_IP=$(curl -s http://169.254.169.254/latest/meta-data/public-ipv4)
echo "EC2 Public IP: $EC2_IP"
echo ""

# Show DNS configuration
echo "📝 DNS Configuration Required:"
echo "-----------------------------"
echo "In your domain registrar's DNS settings, add these A records:"
echo ""
echo "Type | Name | Value | TTL"
echo "-----|------|-------|-----"
echo "A    | @    | $EC2_IP | 300"
echo "A    | dev  | $EC2_IP | 300"
echo ""
echo "This will make:"
echo "  $DOMAIN → $EC2_IP"
echo "  $DEV_SUBDOMAIN → $EC2_IP"
echo ""

# Check if DNS is already configured
echo "🔍 Checking DNS Configuration:"
echo "-----------------------------"
echo "Production domain ($DOMAIN):"
if nslookup $DOMAIN > /dev/null 2>&1; then
    RESOLVED_IP=$(nslookup $DOMAIN | grep -A1 "Name:" | tail -1 | awk '{print $2}')
    if [ "$RESOLVED_IP" = "$EC2_IP" ]; then
        echo "✅ Correctly configured ($RESOLVED_IP)"
    else
        echo "⚠️  Configured but pointing to wrong IP ($RESOLVED_IP instead of $EC2_IP)"
    fi
else
    echo "❌ Not configured yet"
fi

echo "Development subdomain ($DEV_SUBDOMAIN):"
if nslookup $DEV_SUBDOMAIN > /dev/null 2>&1; then
    RESOLVED_IP=$(nslookup $DEV_SUBDOMAIN | grep -A1 "Name:" | tail -1 | awk '{print $2}')
    if [ "$RESOLVED_IP" = "$EC2_IP" ]; then
        echo "✅ Correctly configured ($RESOLVED_IP)"
    else
        echo "⚠️  Configured but pointing to wrong IP ($RESOLVED_IP instead of $EC2_IP)"
    fi
else
    echo "❌ Not configured yet"
fi

echo ""

# Check if SSL is already configured
echo "🔒 SSL Certificate Status:"
echo "-------------------------"
if [ -d "/etc/letsencrypt/live/$DOMAIN" ]; then
    echo "✅ Production SSL certificate exists"
else
    echo "❌ Production SSL certificate not found"
fi

if [ -d "/etc/letsencrypt/live/$DEV_SUBDOMAIN" ]; then
    echo "✅ Development SSL certificate exists"
else
    echo "❌ Development SSL certificate not found"
fi

echo ""

# Provide next steps
echo "📋 Next Steps:"
echo "-------------"

if ! nslookup $DOMAIN > /dev/null 2>&1; then
    echo "1. Configure DNS records in your domain registrar:"
    echo "   - A record: @ → $EC2_IP"
    echo "   - A record: dev → $EC2_IP"
    echo "   - Wait 5-60 minutes for DNS propagation"
    echo ""
fi

if [ ! -d "/etc/letsencrypt/live/$DOMAIN" ]; then
    echo "2. Set up SSL certificates:"
    echo "   ./scripts/setup-ssl.sh $DOMAIN"
    echo ""
fi

echo "3. Test your setup:"
echo "   ./scripts/health-check.sh"
echo ""

echo "4. Update GitHub Actions secrets:"
echo "   - EC2_HOST: $DOMAIN (instead of IP)"
echo ""

# Offer to run SSL setup if DNS is ready
if nslookup $DOMAIN > /dev/null 2>&1 && [ ! -d "/etc/letsencrypt/live/$DOMAIN" ]; then
    echo ""
    read -p "DNS appears to be configured. Would you like to set up SSL certificates now? (y/n): " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "Setting up SSL certificates..."
        ./scripts/setup-ssl.sh $DOMAIN
    fi
fi

echo ""
echo "✅ Domain setup helper complete!" 