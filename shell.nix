{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    clippy
    gccStdenv
    openssl.dev
    pkg-config
    rustc

    # keep this line if you use bash
    bashInteractive
  ];
}
