// Status of the audio player startup
pub enum AudioStartStatus {
    FSError,        // Error with reading the file, or permission errors
    Success,        // Everything is OK
    ThreadingError, // Problem spawning the audio thread
}

// State of the audio player
#[derive(PartialEq)]
pub enum PlayerState {
    Playing,    // Playing an audio file
    Idle,       // The player is doing nothing, or idling
    Killed,     // The player has been killed, or is shutdown
    Stop,       // The player is stopping the audio, and will change to a idle state when done
    Rebuilding, // The player is rebuilding the music database
    Paused,     // The player is paused
    Unpaused,   // The player is changing to a playing state from being paused
}

// Music database rebuild state
pub enum MusicDatabaseRebuildState {
    ConfigError,
    FSError,
    DatabaseWriteError,
    Rebuilt,
    PlayerRunning,
}

// Hook types
#[derive(PartialEq, Eq)]
pub enum HookType {
    PlayerStart,
    PlayerPause,
    PlayerUnpause,
    ServerShutdown,
    Unknown,
}

pub enum HookAddState {
    Added,
    FsError,
    InvalidHookType,
}

pub enum MetadataEditState {
    Wrote,
    FileReadError,
    MetadataWriteError,
    InvalidType
}
