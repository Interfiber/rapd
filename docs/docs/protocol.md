Guide to the RAPD server protocol and API

## The TCP socket
RAPD will listen on the following address by default: ```http://127.0.0.1:8932```.

## Sending requests
One a client has connected to the socket, the server will wait for a request to be sent, once one is sent it will be parsed and a response is sent back. Once the server completes this it will terminate the connection.

## Basic request
All requests sent to the rapd server MUST be json, and the server will parse them as json. All requests follow the structure like below:
```json
{
   "request_type": "type",
   "key": "value"
}
```

## current_file
About: Returns the current playing file, if nothing it returns empty

Expects:
```json
{
   "request_type": "current_file"
}
```

Returns:
```json
{
   "error": true/false,
   "message": "empty or file path",
   "request_type": "Succeeded/??"
}
```

## get_music
About: Returns the contents of the JSON music database

Expects: 
```json
{
   "request_type": "get_music"
}
```

Returns:
```json
{
  "error": true/false,
  "message": ["foo.mp3", "bar.mp3"],
  "request_type": "Succeeded/??"
}
```

## play_audio_file
About: Plays an audio file directly from disk

Expects:
```json
{
  "request_type": "play_audio_file",
  "audio_file_path": "path/to/audio/file.mp3",
  "loop_audio": true/false
}
```

Returns:
```json
{
  "error": true/false,
  "message": "Attempting audio playback.../??",
  "request_type": "Succeeded"
}
```

## stop_player
About: Halts the audio play, **DOES NOT** shutdown the rapd server

Expects:
```json
{
  "request_type": "stop_player"
}
```

Returns:
```json
{
  "error": true/false,
  "message": "Sent stop request to player",
  "request_type": "Succedded/??"
}
```

## server_shutdown
About: Sends a shutdown request to the rapd server

Expects:
```json
{
  "request_type": "server_shutdown"
}
```

Returns: 
```json
{
 "error": true/false,
 "message": "Shutting down server",
 "request_type": "Succeeded/??"
}
```

## rebuild_music_database
About: Rebuilds the rapd music database from disk

Expects:
```json
{
 "request_type": "rebuild_music_db"
}
```

Returns:
```json
{
  "error": true/false,
  "message": "Rebuilt the music database/??"
  "request_type": "Succeeded/??"
}
```

## metadata_get
About: Gets a supported metadata value from a file

Expects:
```json
{
  "request_type": "metadata_get",
  "metadata_type": "title/author",
  "path": "/path/to/audio/file.mp3"
}
```

Returns:
```json
{
  "request_type": "Success/??",
  "error": true/false,
  "message": "author of audio/unknown"
}
```

## metadata_set
About: Sets a supported metadata value for a file

Expects:
```json
{
  "request_type": "metadata_set",
  "metadata_type": "title/author",
  "path": "/path/to/audio/file.mp3",
  "new_value": "epic author name"
}
```

Returns:
```json
{
  "request_type": "Success/??",
  "error": true/false,
  "message": "Updated file metadata/??"
}
```

## hook_add
About: Adds a hook to the rapd database

Expects:
```json
{
  "request_type": "hook_add",
  "hook_type": "player_start/...",
  "command": "command/to/execute"
}
```

Returns:
```json
{
  "request_type": "Succeeded/??",
  "error": true/false
  "message": "Hook added/??"
}
```

## player_pause
About: Pauses the music player

Expects:
```json
{
  "request_type": "pause_player"
}
```

Returns:
```json
{
  "request_type": "Success/??"
  "error": true/false,
  "message: "Paused player/??"
}
```

## player_unpause
About: Unpauses, or resumes the player

Expects:
```json
{
  "request_type": "unpause_player"
}
```

Returns:
```json
{
  "request_type": "Success",
  "error": true/false,
  "message": "Unpaused player"
}
```
