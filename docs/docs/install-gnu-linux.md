How to install rapd on GNU/Linux

## Installing from Source
If you want more the latest features in the master branch its recommended you build from source.
First clone the source repo with git:
```bash
git clone https://github.com/Interfiber/rapd.git
```
Then install [rustup](https://rustup.rs) and switch to the latest version like this: ```rustup default stable```.
Then cd into the cloned directory and run the build:
```bash
cargo build --release
sudo cp ./target/release/rapd /usr/bin/rapd
sudo cp ./target/release/rapc /usr/bin/rapc
# RAPD notify plugin: sudo cp ./target/release/rapd-notify /usr/local/bin/rapd-notify
```

## Installing from the AUR
RAPD from the aur uses the latest release of rapd from github, to install it use an AUR helper like below:
```bash
yay -S rapd # yay is used here
```
