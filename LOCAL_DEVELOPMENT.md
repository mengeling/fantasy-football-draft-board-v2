# Local Development Setup

This guide will help you set up the Fantasy Football Draft Board for local development using Nix.

## Prerequisites

### 1. Install Nix

```bash
curl -L https://nixos.org/nix/install | sh
source ~/.bashrc  # or ~/.zshrc
```

### 2. Enable Flakes (if not already enabled)

Create or edit `~/.config/nix/nix.conf`:

```bash
mkdir -p ~/.config/nix
echo "experimental-features = nix-command flakes" >> ~/.config/nix/nix.conf
```

## Local Development Setup

### 1. Clone the Repository

```bash
git clone <your-repo-url>
cd fantasy-football-draft-board-v2
```

### 2. Enter the Nix Development Environment

```bash
nix develop
```

This will give you access to all the tools defined in `flake.nix`:

- Rust toolchain
- Node.js
- PostgreSQL
- Docker & Docker Compose
- Terraform
- AWS CLI
- And more...

### 3. Set Up the Database

Start PostgreSQL:

```bash
# In the nix develop shell
pg_ctl -D /nix/store/*/postgresql/data -l logfile start
```

Create the database:

```bash
createdb ffball
```

Set up the schema:

```bash
cd backend
PGPASSWORD=ffball psql -U ffball -d ffball -f "src/database/setup_db.sql"
```

### 4. Start the Application

#### Option A: Using Docker Compose (Recommended)

```bash
# Build and start all services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

#### Option B: Using the Deploy Script

```bash
# Deploy locally using the same script as production
./deploy/scripts/deploy.sh
```

#### Option C: Manual Development

**Backend:**

```bash
cd backend
cargo watch -x run
```

**Frontend:**

```bash
cd frontend
npm run dev
```

### 5. Access the Application

- **Frontend**: http://localhost:5173 (Vite dev server)
- **Backend**: http://localhost:8080
- **Database**: postgres://ffball:ffball@localhost:5432/ffball

## Development Workflow

### 1. Making Changes

1. Enter the Nix environment: `nix develop`
2. Make your changes to the code
3. For backend changes: `cargo watch -x run` (auto-reloads)
4. For frontend changes: `npm run dev` (auto-reloads)

### 2. Testing

**Backend tests:**

```bash
cd backend
cargo test
```

**Frontend tests:**

```bash
cd frontend
npm test
```

### 3. Database Schema Changes

To modify the database schema, edit `backend/src/database/setup_db.sql` and re-run the setup:

```bash
cd backend
PGPASSWORD=ffball psql -U ffball -d ffball -f "src/database/setup_db.sql"
```

**Note**: This project uses a single SQL file for database setup rather than migrations.

### 4. Scrape Data to Populate DB

Start the backend server and trigger the scraping endpoint:

```bash
cargo run
curl -X POST http://localhost:8080/fantasy-data/update
```

## Useful Commands

### Nix Commands

```bash
# Enter development environment
nix develop

# Build specific packages
nix build .#backend
nix build .#frontend

# Run specific tools
nix run .#backend
```

### Docker Commands

```bash
# View running containers
docker-compose ps

# View logs
docker-compose logs -f backend
docker-compose logs -f frontend

# Rebuild specific service
docker-compose build backend
docker-compose up -d backend

# Clean up
docker-compose down
docker system prune -f
```

### Database Commands

```bash
# Connect to database
psql postgres://ffball:ffball@localhost:5432/ffball

# Reset database
dropdb ffball
createdb ffball
PGPASSWORD=ffball psql -U ffball -d ffball -f "backend/src/database/setup_db.sql"
```

## Troubleshooting

### Common Issues

**1. Nix not found:**

```bash
# Restart terminal or source profile
source ~/.bashrc  # or ~/.zshrc
```

**2. Database connection issues:**

```bash
# Check if PostgreSQL is running
pg_ctl status

# Start PostgreSQL if needed
pg_ctl start
```

**3. Docker issues:**

```bash
# Check Docker is running
docker ps

# Restart Docker if needed
sudo systemctl restart docker
```

**4. Port conflicts:**

```bash
# Check what's using a port
lsof -i :8080
lsof -i :5173
lsof -i :5432
```

### Getting Help

- **Nix issues**: Check [Nix documentation](https://nixos.org/guides/)
- **Docker issues**: Check [Docker documentation](https://docs.docker.com/)
- **Rust issues**: Check [Rust documentation](https://doc.rust-lang.org/)
- **Svelte issues**: Check [Svelte documentation](https://svelte.dev/docs)

## Environment Variables

Create a `.env` file in the project root:

```bash
# Database
DATABASE_URL=postgres://ffball:ffball@localhost:5432/ffball

# Backend
RUST_LOG=info
RUST_BACKTRACE=1

# Frontend
VITE_API_URL=http://localhost:8080
```

## Next Steps

Once you have the local development environment set up:

1. **Explore the codebase**: Check out the backend and frontend directories
2. **Run the tests**: Ensure everything is working correctly
3. **Make changes**: Start developing new features
4. **Deploy locally**: Test the deployment script locally
5. **Deploy to EC2**: Use the GitHub Actions workflow for production deployment

Happy coding! 🚀
