use serde::{Deserialize, Serialize};
use std::io::Write;
use crate::enums::PlayerState;

#[derive(Serialize, Deserialize)]
pub struct StateFile {
    pub state: String
}

pub fn get_state_path() -> String {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("rapd").unwrap();
    let mut data_path = xdg_dirs.get_data_home();
    data_path.push("statefile");
    return data_path.into_os_string().into_string().expect("Failed to convert to string");
}

pub fn get_state() -> PlayerState {
    let state_path = get_state_path();
    let state_string = std::fs::read_to_string(state_path).expect("Failed to read from statefile");
    match state_string.as_str() {
        "playerstate.playing" => {
            return PlayerState::Playing;
        },
        "playerstate.idle" => {
            return PlayerState::Idle;
        },
        _ => {
            warn!("State file is empty, or corrupted using default value of idle");
            return PlayerState::Idle;
        }
    }
}

pub fn set_state(state: PlayerState){
    info!("Setting state...");
    let state_path = get_state_path();
    let mut state_string = "";
    debug!("Compiler warning removal line: {}", state_string);
    // switch the state 
    match state {
        PlayerState::Idle => {
            debug!("State match is: playerstate.idle");
            state_string = "playerstate.idle"
        },
        PlayerState::Playing => {
            debug!("State match is: playerstate.playing");
            state_string = "playerstate.playing";
        }
    }
    let mut state_file = std::fs::OpenOptions::new().write(true).open(state_path).expect("Failed to open state file");
    match state_file.write_all(state_string.as_bytes()){
        Ok(_) => debug!("Wrote state"),
        Err(err) => {
            error!("Failed to write state");
            error!("Error log: {}", err)
        }
    }
    // flush the state file
    match state_file.flush() {
        Ok(_) => info!("Set state"),
        Err(err) => {
            error!("Failed to set state");
            error!("Error log: {}", err);
        }
    }
}
