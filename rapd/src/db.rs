use crate::config::get_config;
use crate::enums::MusicDatabaseRebuildState;
use crate::enums::PlayerState;
use crate::state::get_state;
use crate::state::set_state;
use crate::state::state_to_string;
use crate::utils::{file_exists, get_default_music_dir, is_directory};
use std::fs;
use std::fs::File;
use std::io::Write;

// check if the database exists
pub fn db_exists() -> bool {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("rapd").unwrap();
    let config_dir = xdg_dirs.get_config_home();
    let data_dir = xdg_dirs.get_data_home();
    if config_dir.exists() && data_dir.exists() {
        return true;
    } else {
        return false;
    }
}

// get the db path
pub fn get_db_path() -> String {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("rapd").unwrap();
    let mut data_dir = xdg_dirs.get_data_home();
    data_dir.push("db.json");
    return data_dir.into_os_string().into_string().unwrap();
}

// create config files & database files
pub fn create_db() {
    info!("Creating rapd database and config files");
    let xdg_dirs = xdg::BaseDirectories::with_prefix("rapd").unwrap();
    let db_path = xdg_dirs
        .place_data_file("db.json")
        .expect("Failed to place database file");
    let state_path = xdg_dirs
        .place_data_file("statefile")
        .expect("Failed to place state file");
    let config_path = xdg_dirs
        .place_config_file("config.toml")
        .expect("Failed to place config file");
    info!("Database path: {}", db_path.as_path().display());
    info!("Config path: {}", config_path.as_path().display());
    info!("State path: {}", state_path.as_path().display());
    info!("Writing files to disk...");
    File::create(db_path.clone()).expect("Failed to write to db_file");
    File::create(config_path).expect("Failed to write to config_file");
    File::create(state_path.clone()).expect("Failed to create state file");
    // also write the default value to the statefile
    let default_state = state_to_string(PlayerState::Idle);
    match std::fs::write(state_path.as_path().display().to_string(), default_state) {
        Ok(_) => info!("Wrote default state"),
        Err(err) => {
            error!("Failed to write default state!");
            error!("Error log: {}", err);
        }
    }
    // init the database
    let base_db = json!({}).to_string();
    match std::fs::write(db_path, base_db) {
        Ok(_) => print!(""),
        Err(err) => {
            error!("Failed to init db!");
            error!("Error log: {}", err);
        }
    }
    info!("Created database and config files");
}
pub fn rebuild_music_database() -> MusicDatabaseRebuildState {
    warn!("Rebuilding the music database, this make take some time.");
    if get_state() == PlayerState::Playing {
        error!("Cant rebuild database while player is active");
        return MusicDatabaseRebuildState::PlayerRunning;
    }
    // find the music dir, in the config file or use the default
    let config = get_config();
    let config_raw = config.get("configuration");
    let configuration: &toml::Value;
    let music_dir: String;
    if config_raw.is_none() {
        error!("Configuration error: configuration is null!");
        return MusicDatabaseRebuildState::ConfigError;
    } else {
        configuration = config_raw.unwrap();
    }
    if configuration.get("music_dir").is_none() {
        warn!("Music dir not specified in config file, using default");
        music_dir = get_default_music_dir();
        info!("Music dir is: {}", music_dir);
    } else {
        music_dir = configuration
            .get("music_dir")
            .unwrap()
            .to_string()
            .replace("\"", "");
        info!("Music dir is: {}", music_dir);
    }
    if !file_exists(music_dir.clone()) {
        error!("Music directory does not exist! Make sure the path given is the absolute path");
        return MusicDatabaseRebuildState::FSError;
    }
    // index all of the files in the music folder
    info!("Indexing top-level items in the music directory...");
    set_state(PlayerState::Rebuilding);
    // loop through every file in the music dir
    let music_items = fs::read_dir(music_dir).unwrap();

    let mut music_files = Vec::new();
    for path in music_items {
        let path_string = path.unwrap().path().display().to_string();
        info!("Found item at path: {}", path_string);
        // check if the item is a directory
        if is_directory(path_string.clone()) {
            warn!("Skipping item, item is a directory");
        } else {
            info!("Adding file to list");
            music_files.push(path_string);
        }
    }
    info!("Collected files, updating database");
    let music_array = json!(music_files);
    // update the music database
    let db_path = get_db_path();
    let db_raw =
        std::fs::read_to_string(db_path.clone()).expect("Failed to read from database path");
    let mut db_json: serde_json::Value =
        serde_json::from_str(&db_raw).expect("Failed to parse json");
    let db = db_json.as_object_mut().expect("Failed to borrow");
    db.insert(String::from("music"), music_array);
    let db_json_raw = serde_json::to_string(db).expect("Failed to convert database to json");
    info!("Writing database");
    let mut db_file = std::fs::OpenOptions::new()
        .truncate(true)
        .write(true)
        .open(db_path.clone())
        .expect("Failed to open db");
    match db_file.write_all(db_json_raw.as_bytes()) {
        Ok(_) => print!(""),
        Err(err) => {
            error!("Failed to write database!");
            error!("Error log: {}", err);
            set_state(PlayerState::Idle);
            return MusicDatabaseRebuildState::DatabaseWriteError;
        }
    }
    db_file.flush().expect("Failed to flush database to disk");
    set_state(PlayerState::Idle);
    info!("Rebuilt, wiping current symlink if it exists");
    crate::utils::remove_current_symlink();
    return MusicDatabaseRebuildState::Rebuilt;
}

// get all music from the database
pub fn get_music() -> serde_json::Value {
    let db_path = get_db_path();
    let db_raw = std::fs::read_to_string(db_path).expect("Failed to read from database path");
    let db_json: serde_json::Value = serde_json::from_str(&db_raw).expect("Failed to parse json");
    let music = db_json.get("music");
    if music.is_none() {
        warn!("Music database is empty, returning empty");
        return json!([]);
    } else {
        return music.unwrap().to_owned();
    }
}

// get the location of the currentfile.symlink
pub fn get_current_file_symlink_location() -> String {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("rapd").unwrap();
    let mut data_dir = xdg_dirs.get_data_home();
    // push the file into the pathbuf
    data_dir.push("currentfile.symlink");
    return data_dir
        .into_os_string()
        .into_string()
        .expect("Failed to convert to string");
}
