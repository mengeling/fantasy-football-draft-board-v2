# Fantasy Football Draft Board

[Click here to use the draft board!](http://54.162.53.255/) Type "test" when prompted for your name if you want to play with the draft board without waiting for the data to be scraped.

This web application provides the same interactive fantasy football drafting experience as the official draft boards on ESPN, Yahoo, NFL.com, etc., but it uses consensus player rankings consolidated from 100+ experts.

## App Demo

![Demo](frontend/src/static/img/fantasy_football_recording.gif)

## App Screenshot

![App Screenshot](frontend/src/static/img/app_pic.png)

## App Setup

1. Clone the repository
2. Run `crontab -e` and paste this line with updated paths: `0 0 * * * RUST_LOG=info ./target/release/your_binary_name >> /path/to/logfile.log 2>&1`
3. Run `sudo apt-get install postgresql`
4. Run `sudo systemctl start postgresql`
5. Run `sudo -u postgres psql -c "CREATE USER ffball WITH SUPERUSER CREATEDB CREATEROLE LOGIN PASSWORD 'ffball';"`
6. Run `sudo -u postgres createdb -O ffball ffball`
7. Run `sudo nano /etc/postgresql/17/main/pg_hba.conf`
8. Update the following lines:

   ```bash
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

9. Run `sudo systemctl restart postgresql`
   i. Run `PGPASSWORD=ffball psql -U ffball -d ffball` to access database
10. Run `./src/scripts/setup_db.sql` from the `backend` directory to perform initial DB setup
11. Run `cargo run` from the `backend` directory
12. Run `npm run dev` from the `frontend` directory
13. Open your browser and navigate to `http://localhost:3000`

## Deployment Guide

### Prerequisites

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

3. Set up GitHub SSH key:

```bash
# Generate SSH key (press enter to accept defaults)
ssh-keygen

# Start ssh-agent and add key
eval "$(ssh-agent -s)"
ssh-add ~/.ssh/id_ed25519

# Display public key to copy to GitHub
cat ~/.ssh/id_ed25519.pub
```

Then add this key to your GitHub account:

- Go to GitHub → Settings → SSH and GPG keys
- Click "New SSH key"
- Paste the contents of your public key and call it "ffball"
- Test the connection: `ssh -T git@github.com`

4. Set up PostgreSQL:

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

5. Clone the repository:

```bash
git clone git@github.com:mengeling/fantasy-football-draft-board-v2.git
cd fantasy-football-draft-board-v2
```

### Backend Setup

1. Set up database:

```bash
./src/scripts/setup_db.sh
```

2. Build the backend:

```bash
cd backend
cargo build --release
```

3. Create a systemd service for the backend:

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
Environment="RUST_LOG=info"
WorkingDirectory=/home/ubuntu/fantasy-football-draft-board-v2/backend
ExecStart=/home/ubuntu/fantasy-football-draft-board-v2/backend/target/release/ffball-app
Restart=always

[Install]
WantedBy=multi-user.target
```

### Frontend Setup

1. Install frontend dependencies and build the frontend:

```bash
cd ../frontend
npm install
npm run build
```

This will create a `build` directory containing your static files.

### Nginx Configuration

1. Create Nginx configuration:

```bash
sudo nano /etc/nginx/sites-available/fantasy-app
```

Add the following configuration:

```nginx
server {
    listen 80;
    server_name your-domain.com;  # Replace with your domain or EC2 public IP

    # Frontend - serve static files
    root /home/ubuntu/fantasy-football-draft-board-v2/frontend/build;
    index index.html;

    # This ensures your SvelteKit app's client-side routing works
    location / {
        try_files $uri $uri/ /index.html;
    }

    # Backend API
    location /api/ {
        proxy_pass http://127.0.0.1:8080/;  # Backend port
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
sudo ln -s /etc/nginx/sites-available/fantasy-app /etc/nginx/sites-enabled/
sudo rm /etc/nginx/sites-enabled/default  # Remove default config if it exists
sudo nginx -t
sudo systemctl restart nginx
```

3. Start the backend service:

```bash
sudo systemctl start ffball
sudo systemctl enable ffball
```

### Monitoring

- Backend logs: `sudo journalctl -u ffball -f`
- Nginx logs: `sudo tail -f /var/log/nginx/error.log`

### SSL Setup (Optional)

To enable HTTPS:

```bash
sudo apt install -y certbot python3-certbot-nginx
sudo certbot --nginx -d your-domain.com
```

### Cron Job Setup

Set up the cron job for data updates:

```bash
crontab -e
```

Add:

```
0 0 * * * RUST_LOG=info /home/ubuntu/fantasy-football-draft-board-v2/backend/target/release/ffball-app >> /home/ubuntu/ffball.log 2>&1
```
