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
    let mut tick_since_state_recheck = 0;
    while sl.voice_count() > 0 {
        tick_since_state_recheck += 1;
        if tick_since_state_recheck >= 3 {
            // re-evaluate the player state
            if get_state() == PlayerState::Stop {
                info!("Got stop request in statefile, stopping player");
                break;
            }
            tick_since_state_recheck = 0;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    info!("Audio playback completed for file: {}", file);
    set_state(PlayerState::Idle);
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
    set_state(PlayerState::Stop);
}
