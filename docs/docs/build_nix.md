# Build from source on NixOS

!!! note
    Currently rapd does not use flakes, support for flakes will be added soon!


## Step 1. Clone from GitHub
You can clone the repo from github using the following command
```bash
git clone https://github.com/Interfiber/rapd
```

## Step 2. Enter a Nix shell
Now enter a nix-shell using the provided ```shell.nix```, you can do this using the command below
```bash
nix-shell
```

## Step 3. Build binarys
Now build the binarys using cargo
```bash
cargo build # build a debug build
cargo build --release # build a release build
```

## Step 4. Move binarys into ```PATH```
Now ```mv``` the resulting binary into a directory thats in your ```PATH```, such as ```~/.local/share/bin```
