pub enum AudioStartStatus {
    FSError, // Error with reading the file, or permission errors
    Success, // Everything is OK
    ThreadingError // Problem spawning the audio thread
}
