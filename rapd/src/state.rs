use crate::enums::PlayerState;
use std::io::Write;

// convert a statestring(example: playerstate.idle) to a PlayerState
pub fn state_string_to_state(state_string: String) -> Option<PlayerState> {
    match state_string.as_str() {
        "playerstate.playing" => {
            return Some(PlayerState::Playing);
        }
        "playerstate.idle" => {
            return Some(PlayerState::Idle);
        }
        "playerstate.killed" => {
            return Some(PlayerState::Killed);
        }
        "playerstate.stop" => {
            return Some(PlayerState::Stop);
        }
        "playerstate.rebuilding" => {
            return Some(PlayerState::Rebuilding);
        }
        "playerstate.paused" => {
            return Some(PlayerState::Paused);
        }
        "playerstate.unpaused" => {
            return Some(PlayerState::Unpaused);
        }
        _ => {
            error!("statefile is empty, or corrupted!");
            return None;
        }
    }
}

// convert a PlayerState to a state string(example: playerstate.idle)
pub fn state_to_string(state: PlayerState) -> String {
    match state {
        PlayerState::Idle => {
            debug!("State match is: playerstate.idle");
            return String::from("playerstate.idle");
        }
        PlayerState::Rebuilding => {
            debug!("State match is: playerstate.rebuilding");
            return String::from("playerstate.rebuilding");
        }
        PlayerState::Playing => {
            debug!("State match is: playerstate.playing");
            return String::from("playerstate.playing");
        }
        PlayerState::Killed => {
            debug!("State match is: playerstate.killed");
            return String::from("playerstate.killed");
        }
        PlayerState::Stop => {
            debug!("State match is playerstate.stop");
            return String::from("playerstate.stop");
        }
        PlayerState::Paused => {
            debug!("State match is playerstate.paused");
            return String::from("playerstate.paused");
        }
        PlayerState::Unpaused => {
            debug!("State match is playerstate.unpaused");
            return String::from("playerstate.unpaused");
        }
    }
}

// Return the path to the statefile
pub fn get_state_path() -> String {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("rapd").unwrap();
    let mut data_path = xdg_dirs.get_data_home();
    data_path.push("statefile");
    return data_path
        .into_os_string()
        .into_string()
        .expect("Failed to convert to string");
}

// get the current player state
pub fn get_state() -> PlayerState {
    let state_path = get_state_path();
    let state_string = std::fs::read_to_string(state_path).expect("Failed to read from statefile");
    let state = state_string_to_state(state_string).unwrap();
    return state;
}

// set the player state
pub fn set_state(state: PlayerState) {
    info!("Setting state...");
    let state_path = get_state_path();
    let state_string = state_to_string(state);
    // switch the state
    let mut state_file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(state_path)
        .expect("Failed to open state file");
    match state_file.write_all(state_string.as_bytes()) {
        Ok(_) => debug!("Wrote state"),
        Err(err) => {
            error!("Failed to write state");
            error!("Error log: {}", err)
        }
    }
    // write to disk
    match state_file.flush() {
        Ok(_) => info!("Set state"),
        Err(err) => {
            error!("Failed to set state");
            error!("Error log: {}", err);
        }
    }
}
