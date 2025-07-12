# Fantasy Football Draft Board v2

A modern fantasy football draft board application with a Rust backend and Svelte frontend, featuring real-time updates, player rankings, and draft management.

## 🚀 Quick Start

### Prerequisites

- **Nix** (recommended) or manual installation of:
  - Rust (latest stable)
  - Node.js 20+
  - PostgreSQL
  - Docker & Docker Compose

### Development Setup

1. **Clone the repository**:

   ```bash
   git clone https://github.com/yourusername/fantasy-football-draft-board-v2.git
   cd fantasy-football-draft-board-v2
   ```

2. **Enter development environment**:

   ```bash
   # Using Nix (recommended)
   nix develop

   # Or using traditional Nix
   nix-shell

   # Or manual setup (see manual setup section below)
   ```

3. **Start development servers**:

   ```bash
   just dev-backend    # Start backend server
   just dev-frontend   # Start frontend server
   ```

4. **Access the application**:
   - Frontend: http://localhost:5173
   - Backend API: http://localhost:8000

## 📦 Package Management with Nix

This project uses **Nix** for reproducible builds and dependency management. See [NIX_GUIDE.md](NIX_GUIDE.md) for detailed information.

### Key Benefits

- **Reproducible environments**: Same tools and versions across all machines
- **Isolated dependencies**: No conflicts with system packages
- **Easy onboarding**: New developers just run `nix develop`
- **CI/CD integration**: Same environment locally and in CI

### Quick Nix Commands

```bash
nix develop              # Enter development environment
just nix-build-all       # Build all packages
just nix-update          # Update dependencies
just nix-clean           # Clean Nix store
```

## 🛠️ Manual Setup (Alternative)

If you prefer not to use Nix:

1. **Install Rust**:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Install Node.js**:

   ```bash
   # Using nvm (recommended)
   curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
   nvm install 20
   nvm use 20
   ```

3. **Install PostgreSQL**:

   ```bash
   # Ubuntu/Debian
   sudo apt update
   sudo apt install postgresql postgresql-contrib

   # macOS
   brew install postgresql
   ```

4. **Install Docker**:
   ```bash
   # Follow instructions at https://docs.docker.com/get-docker/
   ```

## 🏗️ Project Structure

```
fantasy-football-draft-board-v2/
├── backend/                 # Rust backend API
│   ├── src/
│   ├── Cargo.toml
│   └── rust-toolchain.toml  # Rust toolchain specification
├── frontend/                # Svelte frontend
│   ├── src/
│   ├── package.json
│   └── .nvmrc              # Node.js version specification
├── scripts/                 # Deployment and utility scripts
├── docker-compose.yml       # Docker services configuration
├── justfile                 # Task runner (replaces multiple scripts)
├── flake.nix               # Nix flake configuration
├── shell.nix               # Traditional Nix shell configuration
└── README.md
```

## 🎯 Available Commands

This project uses **justfile** as a task runner. Run `just --list` to see all available commands.

### Development

```bash
just dev-backend          # Start backend development server
just dev-frontend         # Start frontend development server
just dev                  # Start both servers
just test                 # Run all tests
just format               # Format code
just lint                 # Lint code
```

### Building

```bash
just build                # Build both backend and frontend
just build-backend        # Build backend only
just build-frontend       # Build frontend only
just nix-build-all        # Build using Nix
```

### Database

```bash
just db-setup             # Setup development database
just db-reset             # Reset development database
just db-migrate-dev       # Run database migrations
```

### Docker

```bash
just docker-build         # Build Docker images
just docker-up            # Start Docker services
just docker-down          # Stop Docker services
just docker-logs          # View Docker logs
```

### Deployment

```bash
just deploy               # Deploy to production
just deploy-nix           # Deploy using Nix builds
just health-check         # Check system health
```

### Nix Commands

```bash
just nix-shell            # Enter Nix development shell
just nix-flake            # Enter Nix development shell (flakes)
just nix-prod             # Enter Nix production environment
just nix-update           # Update Nix dependencies
just nix-clean            # Clean Nix store
```

## 🐳 Docker Deployment

The project includes Docker Compose configuration for easy deployment:

```bash
# Build and start all services
just docker-build
just docker-up

# View logs
just docker-logs

# Stop services
just docker-down
```

### Services

- **Backend**: Rust API server
- **Frontend**: Svelte application served by Nginx
- **Database**: PostgreSQL with separate prod/dev databases
- **Nginx**: Reverse proxy with SSL support
- **Certbot**: SSL certificate management

## 🔧 Configuration

### Environment Variables

Create `.env` files in the backend and frontend directories:

**Backend (.env)**:

```env
DATABASE_URL=postgresql://ffball:ffball@localhost:5432/ffball_dev
RUST_LOG=debug
RUST_BACKTRACE=1
```

**Frontend (.env)**:

```env
VITE_API_URL=http://localhost:8000
VITE_APP_TITLE=Fantasy Football Draft Board
```

### Database Setup

1. **Create databases**:

   ```bash
   createdb -U postgres ffball_prod
   createdb -U postgres ffball_dev
   ```

2. **Run migrations**:
   ```bash
   just db-migrate-dev
   ```

## 🚀 Deployment

### Production Deployment

1. **Build and deploy**:

   ```bash
   just deploy
   ```

2. **Or deploy with Nix**:

   ```bash
   just deploy-nix
   ```

3. **Check health**:
   ```bash
   just health-check
   ```

### CI/CD

The project includes GitHub Actions workflows for:

- **Testing**: Run tests on pull requests
- **Building**: Build artifacts for deployment
- **Deployment**: Deploy to production on main branch

## 📚 Documentation

- [NIX_GUIDE.md](NIX_GUIDE.md) - Comprehensive Nix usage guide
- [JUSTFILE_GUIDE.md](JUSTFILE_GUIDE.md) - Task runner documentation
- [DOCKER_DEPLOYMENT.md](DOCKER_DEPLOYMENT.md) - Docker deployment guide
- [DOMAIN_SETUP.md](DOMAIN_SETUP.md) - Domain and SSL setup guide

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Enter development environment: `nix develop`
4. Make your changes and run tests: `just test`
5. Commit your changes: `git commit -m 'Add amazing feature'`
6. Push to the branch: `git push origin feature/amazing-feature`
7. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

- **Issues**: [GitHub Issues](https://github.com/yourusername/fantasy-football-draft-board-v2/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/fantasy-football-draft-board-v2/discussions)
- **Documentation**: Check the guides in the `docs/` directory

## 🏆 Features

- **Real-time draft board** with live updates
- **Player rankings** from multiple sources
- **Draft management** with customizable settings
- **Responsive design** for mobile and desktop
- **User authentication** and draft rooms
- **Statistics and analytics** for draft decisions
- **Export functionality** for draft results
- **Multi-environment support** (dev/prod)
- **Docker containerization** for easy deployment
- **Nix package management** for reproducible builds
- **CI/CD pipeline** with automated testing and deployment
