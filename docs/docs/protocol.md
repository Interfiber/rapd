# RAPD TCP protocol reference

!!! note
  
  Read the [connection guide](./connect.md) to find out how to create a connection
  to the RAPD tcp server.

!!! note
  Find a list of player states in the [player states list](./states.md)

## Ping packet

=== "Request"

    ```json
    {
      "command": "ping",
      "params": [],
      "client_name": "test"
    }
    ```

=== "Response"

    ```json
    {
      "message": "RAPD server is up! pong",
      "timestamp": "000",
      "failed": false
    }
    ```

## Play file packet

=== "Request"

    ```json
    {
      "command": "play_file",
      "params": ["file_path", "should_loop"],
      "client_name": "test"
    }
    ```

=== "Response"

    ```json
    {
      "message": "Started audio playback",
      "timestamp": "000",
      "failed": false
    }
    ```

## Player state packet

=== "Request"

    ```json
    {
      "command": "player_state",
      "params": [],
      "client_name": "test"
    }
    ```

=== "Response"

    ```json
    {
      "message": "[PLAYER STATE IS HERE]",
      "timestamp": "000",
      "failed": false
    }
    ```

## Player stop packet

=== "Request"

    ```json
    {
      "command": "player_stop",
      "params": [],
      "client_name": "test"
    }
    ```

=== "Response"

    ```json
    {
      "message": "Stopped player",
      "timestamp": "000",
      "failed": false
    }
    ```

## Player toggle pause packet

=== "Request"

    ```json
    {
      "command": "player_toggle_pause",
      "params": [],
      "client_name": "test"
    }
    ```

=== "Response"

    ```json
    {
      "message": "Toggled pause for player, is_paused = true/false",
      "timestamp": "000",
      "failed": false
    }
    ```

## Player length packet

=== "Request"

    ```json
    {
      "command": "player_length",
      "params": [],
      "client_name": "test"
    }
    ```

=== "Response"

    ```json
    {
      "message": {
        "hour": 0,
        "min": 0,
        "second": 0
      },
      "timestamp": "000",
      "failed": false
    }
    ```

## Player time packet

=== "Request"

    ```json
    {
      "command": "player_time",
      "params": [],
      "client_name": "test"
    }
    ```

=== "Response"

    ```json
    {
      "message": {
        "hour": 0,
        "min": 0,
        "second": 0
      },
      "timestamp": "000",
      "failed": false
    }
    ```

## Player file packet

=== "Request"

    ```json
    {
      "command": "player_file",
      "params": [],
      "client_name": "test"
    }
    ```

=== "Response"

    ```json
    {
      "message": "/path/to/file",
      "timestamp": "000",
      "failed": false
    }
    ```
