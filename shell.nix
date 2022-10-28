let
  # Rolling updates, not deterministic.
  pkgs = import (fetchTarball("channel:nixpkgs-unstable")) {};
in pkgs.mkShell {
  buildInputs = [ pkgs.rustup pkgs.pkg-config pkgs.libpulseaudio pkgs.rust-analyzer ];
}
