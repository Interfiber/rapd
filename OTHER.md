# Rapd architecture


## Current playing file
When the player starts playing a file rapd symlinks that file to ```~/.local/share/rapd/current```. When complete rapd deletes this file

## Statefile
Rapd caches it state in a file called the statefile, which contains a string which can be converted into a ```PlayerState``` enum during runtime.
Never touch this file.

