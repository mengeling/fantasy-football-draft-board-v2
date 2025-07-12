# Domain and SSL Setup Guide

This guide will help you set up a custom domain name and SSL certificates for your Fantasy Football Draft Board.

## Prerequisites

1. A domain name (purchased from a registrar like Namecheap, GoDaddy, Google Domains, etc.)
2. Access to your domain's DNS settings
3. Your EC2 instance running and accessible

## Step 1: Choose Your Domain

### Recommended Domain Structure

- **Production**: `yourdomain.com` (e.g., `fantasyfootball.com`)
- **Development**: `dev.yourdomain.com` (e.g., `dev.fantasyfootball.com`)

### Domain Name Ideas

Here are some domain name suggestions for your fantasy football draft board:

- `fantasydraft.com`
- `draftboard.com`
- `ffdraft.com`
- `fantasyfootball.com`
- `draftpro.com`
- `fantasyboard.com`
- `draftcentral.com`

## Step 2: Purchase Your Domain

### Popular Domain Registrars

1. **Namecheap** (Recommended)

   - Good prices
   - Free privacy protection
   - Easy DNS management

2. **Google Domains**

   - Clean interface
   - Good integration with Google services

3. **GoDaddy**

   - Widely used
   - Good customer support

4. **Cloudflare Registrar**
   - Free privacy protection
   - Good security features

### Purchase Steps

1. Go to your chosen registrar
2. Search for your desired domain name
3. Add it to cart and complete purchase
4. Note: You'll need access to DNS settings

## Step 3: Configure DNS Records

### Get Your EC2 IP Address

First, get your EC2 instance's public IP address:

```bash
# On your EC2 instance
curl -s http://169.254.169.254/latest/meta-data/public-ipv4
```

Or check your AWS Console → EC2 → Instances → Your Instance → Public IPv4 address.

### Add DNS Records

In your domain registrar's DNS settings, add these A records:

| Type | Name | Value         | TTL |
| ---- | ---- | ------------- | --- |
| A    | @    | `YOUR_EC2_IP` | 300 |
| A    | dev  | `YOUR_EC2_IP` | 300 |

**Example:**

- If your domain is `fantasydraft.com` and EC2 IP is `100.29.78.245`:
  - A record: `@` → `100.29.78.245`
  - A record: `dev` → `100.29.78.245`

### DNS Propagation

DNS changes can take 5-60 minutes to propagate globally. You can check propagation with:

```bash
# Check if DNS is resolving
nslookup yourdomain.com
nslookup dev.yourdomain.com

# Or use online tools like:
# https://www.whatsmydns.net/
```

## Step 4: Set Up SSL Certificates

### Run the SSL Setup Script

On your EC2 instance:

```bash
# Make the script executable
chmod +x scripts/setup-ssl.sh

# Run the SSL setup (replace with your domain)
./scripts/setup-ssl.sh yourdomain.com
```

### What the Script Does

1. Installs Certbot (Let's Encrypt client)
2. Creates temporary nginx configuration for certificate validation
3. Generates SSL certificates for both domains
4. Configures nginx with SSL and security headers
5. Sets up automatic certificate renewal

### Manual SSL Setup (Alternative)

If you prefer to set up SSL manually:

```bash
# Install certbot
sudo apt update
sudo apt install -y certbot python3-certbot-nginx

# Generate certificates
sudo certbot --nginx -d yourdomain.com -d dev.yourdomain.com

# Set up auto-renewal
sudo crontab -e
# Add: 0 12 * * * /usr/bin/certbot renew --quiet
```

## Step 5: Update Configuration

### Update GitHub Actions

Update your GitHub repository secrets:

1. Go to Settings → Secrets and variables → Actions
2. Update `EC2_HOST` with your domain name
3. Add new secrets if needed for domain-specific configurations

### Update Application Configuration

If your application has any hardcoded URLs, update them:

```bash
# Check for hardcoded URLs in your codebase
grep -r "100.29.78.245" .
grep -r "http://" frontend/src/
```

## Step 6: Test Your Setup

### Test DNS Resolution

```bash
# Test production domain
curl -I http://yourdomain.com
curl -I https://yourdomain.com

# Test development domain
curl -I http://dev.yourdomain.com
curl -I https://dev.yourdomain.com
```

### Test SSL Certificates

```bash
# Check certificate validity
openssl s_client -connect yourdomain.com:443 -servername yourdomain.com
openssl s_client -connect dev.yourdomain.com:443 -servername dev.yourdomain.com
```

### Test Application Functionality

1. Visit `https://yourdomain.com` in your browser
2. Check that the SSL padlock appears
3. Test the application functionality
4. Repeat for `https://dev.yourdomain.com`

## Step 7: Security Considerations

### Security Headers

The SSL setup script includes these security headers:

- **HSTS**: Forces HTTPS connections
- **X-Frame-Options**: Prevents clickjacking
- **X-Content-Type-Options**: Prevents MIME type sniffing
- **X-XSS-Protection**: Basic XSS protection

### SSL Configuration

- TLS 1.2 and 1.3 only
- Strong cipher suites
- SSL session caching
- Automatic redirects from HTTP to HTTPS

## Troubleshooting

### Common Issues

1. **DNS Not Resolving**

   ```bash
   # Check DNS propagation
   dig yourdomain.com
   nslookup yourdomain.com
   ```

2. **SSL Certificate Errors**

   ```bash
   # Check certificate status
   sudo certbot certificates

   # Renew certificates manually
   sudo certbot renew
   ```

3. **Nginx Configuration Errors**

   ```bash
   # Test nginx configuration
   sudo nginx -t

   # Check nginx logs
   sudo tail -f /var/log/nginx/error.log
   ```

4. **Certificate Renewal Issues**
   ```bash
   # Test renewal process
   sudo certbot renew --dry-run
   ```

### SSL Labs Test

Test your SSL configuration with SSL Labs:

1. Go to https://www.ssllabs.com/ssltest/
2. Enter your domain name
3. Check for any security issues

## Maintenance

### Certificate Renewal

Certificates automatically renew every 60 days. Check renewal status:

```bash
# Check renewal status
sudo certbot renew --dry-run

# View certificate expiration
sudo certbot certificates
```

### Monitoring

Add SSL monitoring to your health check:

```bash
# Add to your health check script
echo "SSL Certificate Status:"
echo "Production:"
openssl s_client -connect yourdomain.com:443 -servername yourdomain.com < /dev/null 2>/dev/null | openssl x509 -noout -dates

echo "Development:"
openssl s_client -connect dev.yourdomain.com:443 -servername dev.yourdomain.com < /dev/null 2>/dev/null | openssl x509 -noout -dates
```

## Cost Considerations

### Domain Registration

- **Domain**: $10-15/year
- **Privacy Protection**: Often free or $5-10/year

### SSL Certificates

- **Let's Encrypt**: Free
- **Commercial certificates**: $50-200/year (not needed)

### Total Annual Cost

- **Domain + Privacy**: $15-25/year
- **SSL**: Free
- **Total**: ~$20/year

## Next Steps

1. **Set up monitoring** for SSL certificate expiration
2. **Configure backup** for your SSL certificates
3. **Consider CDN** (Cloudflare) for better performance
4. **Set up email** forwarding for your domain
5. **Configure subdomains** for additional services (api.yourdomain.com, etc.)

Your Fantasy Football Draft Board will now be accessible via your custom domain with secure HTTPS connections!
