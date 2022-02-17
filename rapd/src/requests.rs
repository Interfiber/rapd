use crate::enums::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AudioPlayRequest {
    pub request_type: String,
    pub audio_file_path: String,
    pub loop_audio: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CurrentFileRequest {
    pub request_type: String,
    pub full_path: bool,
}

// functions
pub fn get_request_rejected_string(why: &str) -> String {
    return json!({
        "request_type": "Rejection",
        "error": true,
        "message": why
    })
    .to_string();
}
pub fn get_request_ok_string(why: &str) -> String {
    return json!({
        "request_type": "Succeeded",
        "error": false,
        "message": why
    })
    .to_string();
}
pub fn audio_play_status_request_string(result: AudioStartStatus) -> String {
    match result {
        AudioStartStatus::FSError => {
            return json!({
                "request_type": "Failed",
                "error": true,
                "message": "Failed to start player, due to a filesystem error. Make sure the file exists and is readable by rapd"
            }).to_string();
        }
        AudioStartStatus::Success => {
            return get_request_ok_string("Player will attempt audio playback");
        }
        AudioStartStatus::ThreadingError => {
            return json!({
                "request_type": "Failed",
                "error": true,
                "message": "Failed to start the player due to an internal threading error."
            })
            .to_string();
        }
    }
}

pub fn current_file_request_string(path: String) -> String {
    return json!({
         "request_type": "Succeeded",
         "error": false,
         "message": path
    })
    .to_string();
}

pub fn db_rebuild_status_request_string(result: MusicDatabaseRebuildState) -> String {
    match result {
        MusicDatabaseRebuildState::ConfigError => {
            return json!({
                "request_type": "Failed",
                "error": true,
                "message": "Failed to rebuild the database due to a config error"
            })
            .to_string();
        }
        MusicDatabaseRebuildState::DatabaseWriteError => {
            return json!({
                "request_type": "Failed",
                "error": true,
                "message": "The database was rebuilt but was unable to be written to disk"
            })
            .to_string();
        }
        MusicDatabaseRebuildState::FSError => {
            return json!({
                "request_type": "Failed",
                "error": true,
                "message": "Failed to rebuild the database due to a filesystem error"
            })
            .to_string();
        }
        MusicDatabaseRebuildState::Rebuilt => {
            return json!({
                "request_type": "Failed",
                "error": true,
                "message": "Rebuilt the music database"
            })
            .to_string();
        }
        MusicDatabaseRebuildState::PlayerRunning => {
            return json!({
                "request_type": "Failed",
                "error": true,
                "message": "The player MUST be stopped while the music database is being rebuilt"
            })
            .to_string();
        }
    }
}
