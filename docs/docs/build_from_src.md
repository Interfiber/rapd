# Build from source

!!! warning
    This guide will not work on NixOS, please follow the [NixOS guide](./build_nix.md) instead!


## Step 1. Install rustup
To build rapd, you need to install [rustup](https://rustup.rs). The following terminal command will install it
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


## Step 2. Install required librarys
RAPD will require the following libraries/tools installed on your system:

  - PulseAudio, if using pipewire the pulseaudio pipewire plugin needs to be installed
  - GCC
  - Git

## Step 3. Clone RAPD source code
You can clone the rapd source code with git as shown below
```bash
git clone https://github.com/Interfiber/rapd.git
```

## Step 4. Build the source code
You can build the source code using cargo, the output binary will be located at ```target/debug/rapd```, or ```target/release/rapd```
```bash
cargo build # build debug
cargo build --release # build release
```

## Step 5. Move the binary into your ```PATH```
Now ```mv``` the binary into a location in your ```PATH```, such as ```/usr/bin```, ```/usr/local/bin```, or somewhere that you specified to be in your path.
