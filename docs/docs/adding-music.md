This guide will help you add music to rapd, note that the [config guide](create-a-config.md) should be completed first.

## How rapd stores music
Rapd keeps track of all of you're music in a database located in ```~/.local/share/rapd/db.json```. This file has a list of files that rapd knows is music

## Adding music
To add music simply place all of the music files in the ```music_dir``` we set in the creating a config chapter, as this is where rapd will look for music.

## Rebuilding the music database
Rapd dosent auto-detect music changes, so when ever you have updated you're library you need to tell rapd to rebuild its database.
You can do this using a rapd client(in this case rapc) like below
```bash
rapc db_rebuild
# Rebuilt database
```
This will rebuild rapd's music database with all of the new music added.
