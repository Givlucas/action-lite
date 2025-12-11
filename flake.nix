{
  description = "action-lite CLI - Command-line tool for managing action-lite methodology workflows";

  inputs = {
    # Use nixpkgs-unstable for recent Rust toolchain and packages
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    # naersk - Nix build system for Rust projects using Cargo
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    # flake-utils for ergonomic multi-system support
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, naersk, flake-utils }:
    # Only support x86_64-linux as per specification #8
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let
        pkgs = import nixpkgs { inherit system; };

        # Initialize naersk for this system
        naersk-lib = pkgs.callPackage naersk { };

      in {
        # Default package: the action-lite CLI tool
        packages.default = naersk-lib.buildPackage {
          # Build from project root
          src = ./.;

          # naersk will read package name and metadata from Cargo.toml
          # No need to specify pname and version explicitly

          # Build only for Linux (no cross-compilation)
          # naersk handles this automatically via Cargo
        };

        # Development shell with Rust toolchain and tools
        devShells.default = pkgs.mkShell {
          # Development dependencies
          buildInputs = with pkgs; [
            # Rust compiler and package manager
            rustc
            cargo

            # Development tools
            rust-analyzer  # LSP server for IDE integration
            clippy         # Rust linter
            rustfmt        # Code formatter

            # Useful for debugging Nix issues
            nix-tree
          ];

          # Environment setup for development
          shellHook = ''
            echo "action-lite development environment"
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
            echo ""
            echo "Available commands:"
            echo "  cargo build    - Build the project"
            echo "  cargo test     - Run tests"
            echo "  cargo run      - Run the CLI"
            echo "  nix build      - Build with Nix"
          '';
        };

        # Make the package available via 'nix run'
        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/action-lite";
        };
      }
    );
}
