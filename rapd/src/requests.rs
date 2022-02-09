use serde::{Deserialize, Serialize};
use crate::enums::*;

#[derive(Serialize, Deserialize)]
pub struct AudioPlayRequest { 
    pub request_type: String,
    pub audio_file_path: String,
    pub push_to_top: bool,
    pub title: String,
    pub author: String,
    pub playlist_id: i32
}


// functions
pub fn get_request_rejected_string(why: &str) -> String {
    return json!({
        "request_type": "Rejection",
        "error": true,
        "message": why
    }).to_string()
}
pub fn get_request_ok_string(why: &str) -> String {
    return json!({
        "request_type": "Succeeded",
        "error": false,
        "message": why
    }).to_string();
}
pub fn audio_play_status_request_string(result: AudioStartStatus) -> String {
    match result {
        AudioStartStatus::FSError => {
            return json!({
                "request_type": "Failed",
                "error": true,
                "message": "Failed to start player, due to a filesystem error. Make sure the file exists and is readable by rapd"
            }).to_string();
        },
        AudioStartStatus::Success => {
            return get_request_ok_string("Player will attempt audio playback");
        },
        AudioStartStatus::ThreadingError => {
            return json!({
                "request_type": "Failed",
                "error": true,
                "message": "Failed to start the player due to an internal threading error."
            }).to_string();
        }
    }
}
