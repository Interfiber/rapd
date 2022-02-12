# Rapc
Rust Audio Player Client

## Playing an audio file from disk
```bash
rapc play ~/foo/bar.mp3
```


## Rebuild the music database
```bash
rapc db_rebuild
```

## Play a song from the music database
```bash
rapc db_select
# 1: ~/foo/bar.mp3
# Select: 1
# Playing...
```

## Print the contents of the music database
```bash
rapc db_print
```
