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
        nodejs = pkgs.nodejs_22;
        
        # Database
        postgresql = pkgs.postgresql_16;
        
        # Development shell with Linux-compatible tools
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            nodejs_22
            nodePackages.npm
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

        # Frontend - built with Docker (not Nix)
        # Use 'just build-frontend' to build the frontend image

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

      in
      {
        # Development shell
        devShells.default = devShell;

        # Packages
        packages = {
          inherit backend backendImage;
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