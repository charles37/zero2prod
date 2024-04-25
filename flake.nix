{
  description = "Rust Development Shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
          toolchain.default.override {
            extensions = [
              "rust-src"
              "rustfmt"
              "clippy"
              "rust-analyzer"
            ];
          });
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          preConfigure = ''
            export RUST_LOG=debug
          '';
          pname = "zero2prod";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = [
            pkgs.pkg-config
          ];
          buildInputs = [
            pkgs.openssl
            pkgs.sqlx-cli
          ];
          #RUST_LOG = "debug";
          #SQLX_OFFLINE = true;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [
            rust
            pkgs.openssl
            pkgs.pkg-config
            pkgs.sqlx-cli
            pkgs.bunyan-rs
          ];
        };
      }
    );
}
