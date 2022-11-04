# RAPD
Rust audio player daemon 2.0

## Changes in version 2.0

- Better code readability
- No need to make multiple connections to perform diffrent tasks, the server can keep the connection open after a response
- No on-disk statefile, which removes the chance of state errors, this also allows the player to pause/stop/begin the audio faster
- Better request/response format
- No use of rodio, symphonia is used for audio decoding instead
- Get place in song 
- Better metadata support
- Built in notifications
- Configure during runtime, or with startup script that uses rapc
- Better rapc code readability
- More verbose rapc command line interface
- Better music database
