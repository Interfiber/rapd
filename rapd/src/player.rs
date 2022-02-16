use std::fs::read_link;
use std::thread::Builder;
use crate::db::get_current_file_symlink_location;
use crate::utils::remove_current_symlink;
use crate::enums::AudioStartStatus;
use crate::enums::PlayerState;
use crate::state::set_state;
use crate::state::get_state;
use std::os::unix::fs::symlink;
use std::time::Duration;
use std::thread;
use crate::utils::file_exists;
use crate::requests::AudioPlayRequest;
use std::io::BufReader;

// NOTE: Cleanup code after audio backend change

// play an audio file
pub fn play_audio_file(file: &str, loop_audio: bool) {
    // check if we already have an audio file playing
    if get_state() == PlayerState::Playing {
        warn!("Not playing audio file, audio playback already in progress!");
        return;
    }
    // create a rodio stream handler
    let (_stream, stream_handle) = rodio::OutputStream::try_default().expect("Failed to create an output stream");
    let audio_file = std::fs::File::open(file).expect("Failed to open file for reading" );
    let sound = stream_handle.play_once(BufReader::new(audio_file)).unwrap();
    sound.set_volume(0.2);
    info!("Starting audio playback for file: {}", file);
    // play the audio
    debug!("Creating symlinks...");
    // create symlinks
    match symlink(file, get_current_file_symlink_location()){
        Ok(_) => print!(""),
        Err(err) => {
            error!("Failed to symlink current file, this will prevent programs from getting the current playing file");
            error!("Error log: {}", err);
        }
    }

    // update the state
    set_state(PlayerState::Playing);
    // sound update loop
    let mut state_recheck_ticker = 0;
    loop {
        if sound.empty() {
            // the sound ended, break
            break;
        } else {
            state_recheck_ticker += 1;
            // check if we need to stop via getting the state
            if state_recheck_ticker >= 2 {
                if get_state() == PlayerState::Stop {
                    info!("Got stop request in state file, halting player");
                    sound.stop();
                    info!("Breaking...");
                    break;
                }
                state_recheck_ticker = 0;
            }
            info!("{}", state_recheck_ticker);
            thread::sleep(Duration::from_millis(500));
        }
    }
    // remove the symlinks
    remove_current_symlink(); 
    info!("Audio playback completed for file: {}", file);
   
    // update the state
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
