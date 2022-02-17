use crate::db::get_current_file_symlink_location;
use crate::enums;
use crate::state;
use std::path::Path;

pub fn file_exists(path: String) -> bool {
    if Path::new(&path).exists() {
        return true;
    } else {
        return false;
    }
}

pub fn get_default_music_dir() -> String {
    // the default music dir is ~/Audio
    let mut audio_dir = home::home_dir().unwrap();
    audio_dir.push("Audio");
    return audio_dir.into_os_string().into_string().unwrap();
}

pub fn is_directory(path: String) -> bool {
    return Path::new(&path).is_dir();
}

pub fn remove_current_symlink() {
    let location = get_current_file_symlink_location();
    if !file_exists(location) {
        warn!("Failed to remove symlink, no such file or directory");
    } else {
        match std::fs::remove_file(get_current_file_symlink_location()) {
            Ok(_) => print!(""),
            Err(err) => {
                error!("Failed to remove current file symlink!");
                error!("Error log: {}", err)
            }
        }
    }
}

pub fn shutdown() {
    state::set_state(enums::PlayerState::Killed);
    info!("Removing symlink...");
    remove_current_symlink();
    info!("Exiting");
    std::process::exit(1);
}
