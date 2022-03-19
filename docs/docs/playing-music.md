This guide will help you get started playing music using [rapc](https://github.com/Interfiber/rapd/tree/main/rapc)

## Play the music
Rapc has a command to print the music database and select a entry from it to play this command is called ```db_select```. You can use it like below
```bash
rapc db_select
```
This will print out the music database files, with numbers beside them selecting the number and pressing enter will play it.

## Playing music directly from disk
If you want to play music directly from disk, without the music database you can use rapc to play it directly like below
```bash
rapc play /path/to/music/file.mp3
```

## Print the current playing file
To print the current playing file you can use the following command
```bash
rapc player_file
# OR use --full-path for the full file path
rapc player_file --full-path
```

## Stop the player
To stop the playing of the current song you can use the following command
```bash
rapc player_stop
```

