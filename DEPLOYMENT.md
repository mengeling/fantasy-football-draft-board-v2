# Deployment Guide

This guide explains how to deploy the Fantasy Football Draft Board application using various methods.

## Deployment Methods

### 1. GitHub Actions Workflow Dispatch (Recommended for Feature Branches)

The easiest way to deploy from any branch is using GitHub Actions workflow dispatch.

#### Quick Start

```bash
# Get the deployment URL
just deploy-github
```

#### Manual Steps

1. **Navigate to GitHub Actions**:

   - Go to your repository on GitHub
   - Click on the "Actions" tab
   - Find the "Deploy" workflow
   - Click "Run workflow"

2. **Configure Deployment**:

   - **Environment**: Choose `staging` for testing or `production` for live deployment
   - **Git branch**: Enter your feature branch name (e.g., `feature/new-feature`)
   - **Force recreate**: Check this to re-run the user_data script (useful if setup failed)
   - **Skip infrastructure**: Check this to deploy only the application (faster for code changes)

3. **Run the Workflow**:
   - Click "Run workflow"
   - Monitor the progress in the Actions tab

#### Use Cases

- **New Feature Testing**: Deploy your feature branch to staging
- **Bug Fixes**: Deploy fixes to production
- **Infrastructure Issues**: Use "Force recreate" to re-run user_data script
- **Quick Code Updates**: Use "Skip infrastructure" for faster deployments

### 2. Local Deployment

For local development and testing:

```bash
# Deploy to production
just deploy

# Deploy infrastructure only
just infra

# Deploy application only
just app
```

### 3. Automated Deployment

Push to the `main` branch to trigger automatic deployment to production.

## Environment Configuration

### Staging Environment

- **Purpose**: Testing new features and changes
- **URL**: `staging.yourdomain.com` (if configured)
- **Branch**: Any feature branch
- **Auto-deploy**: No (manual via workflow dispatch)

### Production Environment

- **Purpose**: Live application
- **URL**: `yourdomain.com`
- **Branch**: `main`
- **Auto-deploy**: Yes (on push to main)

## Deployment Options

### Force Recreate Infrastructure

Use this option when:

- The user_data script didn't complete properly
- You need to reinstall software on the server
- Infrastructure changes require a fresh instance

**Warning**: This will destroy and recreate the EC2 instance, causing downtime.

### Skip Infrastructure

Use this option when:

- You only changed application code
- You want faster deployments
- The infrastructure is already set up correctly

**Benefit**: Much faster deployment (2-3 minutes vs 10-15 minutes).

## Troubleshooting

### User Data Script Issues

If the user_data script didn't complete:

1. **Check the logs**:

   ```bash
   ssh ubuntu@your-instance-ip
   sudo cat /var/log/cloud-init-output.log
   ```

2. **Re-run the script**:
   - Use "Force recreate" in GitHub Actions
   - Or run locally: `terraform apply -replace="aws_instance.web"`

### Deployment Failures

1. **Check GitHub Actions logs** for specific error messages
2. **Verify AWS credentials** are configured correctly
3. **Check SSH key** is added to GitHub secrets
4. **Verify domain configuration** if using custom domains

### Health Check Failures

If the application doesn't respond after deployment:

1. **SSH into the instance**:

   ```bash
   ssh ubuntu@your-instance-ip
   ```

2. **Check Docker services**:

   ```bash
   cd /home/ubuntu/app
   docker-compose ps
   docker-compose logs
   ```

3. **Check application logs**:
   ```bash
   docker-compose logs backend
   docker-compose logs frontend
   ```

## Security Considerations

- **Environment Secrets**: Stored in GitHub repository secrets
- **SSH Keys**: Private key stored in GitHub secrets, public key in AWS
- **State Encryption**: Terraform state is encrypted in S3
- **SSL Certificates**: Automatically managed by Let's Encrypt

## Best Practices

1. **Always test in staging first**
2. **Use feature branches** for development
3. **Review changes** before deploying to production
4. **Monitor deployments** in GitHub Actions
5. **Keep infrastructure changes minimal** during active development
6. **Use "Skip infrastructure"** for code-only changes

## Monitoring

- **GitHub Actions**: Monitor deployment progress
- **AWS Console**: Check EC2 instance status
- **Application Health**: Check `/health` endpoint
- **Logs**: SSH into instance for detailed logs
