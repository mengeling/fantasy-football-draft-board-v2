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
          sqlx-cli
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

        # Docker images built with Nix
        backendImage = pkgs.dockerTools.buildLayeredImage {
          name = "ffball-backend";
          tag = "latest";
          contents = with pkgs; [ 
            backend 
            cacert        # SSL certificates
            chromium      # For web scraping
            bash          # For scripts
            coreutils     # Basic utilities
          ];
          config = {
            Cmd = [ "${backend}/bin/backend" ];
            ExposedPorts = {
              "8080/tcp" = {};
            };
            Env = [
              "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
            ];
            User = "1000:1000";
            WorkingDir = "/app";
          };
          # Create app user and directory
          extraCommands = ''
            mkdir -p app
            echo "app:x:1000:1000:app:/app:/bin/bash" >> etc/passwd
            echo "app:x:1000:" >> etc/group
            chown 1000:1000 app
          '';
        };

        frontendImage = pkgs.dockerTools.buildLayeredImage {
          name = "ffball-frontend";
          tag = "latest";
          contents = with pkgs; [ 
            nginx
            frontend
            cacert
          ];
          config = {
            Cmd = [ "${pkgs.nginx}/bin/nginx" "-g" "daemon off;" ];
            ExposedPorts = {
              "80/tcp" = {};
            };
            Env = [
              "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
            ];
            User = "nginx";
          };
          # Set up nginx configuration
          extraCommands = ''
            mkdir -p etc/nginx var/log/nginx var/cache/nginx
            mkdir -p var/cache/nginx/{client_temp,proxy_temp,fastcgi_temp,uwsgi_temp,scgi_temp}
            
            # Create nginx.conf
            cat > etc/nginx/nginx.conf << 'EOF'
            user nginx;
            worker_processes auto;
            error_log /var/log/nginx/error.log warn;
            pid /var/run/nginx.pid;

            events {
                worker_connections 1024;
            }

            http {
                include ${pkgs.nginx}/conf/mime.types;
                default_type application/octet-stream;
                
                log_format main '$remote_addr - $remote_user [$time_local] "$request" '
                               '$status $body_bytes_sent "$http_referer" '
                               '"$http_user_agent" "$http_x_forwarded_for"';
                
                access_log /var/log/nginx/access.log main;
                sendfile on;
                keepalive_timeout 65;
                
                server {
                    listen 80;
                    server_name localhost;
                    root ${frontend};
                    index index.html;
                    
                    location / {
                        try_files $uri $uri/ /index.html;
                    }
                    
                    location /api/ {
                        proxy_pass http://backend:8080/;
                        proxy_set_header Host $host;
                        proxy_set_header X-Real-IP $remote_addr;
                        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
                        proxy_set_header X-Forwarded-Proto $scheme;
                    }
                }
            }
            EOF
            
            # Create nginx user
            echo "nginx:x:101:101:nginx:/var/cache/nginx:/sbin/nologin" >> etc/passwd
            echo "nginx:x:101:" >> etc/group
            
            # Set permissions
            chown -R 101:101 var/cache/nginx var/log/nginx
            chmod 755 var/cache/nginx var/log/nginx
          '';
        };

        # Combined development image for easier local development
        devImage = pkgs.dockerTools.buildLayeredImage {
          name = "ffball-dev";
          tag = "latest";
          contents = systemDeps ++ devTools ++ [ cacert ];
          config = {
            Cmd = [ "${pkgs.bash}/bin/bash" ];
            Env = [
              "DATABASE_URL=postgres://ffball:ffball@localhost:5432/ffball"
              "RUST_LOG=info"
              "RUST_BACKTRACE=1"
              "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
            ];
            WorkingDir = "/workspace";
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
          default = backend;
          
          # Docker images
          inherit backendImage frontendImage devImage;
          
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