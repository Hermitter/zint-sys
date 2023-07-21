{
  description = "Rust-bindgen for Zint";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-23.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , rust-overlay
    }:

    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs { inherit system overlays; };
      overlays = [ (import rust-overlay) ];
    in
    rec
    {
      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          valgrind
          nixpkgs-fmt

          rust-bin.stable.latest.default
          cargo-audit
        ];

        buildInputs = with pkgs; [
          rustPlatform.bindgenHook
          zint
        ];

        shellHook = ''
          ${pkgs.zint}/bin/zint --version
        '';

        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    });
}