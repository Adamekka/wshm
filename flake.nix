{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk { };

      in
      rec {
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            bacon
            gtk4
            gtk4-layer-shell
            pkg-config
            rustup
          ];
        };
      }
    );
}
