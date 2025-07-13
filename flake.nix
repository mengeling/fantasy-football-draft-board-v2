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
        
        # System dependencies
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
          jq
          yq
        ];

        # Development tools
        devTools = with pkgs; [
          rustToolchain
          nodejs
          postgresql
          nginx
          sqlx-cli
          cargo-watch
          cargo-edit
          cargo-audit
          bacon
          dive # Docker image analysis
          k9s # Kubernetes CLI
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

        # Docker image builds
        backendImage = pkgs.dockerTools.buildImage {
          name = "ffball-backend";
          tag = "latest";
          contents = [ backend pkgs.bash pkgs.coreutils pkgs.curl ];
          config = {
            Cmd = [ "${backend}/bin/backend" ];
            ExposedPorts = {
              "8080/tcp" = {};
            };
            WorkingDir = "/app";
          };
        };

        frontendImage = pkgs.dockerTools.buildImage {
          name = "ffball-frontend";
          tag = "latest";
          contents = [ frontend pkgs.nginx ];
          config = {
            Cmd = [ "${pkgs.nginx}/bin/nginx" "-g" "daemon off;" ];
            ExposedPorts = {
              "80/tcp" = {};
            };
          };
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
          frontend-image = frontendImage;
          backend-image = backendImage;
          default = backend;
        };

        # Development apps
        apps = {
          backend = flake-utils.lib.mkApp {
            drv = backend;
            exePath = "/bin/backend";
          };
        };

        # CI/CD formatter
        formatter = pkgs.nixpkgs-fmt;

        # System configurations for deployment
        nixosConfigurations = {
          # AWS EC2 configuration
          ec2-instance = nixpkgs.lib.nixosSystem {
            inherit system;
            modules = [
              ./deploy/nixos/ec2-configuration.nix
              {
                system.stateVersion = "24.05";
                networking.hostName = "ffball-server";
                
                # Enable Docker
                virtualisation.docker.enable = true;
                
                # Install required packages
                environment.systemPackages = with pkgs; [
                  docker
                  docker-compose
                  git
                  curl
                  nginx
                  postgresql
                  awscli2
                ];
                
                # Enable services
                services.nginx.enable = true;
                services.postgresql.enable = true;
                services.postgresql.package = pkgs.postgresql_16;
                
                # Open firewall ports
                networking.firewall.allowedTCPPorts = [ 22 80 443 ];
              }
            ];
          };
        };
      }
    );
}