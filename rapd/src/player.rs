use soloud::*;
use std::thread::Builder;
use std::path::Path;
use crate::enums::AudioStartStatus;
use crate::enums::PlayerState;
use crate::state::set_state;
use crate::state::get_state;
use crate::utils::file_exists;
use crate::requests::AudioPlayRequest;

pub fn play_audio_file(file: &str, loop_audio: bool) {
    // check if we already have an audio file playing
    if get_state() == PlayerState::Playing {
        warn!("Not playing audio file, audio playback already in progress!");
        return;
    }
    let sl = Soloud::default().unwrap();
    let mut wav = audio::Wav::default();
    info!("Loading audio data into memory");
    wav.load(&std::path::Path::new(file)).expect("Failed to load audio data into memory");
    info!("Looping audio: {}", loop_audio);
    wav.set_looping(loop_audio);
    info!("Starting audio playback for file: {}", file);
    sl.play(&wav);
    set_state(PlayerState::Playing);
    while sl.voice_count() > 0 {
        if Path::new("/tmp/rapd.stop_player_thread").exists() {
            info!("File rapd.stop_player_thread exists, shutting down player thread");
            std::fs::remove_file("/tmp/rapd.stop_player_thread").unwrap();
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    set_state(PlayerState::Idle);
    info!("Audio playback completed for file: {}", file);
}

pub fn play_audio_from_request(request: AudioPlayRequest) -> AudioStartStatus {
    info!("Spawning audio thread...");    
    if !file_exists(request.audio_file_path.clone()) {
       error!("Attempted to play a nonexistent audio file at path: {}", request.audio_file_path);
       return AudioStartStatus::FSError;
    }
    match Builder::new().name("player".to_string()).spawn(move || play_audio_file(&request.audio_file_path, request.loop_audio)){
        Ok(_) => {
            info!("Spawned audio thread");
            return AudioStartStatus::Success;
        },
        Err(err) => {
            error!("Failed to spawn audio thread: {}", err);
            return AudioStartStatus::ThreadingError;
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
