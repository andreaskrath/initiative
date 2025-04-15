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
      in {
        devShells.default = with pkgs;
          mkShell {
            buildInputs = [
              rust-bin.stable.latest.default
              rust-analyzer
              taplo
              vscode-extensions.vadimcn.vscode-lldb.adapter

              nodejs
              node_modules
            ];
          };
      }
    );
}
