# Justfile Task Runner Guide

This guide explains how to use the `justfile` task runner for managing your Fantasy Football Draft Board deployment and operations.

## 🎯 What is Justfile?

`justfile` is a modern task runner that's like `make` but simpler and more powerful. It allows you to define tasks in a clean, readable format and run them with simple commands.

### **Benefits:**

- **Simple syntax** - Easy to read and write
- **Built-in help** - Run `just --list` to see all commands
- **Parameter support** - Pass arguments to tasks
- **Cross-platform** - Works on Linux, macOS, and Windows
- **No dependencies** - Single binary, no complex setup

## 🚀 Installation

### **On Ubuntu/Debian:**

```bash
# Install just
curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash

# Add to PATH (if not already added)
export PATH="$HOME/.local/bin:$PATH"
```

### **On macOS:**

```bash
# Using Homebrew
brew install just

# Or using the installer
curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash
```

### **On Windows:**

```bash
# Using Chocolatey
choco install just

# Or download from GitHub releases
```

## 📋 Available Commands

Run `just --list` to see all available commands:

```bash
Available recipes:
    backup                    # Backup all databases
    backup-dev               # Backup development database
    backup-prod              # Backup production database
    build-backend            # Build backend
    build-frontend           # Build frontend
    ci-deploy-dev            # Deploy to development (for CI/CD)
    ci-deploy-prod           # Deploy to production (for CI/CD)
    clean                    # Remove all containers and volumes (⚠️ destructive)
    cleanup                  # Clean up Docker system
    config                   # Show configuration
    db-dev                   # Connect to development database
    db-prod                  # Connect to production database
    default                  # Default recipe to run when no arguments are provided
    deploy domain=""         # Deploy the entire application (Docker mode)
    deploy-dev               # Deploy only development environment
    deploy-manual            # Deploy manually (systemd services)
    deploy-prod              # Deploy only production environment
    dev                      # Development mode (with hot reload)
    health                   # Run comprehensive health check
    help                     # Show help and available commands
    info                     # Show system information
    logs service=""          # Watch logs in real-time
    logs-recent service=""   # Show recent logs
    restart                  # Restart all services
    restart-service service  # Restart specific service
    restore-dev file         # Restore development database
    restore-prod file        # Restore production database
    ssl domain               # Set up SSL certificates
    ssl-renew                # Renew SSL certificates
    setup-domain domain      # Set up domain configuration
    start                    # Start all services
    status                   # Check status of all services
    stop                     # Stop all services
    test                     # Run all tests
    test-backend             # Run backend tests
    test-frontend            # Run frontend tests
    update                   # Update and rebuild all services
    update-service service   # Update specific service
    urls                     # Show environment URLs
```

## 🚀 Quick Start

### **1. Deploy Everything (Docker)**

```bash
# Deploy without domain
just deploy

# Deploy with domain and SSL
just deploy fantasyfootball.com
```

### **2. Check Status**

```bash
# Check all services
just status

# Run health check
just health
```

### **3. View Logs**

```bash
# All logs
just logs

# Specific service
just logs backend
```

## 📖 Command Categories

### **🚀 Deployment Commands**

#### **Full Deployment**

```bash
# Deploy everything with Docker
just deploy

# Deploy with domain
just deploy fantasyfootball.com

# Deploy manually (systemd)
just deploy-manual

# Deploy specific environments
just deploy-dev    # Development only
just deploy-prod   # Production only
```

#### **SSL & Domain Setup**

```bash
# Set up domain configuration
just setup-domain fantasyfootball.com

# Set up SSL certificates
just ssl fantasyfootball.com

# Renew SSL certificates
just ssl-renew
```

### **📊 Monitoring Commands**

#### **Status & Health**

```bash
# Check service status
just status

# Run comprehensive health check
just health

# Show system information
just info

# Show environment URLs
just urls
```

#### **Logs**

```bash
# Watch all logs in real-time
just logs

# Watch specific service logs
just logs backend
just logs frontend
just logs nginx

# Show recent logs
just logs-recent
just logs-recent backend
```

### **🗄️ Database Commands**

#### **Connect to Databases**

```bash
# Production database
just db-prod

# Development database
just db-dev
```

#### **Backup & Restore**

```bash
# Backup all databases
just backup

# Backup specific databases
just backup-prod
just backup-dev

# Restore databases
just restore-prod backup_prod_20241201_120000.sql
just restore-dev backup_dev_20241201_120000.sql
```

### **🔧 Maintenance Commands**

#### **Service Management**

```bash
# Start all services
just start

# Stop all services
just stop

# Restart all services
just restart

# Restart specific service
just restart-service backend
```

#### **Updates**

```bash
# Update everything
just update

# Update specific service
just update-service backend
```

#### **Cleanup**

```bash
# Clean up Docker system
just cleanup

# Remove all containers and data (⚠️ destructive)
just clean
```

### **🧪 Development Commands**

#### **Building**

```bash
# Build backend
just build-backend

# Build frontend
just build-frontend
```

#### **Testing**

```bash
# Run all tests
just test

# Run specific tests
just test-backend
just test-frontend
```

#### **Development Mode**

```bash
# Start development mode with hot reload
just dev
```

### **⚙️ Utility Commands**

#### **Information**

```bash
# Show system info
just info

# Show configuration
just config

# Show URLs
just urls
```

## 🎯 Common Workflows

### **Initial Setup**

```bash
# 1. Deploy everything
just deploy

# 2. Check status
just status

# 3. Set up domain (if you have one)
just setup-domain yourdomain.com

# 4. Set up SSL (after DNS is configured)
just ssl yourdomain.com
```

### **Daily Operations**

```bash
# Check everything is working
just health

# View recent logs
just logs-recent

# Check URLs
just urls
```

### **Updates**

```bash
# Update everything
just update

# Check status after update
just status
```

### **Troubleshooting**

```bash
# Check health
just health

# View logs
just logs

# Restart services
just restart

# Check configuration
just config
```

### **Backup & Recovery**

```bash
# Create backup
just backup

# Restore from backup
just restore-prod backup_prod_20241201_120000.sql
```

## 🔄 CI/CD Integration

The GitHub Actions workflow uses justfile commands:

### **Development Deployment**

```yaml
- name: Deploy to development
  run: just ci-deploy-dev
```

### **Production Deployment**

```yaml
- name: Deploy to production
  run: just ci-deploy-prod
```

## 🛠️ Customization

### **Adding New Commands**

To add a new command, edit the `justfile`:

```makefile
# Your new command
my-command:
    echo "This is my custom command"
    # Add your commands here
```

### **Parameters**

Commands can accept parameters:

```bash
# Command with parameter
just ssl mydomain.com

# Command with optional parameter
just logs backend
```

### **Dependencies**

Commands can depend on other commands:

```makefile
deploy-and-test: deploy
    just test
```

## 🎨 Tips & Best Practices

### **1. Use Help**

```bash
# See all commands
just --list

# See command details
just --show deploy
```

### **2. Use Parameters**

```bash
# Instead of hardcoding values
just deploy fantasyfootball.com
just logs backend
just update-service frontend
```

### **3. Chain Commands**

```bash
# Run multiple commands
just deploy && just health
```

### **4. Use Aliases**

You can create aliases in your shell:

```bash
# Add to ~/.bashrc or ~/.zshrc
alias deploy='just deploy'
alias status='just status'
alias logs='just logs'
```

## 🚨 Troubleshooting

### **Common Issues**

1. **Command not found**

   ```bash
   # Install just
   curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash
   ```

2. **Permission denied**

   ```bash
   # Make sure you're in the right directory
   cd /path/to/fantasy-football-draft-board-v2
   ```

3. **Docker not running**
   ```bash
   # Start Docker
   sudo systemctl start docker
   ```

### **Getting Help**

```bash
# Show all commands
just --list

# Show command help
just --show <command>

# Show this guide
cat JUSTFILE_GUIDE.md
```

## 🎉 Benefits Over Multiple Scripts

### **Before (Multiple Scripts):**

- 8+ shell scripts to remember
- Different syntax and behavior
- Hard to maintain
- Confusing which script to use

### **After (Justfile):**

- **One file** with all commands
- **Consistent syntax** and behavior
- **Built-in help** (`just --list`)
- **Easy to maintain** and extend
- **Clear organization** by category

Your deployment process is now much cleaner and more maintainable!
