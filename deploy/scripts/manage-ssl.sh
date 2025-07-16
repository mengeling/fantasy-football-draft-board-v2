#!/bin/bash
set -e

# SSL Certificate Management Script
# This script helps manage Let's Encrypt SSL certificates

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to show usage
show_usage() {
    echo "Usage: $0 [COMMAND] [OPTIONS]"
    echo ""
    echo "Commands:"
    echo "  setup <domain>     Setup SSL certificate for domain"
    echo "  renew              Renew existing certificates"
    echo "  status             Check certificate status"
    echo "  test               Test SSL configuration"
    echo "  troubleshoot       Run SSL troubleshooting"
    echo ""
    echo "Examples:"
    echo "  $0 setup mydomain.com"
    echo "  $0 renew"
    echo "  $0 status"
}

# Function to check if running on server
check_server() {
    if [ ! -f /etc/nginx/sites-available/ffball ]; then
        print_error "This script must be run on the server"
        exit 1
    fi
}

# Function to setup SSL certificate
setup_ssl() {
    local domain=$1
    
    if [ -z "$domain" ]; then
        print_error "Domain name is required"
        show_usage
        exit 1
    fi
    
    print_status "Setting up SSL certificate for $domain..."
    
    # Check if domain resolves
    if ! nslookup $domain > /dev/null 2>&1; then
        print_error "Domain $domain does not resolve. Please check DNS configuration."
        exit 1
    fi
    
    # Stop nginx temporarily for certbot
    systemctl stop nginx
    
    # Obtain certificate
    certbot certonly --standalone -d $domain --non-interactive --agree-tos --email admin@$domain
    
    # Start nginx
    systemctl start nginx
    
    # Test nginx configuration
    if nginx -t; then
        print_success "SSL certificate setup completed for $domain"
    else
        print_error "Nginx configuration test failed"
        exit 1
    fi
}

# Function to renew certificates
renew_ssl() {
    print_status "Renewing SSL certificates..."
    
    # Test renewal without actually renewing
    certbot renew --dry-run
    
    if [ $? -eq 0 ]; then
        print_status "Dry run successful. Running actual renewal..."
        certbot renew --quiet
        
        # Reload nginx to pick up new certificates
        systemctl reload nginx
        
        print_success "SSL certificates renewed successfully"
    else
        print_error "SSL certificate renewal failed"
        exit 1
    fi
}

# Function to check certificate status
check_status() {
    print_status "Checking SSL certificate status..."
    
    # Check certificate expiration
    local cert_file="/etc/letsencrypt/live/*/fullchain.pem"
    if ls $cert_file > /dev/null 2>&1; then
        for cert in $cert_file; do
            local domain=$(basename $(dirname $cert))
            local expiry=$(openssl x509 -enddate -noout -in $cert | cut -d= -f2)
            local days_left=$(echo $(( ($(date -d "$expiry" +%s) - $(date +%s)) / 86400 )))
            
            echo "Domain: $domain"
            echo "Expires: $expiry"
            echo "Days remaining: $days_left"
            
            if [ $days_left -lt 30 ]; then
                print_warning "Certificate expires in less than 30 days"
            else
                print_success "Certificate is valid"
            fi
            echo ""
        done
    else
        print_warning "No SSL certificates found"
    fi
    
    # Check nginx status
    if systemctl is-active --quiet nginx; then
        print_success "Nginx is running"
    else
        print_error "Nginx is not running"
    fi
}

# Function to test SSL configuration
test_ssl() {
    print_status "Testing SSL configuration..."
    
    # Test nginx configuration
    if nginx -t; then
        print_success "Nginx configuration is valid"
    else
        print_error "Nginx configuration is invalid"
        exit 1
    fi
    
    # Test SSL certificate
    local cert_file="/etc/letsencrypt/live/*/fullchain.pem"
    if ls $cert_file > /dev/null 2>&1; then
        for cert in $cert_file; do
            local domain=$(basename $(dirname $cert))
            print_status "Testing certificate for $domain..."
            
            # Test certificate validity
            if openssl x509 -checkend 0 -noout -in $cert; then
                print_success "Certificate is valid"
            else
                print_error "Certificate has expired"
            fi
            
            # Test SSL connection
            if curl -s -I https://$domain > /dev/null 2>&1; then
                print_success "HTTPS connection successful"
            else
                print_error "HTTPS connection failed"
            fi
        done
    else
        print_warning "No SSL certificates found"
    fi
}

# Function to troubleshoot SSL issues
troubleshoot() {
    print_status "Running SSL troubleshooting..."
    
    echo "=== System Information ==="
    echo "OS: $(lsb_release -d | cut -f2)"
    echo "Nginx version: $(nginx -v 2>&1)"
    echo "Certbot version: $(certbot --version)"
    echo ""
    
    echo "=== Nginx Status ==="
    systemctl status nginx --no-pager -l
    echo ""
    
    echo "=== Nginx Configuration ==="
    nginx -T | grep -E "(server_name|ssl_certificate)" || true
    echo ""
    
    echo "=== Certificate Files ==="
    ls -la /etc/letsencrypt/live/*/ 2>/dev/null || echo "No certificates found"
    echo ""
    
    echo "=== Firewall Status ==="
    ufw status || echo "UFW not installed"
    echo ""
    
    echo "=== DNS Resolution ==="
    local domain=$(grep "server_name" /etc/nginx/sites-available/ffball | head -1 | awk '{print $2}' | sed 's/;$//')
    if [ -n "$domain" ]; then
        echo "Domain: $domain"
        nslookup $domain || echo "DNS resolution failed"
    fi
    echo ""
    
    echo "=== SSL Certificate Details ==="
    local cert_file="/etc/letsencrypt/live/*/fullchain.pem"
    if ls $cert_file > /dev/null 2>&1; then
        for cert in $cert_file; do
            echo "Certificate: $cert"
            openssl x509 -in $cert -text -noout | grep -E "(Subject:|Issuer:|Not Before|Not After)" || true
            echo ""
        done
    fi
}

# Main script logic
case "${1:-}" in
    setup)
        check_server
        setup_ssl "$2"
        ;;
    renew)
        check_server
        renew_ssl
        ;;
    status)
        check_server
        check_status
        ;;
    test)
        check_server
        test_ssl
        ;;
    troubleshoot)
        check_server
        troubleshoot
        ;;
    *)
        show_usage
        exit 1
        ;;
esac 