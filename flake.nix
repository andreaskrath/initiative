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

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = ["rust-src" "rust-analyzer"];
        };

        iced = with pkgs; [
          libx11
          libxcursor
          libxi
          libxkbcommon
          libxcb
          libGL
          vulkan-loader
        ];
      in {
        devShells.default = with pkgs;
          mkShell {
            buildInputs =
              [
                rustToolchain
                vscode-extensions.vadimcn.vscode-lldb.adapter

                # tokio
                pkg-config
                openssl
              ]
              ++ iced;
            RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
            LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath iced}:$LD_LIBRARY_PATH";
          };
      }
    );
}
