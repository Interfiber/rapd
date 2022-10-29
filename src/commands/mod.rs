use std::path::Path;

use crate::state::PLAYER;
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

            channel
                .send(format!("play_file:{},{}", msg.params[0], msg.params[1]))
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

        if player.get_state() == &PlayerState::Playing {
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

// end section: Command impl
