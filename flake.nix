{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
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
        node_modules = with pkgs.nodePackages; [
          npm
          svelte-check
          svelte-language-server
          typescript
          typescript-language-server
          eslint
        ];

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = ["rust-src" "rust-analyzer"];
        };
      in {
        devShells.default = with pkgs;
          mkShell {
            buildInputs = [
              rustToolchain
              vscode-extensions.vadimcn.vscode-lldb.adapter
              openssl
              pkg-config
              sqlx-cli

              nodejs
              node_modules
            ];
            RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          };
      }
    );
}
