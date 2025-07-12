# Nix Package Management Guide

This guide explains how to use Nix for reproducible builds and dependency management in the Fantasy Football Draft Board project.

## What is Nix?

Nix is a purely functional package manager that provides:

- **Reproducible builds**: Same inputs always produce the same outputs
- **Isolated environments**: Dependencies don't conflict with system packages
- **Multi-language support**: Works with Rust, Node.js, Python, and more
- **Declarative configuration**: All dependencies defined in configuration files

## Quick Start

### Prerequisites

1. **Install Nix** (if not already installed):

   ```bash
   # On macOS/Linux
   curl --proto '=https' --tlsv1.2 -sSf https://install.determinate.systems/nix | sh -s -- install

   # Or using the official installer
   sh <(curl -L https://nixos.org/nix/install) --daemon
   ```

2. **Enable flakes** (if using flakes):
   ```bash
   echo "experimental-features = nix-command flakes" >> ~/.config/nix/nix.conf
   ```

### Getting Started

1. **Enter the development environment**:

   ```bash
   # Using traditional Nix (shell.nix)
   nix-shell

   # Using flakes (flake.nix)
   nix develop
   ```

2. **Start development**:
   ```bash
   # All tools are now available
   just dev-backend    # Start backend server
   just dev-frontend   # Start frontend server
   just test           # Run all tests
   ```

## Configuration Files

### `flake.nix`

The main Nix flake configuration that defines:

- **Inputs**: Nixpkgs, Rust overlay, flake-utils
- **Outputs**: Development shells, packages, apps
- **Dependencies**: All required tools and libraries

### `shell.nix`

Traditional Nix shell configuration for backward compatibility.

### `backend/rust-toolchain.toml`

Specifies the Rust toolchain version and components.

### `frontend/.nvmrc`

Specifies the Node.js version for the frontend.

## Available Commands

### Development Environment

```bash
just nix-shell      # Enter Nix development shell
just nix-flake      # Enter Nix development shell (flakes)
just nix-prod       # Enter Nix production environment
```

### Building with Nix

```bash
just nix-build-backend    # Build backend using Nix
just nix-build-frontend   # Build frontend using Nix
just nix-build-all        # Build all packages
```

### Docker with Nix

```bash
just nix-docker-backend   # Build backend Docker image
just nix-docker-frontend  # Build frontend Docker image
```

### Maintenance

```bash
just nix-update           # Update Nix flake inputs
just nix-clean            # Clean Nix store
```

## Development Workflow

### 1. Daily Development

```bash
# Enter development environment
nix develop

# Start development servers
just dev-backend & just dev-frontend

# Run tests
just test

# Format and lint code
just format
just lint
```

### 2. Building for Production

```bash
# Build with Nix
just nix-build-all

# Or build traditional way
just build

# Deploy with Nix builds
just deploy-nix
```

### 3. Testing Changes

```bash
# Run all tests
just test

# Run specific tests
just test-backend
just test-frontend

# Run CI tests
just ci-test
```

## Package Management

### Backend Dependencies (Rust)

- **Rust toolchain**: Latest stable with rust-analyzer
- **System libraries**: pkg-config, openssl, libpq, postgresql
- **Development tools**: cargo-watch, cargo-audit, cargo-tarpaulin

### Frontend Dependencies (Node.js)

- **Node.js**: Version 20.10.0
- **Package managers**: npm, pnpm
- **Build tools**: TypeScript, Vite, ESLint, Prettier
- **Svelte tools**: svelte-check, svelte-preprocess

### DevOps Tools

- **Containerization**: Docker, Docker Compose
- **Web server**: Nginx
- **SSL/TLS**: Certbot
- **Monitoring**: htop, iotop, nethogs
- **Utilities**: curl, jq, git, ripgrep, fzf

## CI/CD Integration

The GitHub Actions workflow uses Nix for:

- **Reproducible builds**: Same environment locally and in CI
- **Caching**: Nix cache speeds up builds
- **Dependency management**: All tools available without installation

### CI/CD Commands

```bash
just ci-build    # Build for CI/CD
just ci-test     # Test for CI/CD
just ci-deploy   # Deploy for CI/CD
```

## Troubleshooting

### Common Issues

1. **Nix not found**:

   ```bash
   # Install Nix
   curl --proto '=https' --tlsv1.2 -sSf https://install.determinate.systems/nix | sh -s -- install
   ```

2. **Flakes not enabled**:

   ```bash
   # Enable flakes
   echo "experimental-features = nix-command flakes" >> ~/.config/nix/nix.conf
   ```

3. **Build failures**:

   ```bash
   # Clean and rebuild
   just nix-clean
   nix develop --command just build
   ```

4. **Missing dependencies**:
   ```bash
   # Update flake inputs
   just nix-update
   nix develop
   ```

### Debugging

1. **Check Nix store**:

   ```bash
   nix store info
   nix store gc
   ```

2. **View build logs**:

   ```bash
   nix build --log-format bar-with-logs
   ```

3. **Inspect shell**:
   ```bash
   nix develop --command bash
   echo $PATH
   which cargo
   which node
   ```

## Benefits of Using Nix

### For Developers

- **Consistent environment**: Same tools and versions across all machines
- **Easy onboarding**: New developers just run `nix develop`
- **No conflicts**: Dependencies isolated from system packages
- **Reproducible builds**: Same results locally and in CI

### For Operations

- **Reliable deployments**: Same build artifacts every time
- **Easy rollbacks**: Previous versions are cached
- **Minimal dependencies**: Only required tools are included
- **Cross-platform**: Works on Linux, macOS, and Windows

### For CI/CD

- **Faster builds**: Nix cache reduces build times
- **Reliable caching**: Deterministic cache keys
- **Parallel builds**: Independent packages can build simultaneously
- **Artifact sharing**: Build artifacts can be shared between jobs

## Migration from Traditional Setup

### Before Nix

```bash
# Manual installation required
sudo apt install rustc cargo nodejs npm postgresql
npm install -g pnpm
cargo install cargo-watch
# ... many more manual steps
```

### After Nix

```bash
# Single command
nix develop
# All tools available immediately
```

## Best Practices

1. **Use flakes for new projects**: More modern and feature-rich
2. **Pin dependencies**: Use specific versions for reproducibility
3. **Keep shells minimal**: Only include necessary dependencies
4. **Use justfile**: Integrate Nix commands with project workflow
5. **Cache builds**: Enable Nix cache in CI/CD
6. **Document changes**: Update flake.nix when adding dependencies

## Resources

- [Nix Documentation](https://nixos.org/learn.html)
- [Nix Flakes](https://nixos.wiki/wiki/Flakes)
- [Nixpkgs Manual](https://nixos.org/manual/nixpkgs/stable/)
- [Rust with Nix](https://nixos.wiki/wiki/Rust)
- [Node.js with Nix](https://nixos.wiki/wiki/Node.js)

## Support

If you encounter issues with Nix:

1. Check the troubleshooting section above
2. Search the [Nix Discourse](https://discourse.nixos.org/)
3. Ask in the project's issue tracker
4. Consult the [NixOS Wiki](https://nixos.wiki/)
