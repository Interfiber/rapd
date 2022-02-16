# very basic rust nix config
with import <nixpkgs> {};
stdenv.mkDerivation {
    name = "dev-environment"; # Probably put a more meaningful name here
    buildInputs = [ pkg-config cmake gcc rustup gnumake alsa-lib alsa-oss ];
}
