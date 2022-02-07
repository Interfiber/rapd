use soloud::*;
use std::thread::Builder;
use std::path::Path;
use crate::requests::AudioPlayRequest;

pub fn play_audio_file(file: &str) {
    // check if we already have an audio file playing
    let sl = Soloud::default().unwrap();
    let mut wav = audio::Wav::default();
    info!("Loading audio data into memory");
    wav.load(&std::path::Path::new(file)).expect("Failed to load audio data into memory");
    info!("Starting audio playback for file: {}", file);
    sl.play(&wav);
    while sl.voice_count() > 0 {
        if Path::new("/tmp/rapd.stop_player_thread").exists() {
            info!("File rapd.stop_player_thread exists, shutting down player thread");
            std::fs::remove_file("/tmp/rapd.stop_player_thread").unwrap();
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    info!("Audio playback completed for file: {}", file);
}

pub fn play_audio_from_request(request: AudioPlayRequest) {
    info!("Spawning audio thread...");    
    match Builder::new().name("player".to_string()).spawn(move || play_audio_file(&request.audio_file_path)){
        Ok(_) => {
            info!("Spawned audio thread");
        },
        Err(err) => {
            error!("Failed to spawn audio thread: {}", err);
        }
    }
}
pub fn stop_player() {
    warn!("Stopping player on request");
    // NOTE: The code inside the play_audio_file function every few milliseconds checks if a file
    // named rapd.stop_player_thread exists, if it does it deletes it and closes the player thread
    // with a break statement
    match std::fs::write("/tmp/rapd.stop_player_thread", ""){
        Ok(_) => info!("Stop file created"),
        Err(err) => {
            error!("Failed to create stop file: {}", err)
        }
    }
}
