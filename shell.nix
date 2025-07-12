{ pkgs ? import <nixpkgs> {} }:

let
  rustToolchain = pkgs.rust-bin.stable.latest.default.override {
    extensions = [ "rust-src" "rust-analyzer" ];
  };

  backendDeps = with pkgs; [
    rustToolchain
    pkg-config
    openssl
    libpq
    postgresql
    cargo-watch
    cargo-audit
  ];

  frontendDeps = with pkgs; [
    nodejs_20
    nodePackages.npm
    nodePackages.pnpm
    nodePackages.typescript
    nodePackages.vite
  ];

  devOpsDeps = with pkgs; [
    docker
    docker-compose
    nginx
    certbot
    curl
    jq
    git
    ripgrep
    htop
  ];

in pkgs.mkShell {
  buildInputs = backendDeps ++ frontendDeps ++ devOpsDeps;
  
  shellHook = ''
    echo "🚀 Fantasy Football Draft Board Development Environment"
    echo "=================================================="
    echo "Available tools:"
    echo "  Backend (Rust): cargo, rustc, rust-analyzer"
    echo "  Frontend (Svelte): node, npm, pnpm, vite"
    echo "  DevOps: docker, nginx, certbot"
    echo "  Utilities: git, curl, jq, ripgrep"
    echo ""
    echo "Quick commands:"
    echo "  just dev-backend    - Start backend development server"
    echo "  just dev-frontend   - Start frontend development server"
    echo "  just test           - Run all tests"
    echo "  just build          - Build both backend and frontend"
    echo "  just deploy         - Deploy to production"
    echo ""
  '';
  
  RUST_BACKTRACE = "1";
  RUST_LOG = "debug";
  DATABASE_URL = "postgresql://ffball:ffball@localhost:5432/ffball_dev";
  RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
} 