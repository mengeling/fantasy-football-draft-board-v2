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
          inherit system overlays;
        };
        
        # Rust toolchain
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        # Node.js environment
        nodejs = pkgs.nodejs_20;
        
        # Database
        postgresql = pkgs.postgresql_16;
        
        # System dependencies (for deployment and basic operations)
        systemDeps = with pkgs; [
          pkg-config
          openssl
          libpq
          chromium
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
        ];

        # Development tools (for development and debugging)
        devTools = with pkgs; [
          rustToolchain
          nodejs
          postgresql
          cargo-watch
        ];

        # Build inputs for Rust
        rustInputs = with pkgs; [
          pkg-config
          openssl
          libpq
          chromium
        ];

        # Frontend build
        frontend = pkgs.stdenv.mkDerivation {
          name = "ffball-frontend";
          src = ./frontend;
          nativeBuildInputs = [ nodejs ];
          buildPhase = ''
            export HOME=$(mktemp -d)
            npm ci
            npm run build
          '';
          installPhase = ''
            mkdir -p $out
            cp -r build/* $out/
          '';
        };

        # Backend build
        backend = pkgs.rustPlatform.buildRustPackage {
          name = "ffball-backend";
          src = ./backend;
          cargoLock = {
            lockFile = ./backend/Cargo.lock;
          };
          nativeBuildInputs = rustInputs;
          buildInputs = rustInputs;
          
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



      in
      {
        # Development shell
        devShells.default = pkgs.mkShell {
          buildInputs = systemDeps ++ devTools;
          
          shellHook = ''
            echo "Fantasy Football Draft Board Development Environment"
            echo "====================================================="
            echo "Available commands:"
            echo "  cargo watch -x run          # Run backend with auto-reload"
            echo "  npm run dev                  # Run frontend dev server"
            echo "  docker-compose up            # Start all services"
            echo "  terraform plan               # Plan infrastructure changes"
            echo "  ./scripts/deploy.sh          # Deploy to production"
            echo ""
            echo "Database URL: postgres://ffball:ffball@localhost:5432/ffball"
            echo ""
            
            # Set up environment variables
            export DATABASE_URL="postgres://ffball:ffball@localhost:5432/ffball"
            export RUST_LOG="info"
            export RUST_BACKTRACE="1"
            
            # Add local node_modules to PATH
            export PATH="$PWD/frontend/node_modules/.bin:$PATH"
          '';
        };

        # Packages
        packages = {
          inherit frontend backend;
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