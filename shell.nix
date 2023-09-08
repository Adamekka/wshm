# VS Code is using this
{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    gtk4
    gtk4-layer-shell
    pkg-config
    rustup
  ];
}
