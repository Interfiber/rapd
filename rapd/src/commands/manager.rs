use std::collections::HashMap;

use serde_json::json;

use crate::json::{RapdCommandResponse, RapdMessage};

use super::{
    AddFileToPlaylistCommand, CreatePlaylistCommand, GetFileCommand, GetFilesInPlaylistCommand,
    GetLengthCommand, GetMetadataCommand, GetMusicFilesCommand, GetPlaylistsCommand,
    GetStateCommand, GetTimeCommand, PingCommand, PlayFileCommand, RapdCommand,
    RebuildDatabaseCommand, RemoveFileFromPlaylistCommand, SetConfigValueCommand, StopCommand,
    TogglePauseCommand,
};

pub struct RapdCommandManager {
    // command name, command
    commands: HashMap<String, Box<dyn RapdCommand>>,
}

impl RapdCommandManager {
    /// Create a new command manager, no commands are added by default
    pub fn new() -> RapdCommandManager {
        RapdCommandManager {
            commands: HashMap::new(),
        }
    }

    /// Add a command to the command manager
    pub fn add_cmd<T>(&mut self, name: &str, cmd: T)
    where
        T: RapdCommand + 'static,
    {
        self.commands.insert(String::from(name), Box::new(cmd));
    }

    /// Process a command issued to the server
    pub fn handle_command(&self, message: RapdMessage) -> RapdCommandResponse {
        let cmd = self.commands.get(&message.command);

        if let Some(command) = cmd {
            command.execute(message)
        } else {
            error!("Invalid command issued to server!");
            error!("{:#?}", message);
            RapdCommandResponse::new(json!("Invalid command!"), true)
        }
    }
}

/// Adds all default commands to a RapdCommandManager
pub fn init_manager(manager: &mut RapdCommandManager) {
    manager.add_cmd("ping", PingCommand {});
    manager.add_cmd("play_file", PlayFileCommand {});
    manager.add_cmd("player_state", GetStateCommand {});
    manager.add_cmd("player_stop", StopCommand {});
    manager.add_cmd("player_toggle_pause", TogglePauseCommand {});
    manager.add_cmd("player_length", GetLengthCommand {});
    manager.add_cmd("player_time", GetTimeCommand {});
    manager.add_cmd("player_file", GetFileCommand {});
    manager.add_cmd("player_get_metadata", GetMetadataCommand {});
    manager.add_cmd("rebuild_database", RebuildDatabaseCommand {});
    manager.add_cmd("config_set", SetConfigValueCommand {});
    manager.add_cmd("db_get_files", GetMusicFilesCommand {});
    manager.add_cmd("db_create_playlist", CreatePlaylistCommand {});
    manager.add_cmd("db_add_file_to_playlist", AddFileToPlaylistCommand {});
    manager.add_cmd(
        "db_remove_file_from_playlist",
        RemoveFileFromPlaylistCommand {},
    );
    manager.add_cmd("db_get_playlists", GetPlaylistsCommand {});
    manager.add_cmd("db_get_files_in_playlist", GetFilesInPlaylistCommand {});
}
