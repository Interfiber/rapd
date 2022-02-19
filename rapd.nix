# very basic rust nix config
with import <nixpkgs> {};
stdenv.mkDerivation {
    name = "rapd-development"; # Probably put a more meaningful name here
    buildInputs = [ pkg-config cmake gcc rustup gnumake alsa-lib alsa-oss ];
}
