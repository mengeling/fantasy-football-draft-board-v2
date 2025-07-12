{
  description = "Fantasy Football Draft Board - Backend (Rust) + Frontend (Svelte)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        # Rust toolchain
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [ "x86_64-unknown-linux-gnu" ];
        };

        # Backend dependencies
        backendDeps = with pkgs; [
          # Rust and Cargo
          rustToolchain
          
          # System dependencies for Rust crates
          pkg-config
          openssl
          libpq
          postgresql
          
          # Development tools
          cargo-watch
          cargo-audit
          cargo-tarpaulin
          
          # Database tools
          postgresql
          
          # Monitoring and debugging
          htop
          lsof
          netcat
        ];

        # Frontend dependencies
        frontendDeps = with pkgs; [
          # Node.js and npm
          nodejs_20
          nodePackages.npm
          nodePackages.pnpm
          
          # Build tools
          nodePackages.typescript
          nodePackages.eslint
          nodePackages.prettier
          
          # Development tools
          nodePackages.nodemon
          nodePackages.vite
          
          # Svelte tools
          nodePackages.svelte-check
          nodePackages.svelte-preprocess
        ];

        # DevOps and deployment tools
        devOpsDeps = with pkgs; [
          # Docker
          docker
          docker-compose
          
          # Nginx
          nginx
          
          # SSL/TLS
          certbot
          
          # Monitoring
          htop
          iotop
          nethogs
          
          # Network tools
          curl
          wget
          jq
          
          # Git and version control
          git
          git-lfs
          
          # Shell utilities
          bash
          zsh
          tmux
          vim
          
          # File system tools
          tree
          fd
          ripgrep
          fzf
          
          # Process management
          procps
          psmisc
        ];

        # Database tools
        dbDeps = with pkgs; [
          postgresql
          postgresqlPackages.postgis
          
          # Database clients
          postgresqlPackages.pgadmin
          
          # Backup tools
          pg_dump
          pg_restore
        ];

      in {
        # Development shell
        devShells.default = pkgs.mkShell {
          buildInputs = backendDeps ++ frontendDeps ++ devOpsDeps ++ dbDeps;
          
          shellHook = ''
            echo "🚀 Fantasy Football Draft Board Development Environment"
            echo "=================================================="
            echo ""
            echo "Available tools:"
            echo "  Backend (Rust): cargo, rustc, rust-analyzer"
            echo "  Frontend (Svelte): node, npm, pnpm, vite"
            echo "  DevOps: docker, nginx, certbot"
            echo "  Database: psql, pg_dump"
            echo "  Utilities: git, curl, jq, ripgrep"
            echo ""
            echo "Quick commands:"
            echo "  just dev-backend    - Start backend development server"
            echo "  just dev-frontend   - Start frontend development server"
            echo "  just test           - Run all tests"
            echo "  just build          - Build both backend and frontend"
            echo "  just deploy         - Deploy to production"
            echo ""
            echo "Database:"
            echo "  just db-setup       - Setup development database"
            echo "  just db-reset       - Reset development database"
            echo ""
            echo "Docker:"
            echo "  just docker-build   - Build all containers"
            echo "  just docker-up      - Start all services"
            echo "  just docker-down    - Stop all services"
            echo ""
          '';
          
          # Environment variables
          RUST_BACKTRACE = "1";
          RUST_LOG = "debug";
          DATABASE_URL = "postgresql://ffball:ffball@localhost:5432/ffball_dev";
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };

        # Production shell
        devShells.production = pkgs.mkShell {
          buildInputs = backendDeps ++ devOpsDeps ++ dbDeps;
          
          shellHook = ''
            echo "🏭 Fantasy Football Draft Board Production Environment"
            echo "==================================================="
            echo ""
            echo "Production tools available:"
            echo "  Backend: cargo, rustc"
            echo "  DevOps: docker, nginx, certbot"
            echo "  Database: psql, pg_dump"
            echo "  Monitoring: htop, iotop, nethogs"
            echo ""
            echo "Production commands:"
            echo "  just build-prod      - Build production binaries"
            echo "  just deploy-prod     - Deploy to production"
            echo "  just health-check    - Check system health"
            echo "  just backup          - Backup database"
            echo ""
          '';
          
          # Production environment variables
          RUST_BACKTRACE = "0";
          RUST_LOG = "info";
          DATABASE_URL = "postgresql://ffball:ffball@localhost:5432/ffball_prod";
        };

        # Backend package
        packages.backend = pkgs.rustPlatform.buildRustPackage {
          pname = "fantasy-football-backend";
          version = "1.0.0";
          
          src = ./backend;
          
          cargoLock.lockFile = ./backend/Cargo.lock;
          
          buildInputs = with pkgs; [
            pkg-config
            openssl
            libpq
          ];
          
          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
          
          # Environment variables for build
          OPENSSL_DIR = pkgs.openssl.dev;
          OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
          PQ_LIB_DIR = "${pkgs.libpq.lib}/lib";
          
          # Build flags
          RUSTFLAGS = "-C target-cpu=native";
          
          # Install binary
          installPhase = ''
            mkdir -p $out/bin
            cp target/release/fantasy_football_backend $out/bin/
          '';
        };

        # Frontend package
        packages.frontend = pkgs.stdenv.mkDerivation {
          pname = "fantasy-football-frontend";
          version = "1.0.0";
          
          src = ./frontend;
          
          buildInputs = with pkgs; [
            nodejs_20
            nodePackages.npm
            nodePackages.pnpm
          ];
          
          buildPhase = ''
            cd frontend
            pnpm install
            pnpm run build
          '';
          
          installPhase = ''
            mkdir -p $out/share/fantasy-football-frontend
            cp -r frontend/build/* $out/share/fantasy-football-frontend/
          '';
        };

        # Docker image for backend
        packages.backend-docker = pkgs.dockerTools.buildImage {
          name = "fantasy-football-backend";
          tag = "latest";
          
          contents = [ self.packages.${system}.backend ];
          
          config = {
            Cmd = [ "/bin/fantasy_football_backend" ];
            ExposedPorts = {
              "8000/tcp" = {};
            };
            Env = [
              "RUST_LOG=info"
              "DATABASE_URL=postgresql://ffball:ffball@db:5432/ffball_prod"
            ];
          };
        };

        # Docker image for frontend
        packages.frontend-docker = pkgs.dockerTools.buildImage {
          name = "fantasy-football-frontend";
          tag = "latest";
          
          contents = [ self.packages.${system}.frontend ];
          
          config = {
            Cmd = [ "nginx" "-g" "daemon off;" ];
            ExposedPorts = {
              "80/tcp" = {};
            };
          };
        };

        # Default package
        packages.default = self.packages.${system}.backend;

        # Apps
        apps = {
          # Development servers
          dev-backend = {
            type = "app";
            program = toString (pkgs.writeShellScript "dev-backend" ''
              cd backend
              cargo run
            '');
          };
          
          dev-frontend = {
            type = "app";
            program = toString (pkgs.writeShellScript "dev-frontend" ''
              cd frontend
              pnpm run dev
            '');
          };
          
          # Build commands
          build-backend = {
            type = "app";
            program = toString (pkgs.writeShellScript "build-backend" ''
              cd backend
              cargo build --release
            '');
          };
          
          build-frontend = {
            type = "app";
            program = toString (pkgs.writeShellScript "build-frontend" ''
              cd frontend
              pnpm run build
            '');
          };
          
          # Test commands
          test-backend = {
            type = "app";
            program = toString (pkgs.writeShellScript "test-backend" ''
              cd backend
              cargo test
            '');
          };
          
          test-frontend = {
            type = "app";
            program = toString (pkgs.writeShellScript "test-frontend" ''
              cd frontend
              pnpm run test
            '');
          };
        };
      }
    );
} 