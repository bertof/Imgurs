{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    cargo-audit
    clippy
    gccStdenv
    openssl.dev
    pkg-config
    rustc

    # keep this line if you use bash
    bashInteractive
  ];
}
