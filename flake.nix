{
  description = "Action Lite - A file-based task tracking system";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
          config.allowUnfree = true;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
        ];

        buildInputs = with pkgs; [
          # Add any system dependencies here
        ];

      in
      {
        packages = {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "action-lite";
            version = "0.1.0";

            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            inherit nativeBuildInputs buildInputs;

            meta = with pkgs.lib; {
              description = "A file-based task tracking system using acyclic directed meta graphs";
              homepage = "https://github.com/yourusername/action-lite";
              license = licenses.mit;
              maintainers = [ ];
            };
          };
        };

        devShells.default = pkgs.mkShell {
          inherit buildInputs;
          nativeBuildInputs = nativeBuildInputs ++ (with pkgs; [
            cargo-watch
            cargo-edit
            clippy
            rustfmt
            claude-code
          ]);

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";

          shellHook = ''
            echo "🦀 Action Lite development environment"
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
            echo "Claude Code version: $(claude --version 2>/dev/null || echo 'not found')"
            echo ""
            echo "Available commands:"
            echo "  cargo build       - Build the project"
            echo "  cargo run         - Run the project"
            echo "  cargo test        - Run tests"
            echo "  cargo clippy      - Run linter"
            echo "  cargo fmt         - Format code"
            echo "  cargo watch       - Watch and rebuild on changes"
            echo "  claude            - Run Claude Code"
          '';
        };

        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/action-lite";
        };
      }
    );
}
