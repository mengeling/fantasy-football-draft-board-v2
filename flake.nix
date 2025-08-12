{
  description = "Fantasy Football Draft Board - Reproducible Development Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          system = "x86_64-linux";
          overlays = [ (import rust-overlay) ];
        };
        
        # Rust toolchain
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        # Node.js environment
        nodejs = pkgs.nodejs_20;
        
        # Database
        postgresql = pkgs.postgresql_16;
        
        # Development shell with Linux-compatible tools
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            nodejs_20
            postgresql
            cargo-watch
            chromium
          ];
          
          shellHook = ''
            echo "Fantasy Football Draft Board development shell"
            echo "All tools are Linux-compatible for Docker deployment"
          '';
        };

        # System dependencies (for deployment and basic operations)
        systemDeps = with pkgs; [
          pkg-config
          openssl
          libpq
          curl
          git
          docker
          docker-compose
          terraform
          awscli2
          certbot
          bash
          coreutils
          nginx
          just
          sqlx-cli
          chromium
        ];

        # Build inputs for Rust
        rustInputs = with pkgs; [
          pkg-config
          openssl
          libpq
          chromium
        ];

        # Backend build
        backend = pkgs.rustPlatform.buildRustPackage {
          name = "ffball-backend";
          src = ./backend;
          cargoLock = {
            lockFile = ./backend/Cargo.lock;
          };
          nativeBuildInputs = with pkgs; [
            pkg-config
            openssl
            libpq
            chromium
            rustfmt
          ];
          buildInputs = with pkgs; [
            pkg-config
            openssl
            libpq
            chromium
          ];
          
          # Set environment variables for compilation
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig:${pkgs.libpq.dev}/lib/pkgconfig";
          
          # Copy scripts
          postInstall = ''
            mkdir -p $out/scripts
            cp -r src/scripts/* $out/scripts/
            chmod +x $out/scripts/*.sh
          '';
        };

        # Frontend build
        frontend = pkgs.stdenv.mkDerivation {
          name = "ffball-frontend";
          src = ./frontend;
          nativeBuildInputs = [ pkgs.nodejs_20 ];
          buildPhase = ''
            export HOME=$(mktemp -d)
            export NODE_ENV=production
            export PATH="$PWD/node_modules/.bin:$PATH"
            
            echo "Installing npm dependencies..."
            # Install all dependencies including devDependencies (needed for vite)
            npm ci --no-audit --no-fund --prefer-offline --include=dev || npm install --no-audit --no-fund --prefer-offline --include=dev
            
            echo "Verifying node_modules installation..."
            ls -la node_modules/ || echo "node_modules directory not found"
            ls -la node_modules/.bin/ || echo "node_modules/.bin directory not found"
            
            # Ensure vite is available in PATH
            if [ ! -f "node_modules/.bin/vite" ]; then
              echo "Error: vite not found in node_modules/.bin"
              echo "Available binaries:"
              ls -la node_modules/.bin/ || echo "No binaries found"
              echo "npm list vite:"
              npm list vite || echo "vite not listed"
              exit 1
            fi
            
            echo "Building frontend..."
            npm run build
          '';
          installPhase = ''
            mkdir -p $out
            cp -r build/* $out/
          '';
        };

        # Docker image for backend
        backendImage = pkgs.dockerTools.buildImage {
          name = "ffball-backend";
          tag = "latest";
          copyToRoot = [ backend ];
          config = {
            Cmd = [ "${backend}/bin/backend" ];
            ExposedPorts = {
              "8080/tcp" = {};
            };
            Env = [
              "DATABASE_URL=postgres://ffball:ffball@postgres:5432/ffball"
              "RUST_LOG=info"
              "RUST_BACKTRACE=1"
            ];
          };
        };

        # Docker image for frontend (static files only)
        frontendImage = pkgs.dockerTools.buildImage {
          name = "ffball-frontend";
          tag = "latest";
          copyToRoot = [ frontend ];
          config = {
            Cmd = [ "sh" "-c" "echo 'Static files ready to be served by nginx'" ];
          };
        };

      in
      {
        # Development shell
        devShells.default = devShell;

        # Packages
        packages = {
          inherit frontend backend backendImage frontendImage;
          default = backend;
          
        # System tools package (for EC2 installation)
        system-tools = pkgs.buildEnv {
          name = "system-tools";
          paths = systemDeps;
          meta = {
            description = "System tools for Fantasy Football Draft Board deployment";
            platforms = pkgs.lib.platforms.linux;
          };
        };
        };

      }
    );
}