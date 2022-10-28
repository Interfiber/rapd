# RAPD
Rust audio player daemon 2.0

## Changes in version 2.0

- Better code readability
- No need to make multiple connections to perform diffrent tasks, the server can keep the connection open after a response
- No on-disk statefile, which removes the chance of state errors, this also allows the player to pause/stop/begin the audio faster
- Better request/response format
- No use of rodio, symphonia is used for audio decoding instead
- Scrubbing (TODO)
- Get place in song 
- Better metadata support (TODO)
- Built in notifications (TODO)
- Configure during runtime, or with startup script via rapc (TODO)
- Better rapc code readability (TODO)
- More verbose rapc command line interface (TODO)
- Built in discord RPC (TODO)
- Better music database
