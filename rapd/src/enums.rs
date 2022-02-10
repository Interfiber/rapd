// Status of the audio player startup
pub enum AudioStartStatus {
    FSError,         // Error with reading the file, or permission errors
    Success,        // Everything is OK
    ThreadingError // Problem spawning the audio thread
}

// State of the audio player
#[derive(PartialEq)]
pub enum PlayerState {
    Playing,   // Playing an audio file
    Idle,     // The player is doing nothing, or idling
}
