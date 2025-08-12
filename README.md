# Fantasy Football Draft Board

[Click here to use the draft board!](http://100.29.78.245/) Create a username and choose your scoring settings to start using the board.

This web application provides the same interactive fantasy football drafting experience as the official draft boards on ESPN, Yahoo, NFL.com, etc., but it uses consensus player rankings consolidated from 100+ experts.

## App Demo

![Demo](frontend/static/img/fantasy_football_recording.gif)

## App Screenshot

![App Screenshot](frontend/static/img/app_pic.png)

## Deployment Guide

### Automated Deployment (Recommended)

For automated deployment using Terraform and GitHub Actions:

1. **Setup Prerequisites**:

   ```bash
   # Install AWS CLI and Terraform
   brew install awscli terraform  # macOS
   # or follow official installation guides for your OS
   ```

2. **Configure AWS Credentials**:

   ```bash
   aws configure
   ```

3. **Setup Terraform Backend**:

   ```bash
   cd deploy/scripts
   ./setup-terraform-backend.sh
   # Run with --help for options and examples
   ```

4. **Generate SSH Keys**:

   ```bash
   ./setup-secrets.sh
   ```

5. **Deploy Infrastructure**:

   ```bash
   cd deploy/terraform
   terraform init
   terraform plan
   terraform apply
   ```

6. **Deploy Application**:
   - Push to main branch for automatic deployment via GitHub Actions
   - Or manually: `just deploy`

**Benefits**: Automated, reproducible, version-controlled deployments with proper state management.

## Local Development

For local development setup, see [LOCAL_DEVELOPMENT.md](LOCAL_DEVELOPMENT.md) for detailed instructions on setting up Nix and running the application locally.

## Database Management

This project uses SQLx for database operations, which requires special handling for builds and deployments.

### SQLx Query Metadata

SQLx uses pre-compiled query metadata (`.sqlx` files) to enable offline compilation. This is essential for:

- **Nix builds**: Allows building without a live database connection
- **CI/CD**: Enables reproducible builds in isolated environments
- **Deployment**: Faster EC2 deployment without compilation

### When to Update SQLx Metadata

You need to run `cargo sqlx prepare` whenever you:

1. **Add new SQL queries** using SQLx macros (`sqlx::query!`, `sqlx::query_as!`, etc.)
2. **Modify existing queries** (change SQL syntax, add/remove columns)
3. **Change database schema** (add/remove tables, modify columns)

### Updating SQLx Metadata

```bash
# 1. Ensure you have a local database running
docker-compose up -d postgres

# 2. Set the database URL
export DATABASE_URL="postgres://ffball:ffball@localhost:5432/ffball"

# 3. Generate new metadata
cd backend
cargo sqlx prepare

# 4. Commit the updated .sqlx directory
git add .sqlx
git commit -m "Update SQLx metadata for new queries"
git push
```

### Future State: Database Migrations

When you're ready to implement database migrations in this app:

1. **Use `sqlx-cli`** for migration management
2. **Run migrations** before `cargo sqlx prepare`
3. **Update metadata** after schema changes
4. **Test locally** before deploying

Example workflow:

```bash
# Create a new migration
cargo sqlx migrate add add_new_table

# Apply migrations
cargo sqlx migrate run

# Update query metadata
cargo sqlx prepare

# Commit changes
git add migrations/ .sqlx
git commit -m "Add new table with migration"
```

**Note**: Always run `cargo sqlx prepare` after any database schema changes to keep the metadata in sync.

## Development Commands

This project uses [Just](https://github.com/casey/just) as the command runner instead of Make. Just provides better syntax, parameter support, and cross-platform compatibility.

### Quick Start

```bash
# Show all available commands
just

# Start development environment
just dev

# Build all components
just build

# Run tests
just test

# Deploy to production
just deploy
```

### Installing Just

If you don't have Just installed:

```bash
# On macOS with Homebrew
brew install just

# On macOS with Nix
nix-env -iA nixpkgs.just

# On other systems
curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash
```

### Manual Deployment

For manual deployment on an existing EC2 instance:

#### Prerequisites

- An AWS Ubuntu EC2 instance
- A domain name (optional)

### Initial Server Setup

1. SSH into your EC2 instance:

```bash
ssh -i ~/.ssh/id_rsa ubuntu@[YOUR-EC2-PUBLIC-IP]
```

2. Install required packages:

```bash
sudo apt update && sudo apt upgrade -y

# Install Node.js and npm
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs

# Install Rust (press enter to accept defaults)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install other dependencies
sudo apt install -y nginx postgresql build-essential pkg-config libssl-dev chromium-browser
```

3. Create an SSH key:

```bash
# Generate SSH key (press enter to accept defaults)
ssh-keygen

# Start ssh-agent and add key
eval "$(ssh-agent -s)"
ssh-add ~/.ssh/id_ed25519

# Display public key to copy to GitHub
cat ~/.ssh/id_ed25519.pub
```

4. Add SSH key to GitHub account:

- Go to GitHub → Settings → SSH and GPG keys
- Click "New SSH key"
- Paste the contents of your public key and call it "ffball"
- Test the connection: `ssh -T git@github.com`

5. Set up PostgreSQL:

```bash
sudo systemctl start postgresql
sudo -u postgres psql -c "CREATE USER ffball WITH SUPERUSER CREATEDB CREATEROLE LOGIN PASSWORD 'ffball';"
sudo -u postgres createdb -O ffball ffball

# Update config file (version number might differ)
sudo nano /etc/postgresql/16/main/pg_hba.conf
```

Paste the following config into the file:

```conf
# Database administrative login by Unix domain socket
local   all             postgres                                peer

# TYPE  DATABASE        USER            ADDRESS                 METHOD

# "local" is for Unix domain socket connections only
local   all             all                                     md5
# IPv4 local connections:
host    all             all             127.0.0.1/32            md5
# IPv6 local connections:
host    all             all             ::1/128                 md5
# Allow replication connections from localhost, by a user with the
# replication privilege.
local   replication     all                                     peer
host    replication     all             127.0.0.1/32            scram-sha-256
host    replication     all             ::1/128                 scram-sha-256
```

Restart postgres and confirm you can connect:

```bash
sudo systemctl restart postgresql
PGPASSWORD=ffball psql -U ffball -d ffball
```

6. Clone the repository:

```bash
git clone git@github.com:mengeling/fantasy-football-draft-board-v2.git
cd fantasy-football-draft-board-v2
```

### Backend Setup

1. Set up database and build the backend:

```bash
cd backend
./src/scripts/setup_db.sh
cargo build --release
```

2. Create a systemd service for the backend:

```bash
sudo nano /etc/systemd/system/ffball.service
```

Add the following content:

```ini
[Unit]
Description=Fantasy Football Backend
After=network.target postgresql.service

[Service]
Type=simple
User=ubuntu
Group=ubuntu
Environment="RUST_LOG=info"
Environment="RUST_BACKTRACE=1"
WorkingDirectory=/home/ubuntu/fantasy-football-draft-board-v2/backend
ExecStart=/home/ubuntu/fantasy-football-draft-board-v2/backend/target/release/backend
Restart=always
RestartSec=5
StandardOutput=append:/home/ubuntu/ffball.log
StandardError=append:/home/ubuntu/ffball.log

[Install]
WantedBy=multi-user.target
```

After updating the service file, run:

```bash
# Reload systemd to pick up changes
sudo systemctl daemon-reload

# Make sure the log file exists and has correct permissions
sudo touch /home/ubuntu/ffball.log
sudo chown ubuntu:ubuntu /home/ubuntu/ffball.log
```

### Frontend Setup

1. Install frontend dependencies and build the frontend:

```bash
cd ../frontend
npm install
npm run build

# Set correct permissions for Nginx
sudo chown -R www-data:www-data /home/ubuntu/fantasy-football-draft-board-v2/frontend/build
sudo chmod -R 755 /home/ubuntu/fantasy-football-draft-board-v2/frontend/build
sudo chmod 755 /home/ubuntu
sudo chmod 755 /home/ubuntu/fantasy-football-draft-board-v2
sudo chmod 755 /home/ubuntu/fantasy-football-draft-board-v2/frontend
```

### Nginx Configuration

1. Create Nginx configuration:

```bash
sudo nano /etc/nginx/sites-available/ffball-app
```

Add the following configuration:

```nginx
server {
    listen 80;
    server_name [YOUR_EC2_PUBLIC_IP];  # Replace with your EC2 public IP or domain

    # Frontend - serve static files
    root /home/ubuntu/fantasy-football-draft-board-v2/frontend/build;
    index index.html;

    location / {
        try_files $uri $uri/ /index.html;
    }

    # Backend API
    location /api/ {
        proxy_pass http://127.0.0.1:8080/;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
}
```

2. Enable the configuration:

```bash
# Create symlink to enable the site
sudo ln -s /etc/nginx/sites-available/ffball-app /etc/nginx/sites-enabled/

# Remove default config if it exists
sudo rm -f /etc/nginx/sites-enabled/default

# Test the configuration
sudo nginx -t

# Restart Nginx
sudo systemctl restart nginx
```

3. Start the backend service:

```bash
sudo systemctl start ffball
sudo systemctl enable ffball
```

### Cron Job Setup

Set up the cron job for daily data updates:

```bash
crontab -e
```

Add:

```bash
0 0 * * * echo "$(date): Starting fantasy data update" >> /home/ubuntu/ffball.log && curl -X POST http://127.0.0.1:8080/fantasy-data/update >> /home/ubuntu/ffball.log 2>&1
```

### SSL Setup (Optional)

To enable HTTPS:

```bash
sudo apt install -y certbot python3-certbot-nginx
sudo certbot --nginx -d your-domain.com
```

### Monitoring

- Backend logs: `sudo journalctl -u ffball`
- Nginx logs: `sudo tail /var/log/nginx/error.log`
