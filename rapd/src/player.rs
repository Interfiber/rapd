use soloud::*;
use std::fs::read_link;
use std::thread::Builder;
use crate::db::get_current_file_symlink_location;
use crate::utils::remove_current_symlink;
use crate::enums::AudioStartStatus;
use crate::enums::PlayerState;
use crate::state::set_state;
use crate::state::get_state;
use std::os::unix::fs::symlink;
use crate::utils::file_exists;
use crate::requests::AudioPlayRequest;

// play an audio file
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
    debug!("Creating symlinks...");
    match symlink(file, get_current_file_symlink_location()){
        Ok(_) => print!(""),
        Err(err) => {
            error!("Failed to symlink current file, this will prevent programs from getting the current playing file");
            error!("Error log: {}", err);
        }
    }
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
    remove_current_symlink(); 
    info!("Audio playback completed for file: {}", file);
    set_state(PlayerState::Idle);
}

// play an audio file from request
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
// stop the player
pub fn stop_player() {
    warn!("Stopping player on request");
    // set the state to be stopped
    set_state(PlayerState::Stop);
}

// get the current file playing
pub fn get_current_playing_file(full_path: bool) -> String {
    let symlink_location = get_current_file_symlink_location();
    if file_exists(symlink_location.clone()) {
       match read_link(symlink_location.clone()) {
            Ok(result) => {
                if full_path {
                    return result.into_os_string().into_string().expect("Failed to convert pathbuf to string").replace("\"", "");
                } else {
                    let path = result.as_path().file_name().expect("Faile to get filename");
                    return path.to_os_string().into_string().expect("Failed to convert to string");
                }
            },
            Err(err) => {
               error!("Failed to read current symlink");
               error!("Error log: {}", err); 
               return String::from("");
            }
       };
    } else {
        return String::from("empty");
    }
}
