{
  description = "Rust Development Flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      system = "x86_64-darwin";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ (import rust-overlay) ];
      };

      rust = pkgs.rust-bin.stable.latest.default;
      rust-src = pkgs.rust-bin.stable.latest.rust-src;
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = [
          rust
          rust-src
          pkgs.rust-analyzer
          pkgs.cargo-watch
        ];

        RUST_SRC_PATH = "${rust-src}/lib/rustlib/src/rust/library";
      };
    };
}
