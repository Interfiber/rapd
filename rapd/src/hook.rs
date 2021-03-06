use crate::db;
use crate::enums::{HookAddState, HookType};
use crate::requests::HookAddRequest;
use serde_json::Value;
use std::process::Command;

fn hook_type_to_hook(hook_type: String) -> HookType {
    match hook_type.as_str() {
        "player_start" => {
            return HookType::PlayerStart;
        }
        "player_pause" => {
            return HookType::PlayerPause;
        }
        "player_unpause" => {
            return HookType::PlayerUnpause;
        }
        "server_shutdown" => {
            return HookType::ServerShutdown;
        }
        _ => {
            error!("Invalid hook");
            return HookType::Unknown;
        }
    }
}

fn add_hook_db(name: String, command: String) -> HookAddState {
    let db_file = db::get_db_path();
    info!("Database file path is {}", db_file);
    // read data from the db file
    // TODO: Error checking
    let db_raw = std::fs::read_to_string(db_file.to_string()).expect("Failed to database json");
    let mut db_content: Value =
        serde_json::from_str(&db_raw).expect("Failed to parse database json!");
    info!("Updating database...");
    db_content[format!("hook_{}", name)] = serde_json::Value::String(command);
    info!("Writing new database to disk...");
    match std::fs::write(db_file, db_content.to_string()) {
        Ok(_) => {
            info!("Added hook to database");
            return HookAddState::Added;
        }
        Err(err) => {
            error!("Failed to add hook due to a file system error!");
            error!("Error log: {}", err);
            return HookAddState::FsError;
        }
    }
}

pub fn add_hook(hook_request: HookAddRequest) -> HookAddState {
    info!("Adding hook with type: {}", hook_request.hook_type);
    let hook = hook_type_to_hook(hook_request.hook_type.to_string());
    if hook == HookType::Unknown {
        error!("Invalid hook type!");
        return HookAddState::InvalidHookType;
    }
    return add_hook_db(hook_request.hook_type, hook_request.command);
}

fn run_command(cmd: String) {
    Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("Failed to run hook command!");
}

pub fn fire_hook(hook_type: HookType) {
    // parse json db
    let db_file = db::get_db_path();
    let db_raw = std::fs::read_to_string(db_file.to_string()).expect("Failed to database json");
    let db_content: Value = serde_json::from_str(&db_raw).expect("Failed to parse database json!");
    match hook_type {
        HookType::PlayerStart => {
            if db_content.get("hook_player_start").is_none() {
                warn!("No such hook");
                return;
            } else {
                let cmd = db_content["hook_player_start"]
                    .to_string()
                    .replace("\"", "");
                info!("Running hook command: {}", cmd);
                run_command(cmd);
            }
        }
        HookType::ServerShutdown => {
            if db_content.get("hook_server_shutdown").is_none() {
                warn!("No such hook!");
                return;
            } else {
                let cmd = db_content["hook_server_shutdown"]
                    .to_string()
                    .replace("\"", "");
                info!("Running hook command: {}", cmd);
                run_command(cmd);
            }
        }
        HookType::PlayerPause => {
            if db_content.get("hook_player_pause").is_none() {
                warn!("No such hook!");
                return;
            } else {
                let cmd = db_content["hook_player_pause"]
                    .to_string()
                    .replace("\"", "");
                info!("Running hook command: {}", cmd);
                run_command(cmd);
            }
        }
        HookType::PlayerUnpause => {
            if db_content.get("hook_player_unpause").is_none() {
                warn!("No such hook");
                return;
            } else {
                let cmd = db_content["hook_player_unpause"]
                    .to_string()
                    .replace("\"", "");
                info!("Running hook command: {}", cmd);
                run_command(cmd);
            }
        }
        _ => unreachable!(),
    }
}
