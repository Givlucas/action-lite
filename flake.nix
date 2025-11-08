{
  description = "Action Lite - A file-based task tracking system";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, naersk, pre-commit-hooks }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
          config.allowUnfree = true;
        };

        # Pin Rust version for reproducibility
        rustToolchain = pkgs.rust-bin.stable."1.83.0".default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        # Initialize naersk with our Rust toolchain
        naersk-lib = pkgs.callPackage naersk {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };

        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
        ];

        buildInputs = with pkgs; [
          # Add any system dependencies here
        ];

        # Clean source filtering - exclude build artifacts and VCS
        src = pkgs.lib.cleanSourceWith {
          src = ./.;
          filter = path: type:
            let baseName = baseNameOf path;
            in !(pkgs.lib.hasPrefix "target" baseName ||
                 pkgs.lib.hasPrefix ".git" baseName ||
                 pkgs.lib.hasPrefix ".claude" baseName ||
                 pkgs.lib.hasPrefix "result" baseName ||
                 baseName == "flake.lock");
        };

        # Pre-commit hooks configuration
        pre-commit-check = pre-commit-hooks.lib.${system}.run {
          src = ./.;
          hooks = {
            rustfmt = {
              enable = true;
              entry = "${rustToolchain}/bin/cargo-fmt fmt --all -- --check";
            };
            clippy = {
              enable = true;
              entry = "${rustToolchain}/bin/cargo-clippy clippy --all-targets --all-features -- -D warnings";
            };
            cargo-check = {
              enable = true;
              entry = "${rustToolchain}/bin/cargo check --all-targets --all-features";
            };
          };
        };

      in
      {
        packages = {
          default = naersk-lib.buildPackage {
            pname = "action-lite";
            version = "0.1.0";

            inherit src;
            inherit nativeBuildInputs buildInputs;

            meta = with pkgs.lib; {
              description = "A file-based task tracking system using acyclic directed meta graphs";
              homepage = "https://github.com/Givlucas/action-lite";
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
            cargo-outdated
            clippy
            rustfmt
            claude-code
          ]);

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";

          shellHook = ''
            ${pre-commit-check.shellHook}
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
            echo "  cargo outdated    - Check for outdated dependencies"
            echo "  claude            - Run Claude Code"
            echo ""
            echo "Pre-commit hooks are installed and will run automatically"
          '';
        };

        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/action-lite";
        };

        # Checks for CI/CD integration
        checks = {
          # Run the package build as a check
          build = self.packages.${system}.default;

          # Run cargo test
          cargo-test = pkgs.runCommand "cargo-test" {
            buildInputs = buildInputs ++ nativeBuildInputs;
          } ''
            cd ${src}
            export HOME=$(mktemp -d)
            ${rustToolchain}/bin/cargo test --release --all-features
            touch $out
          '';

          # Run clippy with strict warnings
          cargo-clippy = pkgs.runCommand "cargo-clippy" {
            buildInputs = buildInputs ++ nativeBuildInputs;
          } ''
            cd ${src}
            export HOME=$(mktemp -d)
            ${rustToolchain}/bin/cargo clippy --all-targets --all-features -- -D warnings
            touch $out
          '';

          # Verify formatting
          cargo-fmt = pkgs.runCommand "cargo-fmt" {
            buildInputs = buildInputs ++ nativeBuildInputs;
          } ''
            cd ${src}
            export HOME=$(mktemp -d)
            ${rustToolchain}/bin/cargo fmt --all -- --check
            touch $out
          '';

          # Pre-commit hooks check
          pre-commit = pre-commit-check;
        };

        # Formatter for `nix fmt`
        formatter = pkgs.nixpkgs-fmt;
      }
    );
}
