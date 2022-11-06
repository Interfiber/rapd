use std::path::Path;

use crate::{
    database::{save_db, RapdPlaylist},
    metadata::RapdMetadata,
    state::{DATABASE, PLAYER},
};
use serde_json::json;

use crate::{
    json::{RapdCommandResponse, RapdMessage},
    player::PlayerState,
    state::PLAYER_SENDER,
};

// modules
pub mod manager;

pub trait RapdCommand {
    fn execute(&self, msg: RapdMessage) -> RapdCommandResponse;
}

// start section: Commands
pub struct PingCommand {}
pub struct PlayFileCommand {}
pub struct GetStateCommand {}
pub struct GetTimeCommand {}
pub struct StopCommand {}
pub struct TogglePauseCommand {}
pub struct GetLengthCommand {}
pub struct GetFileCommand {}
pub struct GetMetadataCommand {}
pub struct RebuildDatabaseCommand {}
pub struct SetConfigValueCommand {}
pub struct GetMusicFilesCommand {}
pub struct CreatePlaylistCommand {}
pub struct AddFileToPlaylistCommand {}
pub struct RemoveFileFromPlaylistCommand {}
pub struct GetPlaylistsCommand {}
pub struct GetFilesInPlaylistCommand {}
pub struct RemovePlaylistCommand {}

// end section: Commands

// start section: Command impl

impl RapdCommand for PingCommand {
    fn execute(&self, _msg: RapdMessage) -> RapdCommandResponse {
        RapdCommandResponse::new(json!("RAPD server is up! pong"), false)
    }
}

impl RapdCommand for PlayFileCommand {
    fn execute(&self, msg: RapdMessage) -> RapdCommandResponse {
        // check parms
        if msg.params.len() == 2 {
            let sender = PLAYER_SENDER.lock();
            let channel = sender.sender.as_ref().unwrap();

            if !Path::new(&msg.params[0]).exists() {
                error!("File {} does not exist", msg.params[0]);
                return RapdCommandResponse::new(
                    json!(format!("File {} does not exist", msg.params[0])),
                    true,
                );
            }

            let player = PLAYER.lock();
            let player_state = player.get_state();
            if player_state == PlayerState::Playing {
                channel.send(String::from("stop_player:_")).unwrap();
            }

            let file_safe = msg.params[0].replace(',', "\\COMMA");
            channel
                .send(format!("play_file:{},{}", file_safe, msg.params[1]))
                .unwrap();

            RapdCommandResponse::new(json!("Starting audio playback"), false)
        } else {
            RapdCommandResponse::new(
                json!("This command takes two params: FILE, and SHOULD_LOOP"),
                true,
            )
        }
    }
}

impl RapdCommand for GetStateCommand {
    fn execute(&self, _msg: RapdMessage) -> RapdCommandResponse {
        let state = PLAYER.lock().get_state().serialize();
        RapdCommandResponse::new(json!(state), false)
    }
}

impl RapdCommand for GetTimeCommand {
    fn execute(&self, _msg: RapdMessage) -> RapdCommandResponse {
        let player = PLAYER.lock();
        RapdCommandResponse::new(json!(player.get_time()), false)
    }
}

impl RapdCommand for StopCommand {
    fn execute(&self, _msg: RapdMessage) -> RapdCommandResponse {
        let sender = PLAYER_SENDER.lock();
        let channel = sender.sender.as_ref().unwrap();
        channel.send(String::from("stop_player:_")).unwrap();
        RapdCommandResponse::new(json!("Stopped player"), false)
    }
}

impl RapdCommand for TogglePauseCommand {
    fn execute(&self, _msg: RapdMessage) -> RapdCommandResponse {
        let sender = PLAYER_SENDER.lock();
        let channel = sender.sender.as_ref().unwrap();
        let player = PLAYER.lock();

        #[allow(unused_assignments)]
        let mut paused = false;

        if player.get_state() == PlayerState::Playing {
            channel.send(String::from("pause_player:_")).unwrap();
            paused = true;
        } else {
            channel.send(String::from("unpause_player:_")).unwrap();
            paused = false;
        }
        RapdCommandResponse::new(
            json!(format!("Toggled pause for player, is_paused = {}", paused)),
            false,
        )
    }
}

impl RapdCommand for GetLengthCommand {
    fn execute(&self, _msg: RapdMessage) -> RapdCommandResponse {
        let player = PLAYER.lock();
        let len = player.get_length();

        RapdCommandResponse::new(json!(len), false)
    }
}

impl RapdCommand for GetFileCommand {
    fn execute(&self, _msg: RapdMessage) -> RapdCommandResponse {
        let player = PLAYER.lock();
        let file = player.get_file();

        RapdCommandResponse::new(json!(file), false)
    }
}

impl RapdCommand for GetMetadataCommand {
    fn execute(&self, msg: RapdMessage) -> RapdCommandResponse {
        if msg.params.is_empty() {
            let player = PLAYER.lock();
            let metadata = player.get_metadata();

            RapdCommandResponse::new(json!(metadata), false)
        } else if msg.params.len() == 1 {
            let mut metadata = RapdMetadata::new(msg.params[0].clone());
            metadata.open(); // Read metadata

            RapdCommandResponse::new(json!(metadata), false)
        } else {
            RapdCommandResponse::new(json!("Too many arguments, expected 0, or 1."), true)
        }
    }
}

impl RapdCommand for RebuildDatabaseCommand {
    fn execute(&self, _msg: RapdMessage) -> RapdCommandResponse {
        info!("Rebuilding database...");
        crate::database::rebuild_db();

        RapdCommandResponse::new(json!("JSON music database rebuilt"), false)
    }
}

impl RapdCommand for SetConfigValueCommand {
    fn execute(&self, msg: RapdMessage) -> RapdCommandResponse {
        if msg.params.len() == 2 {
            let key = &msg.params[0];
            let val = &msg.params[1];

            crate::config::set_value(key, String::from(val));

            RapdCommandResponse::new(json!("Set config value"), false)
        } else {
            RapdCommandResponse::new(json!("This command takes two params: KEY, VALUE"), true)
        }
    }
}

impl RapdCommand for GetMusicFilesCommand {
    fn execute(&self, _msg: RapdMessage) -> RapdCommandResponse {
        let db = DATABASE.lock();

        RapdCommandResponse::new(json!(db.get_files()), false)
    }
}

impl RapdCommand for CreatePlaylistCommand {
    fn execute(&self, msg: RapdMessage) -> RapdCommandResponse {
        if msg.params.len() == 2 {
            DATABASE.lock().add_playlist(RapdPlaylist::new(
                msg.params[0].to_owned(),
                msg.params[1].to_owned(),
            ));
            save_db();

            RapdCommandResponse::new(json!("Added new playlist to database"), false)
        } else {
            RapdCommandResponse::new(json!("This command takes two params: NAME, DESC"), true)
        }
    }
}

impl RapdCommand for AddFileToPlaylistCommand {
    fn execute(&self, msg: RapdMessage) -> RapdCommandResponse {
        if msg.params.len() == 2 {
            DATABASE
                .lock()
                .add_file_to_playlist(msg.params[0].to_owned(), msg.params[1].to_owned());
            save_db();

            RapdCommandResponse::new(json!("Updated playlist"), false)
        } else {
            RapdCommandResponse::new(json!("This command takes two params: TARGET, FILE"), true)
        }
    }
}

impl RapdCommand for RemoveFileFromPlaylistCommand {
    fn execute(&self, msg: RapdMessage) -> RapdCommandResponse {
        if msg.params.len() == 2 {
            DATABASE
                .lock()
                .remove_file_from_playlist(msg.params[0].to_owned(), msg.params[1].to_owned());
            save_db();

            RapdCommandResponse::new(json!("Updated playlist"), false)
        } else {
            RapdCommandResponse::new(json!("This command takes two params: TARGET, FILE"), true)
        }
    }
}

impl RapdCommand for GetPlaylistsCommand {
    fn execute(&self, _msg: RapdMessage) -> RapdCommandResponse {
        let db = DATABASE.lock();
        let playlists = db.get_playlists();
        let mut result = vec![];

        for playlist in playlists.iter() {
            result.push(playlist.playlist_name.clone());
        }

        RapdCommandResponse::new(json!(result), false)
    }
}

impl RapdCommand for GetFilesInPlaylistCommand {
    fn execute(&self, msg: RapdMessage) -> RapdCommandResponse {
        if msg.params.len() == 1 {
            let db = DATABASE.lock();
            let list = db.get_playlist(msg.params[0].clone());

            if list.is_none() {
                RapdCommandResponse::new(json!("No such playlist"), true)
            } else {
                RapdCommandResponse::new(json!(list.unwrap()), false)
            }
        } else {
            RapdCommandResponse::new(json!("This command takes one param: NAME"), true)
        }
    }
}

impl RapdCommand for RemovePlaylistCommand {
    fn execute(&self, msg: RapdMessage) -> RapdCommandResponse {
        if msg.params.len() == 1 {
            let name = &msg.params[0];

            DATABASE.lock().remove_playlist(name.clone());
            save_db();
            info!("Removed playlist: {}", name);
            RapdCommandResponse::new(json!("Removed playlist"), false)
        } else {
            RapdCommandResponse::new(json!("This command takes one param: NAME"), true)
        }
    }
}

// end section: Command impl
