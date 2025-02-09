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
      in {
        devShells.default = with pkgs;
          mkShell rec {
            buildInputs = [
              rust-bin.stable.latest.default
              rust-analyzer
              taplo

              pkg-config
              libxkbcommon
              vulkan-loader
              xorg.libX11
              xorg.libXcursor
              xorg.libXrandr
              xorg.libXi
              xorg.libXinerama

              mesa.dev
              glxinfo
            ];
            LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
          };
      }
    );
}
