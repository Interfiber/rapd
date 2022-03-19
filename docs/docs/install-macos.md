How to install RAPD on MacOS

## From Source
First install [rustup](https://rustup.rs) and switch to the latest version like this: ```rustup default stable```.
Then clone the source repo like this:
```bash
git clone https://github.com/Interfiber/rapd.git
```
And build the release like this:
```bash
cargo build --release
# or with features like this: cargo build --release --features discordplugin,notifyplugin
```
Then install the binarys with the following:
```bash
sudo mkdir -p /usr/local/bin
sudo mv target/release/rapd /usr/local/bin/rapd
sudo mv target/release/rapc /usr/local/bin/rapc
```
