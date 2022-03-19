A guide to create a rapd config
## Why?
Before we can add any music, rapd needs a config. The config we will create will tell rapd where it should look for music.

## Creating the config
On first startup rapd should create a config file at ```~/.config/rapd/config.toml```, which is empty by default.
Lets add the following content to that file
```toml
[configuration]
# replace the path below to the path to you're music folder.
music_dir = "/home/intefiber/Music"
```
What this does is it tells rapd where music should be located, in this case thats in our home-directory in a Music folder(make sure this exists).
Once this file is saved we are ready to go!
