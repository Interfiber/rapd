use serde::{Deserialize, Serialize};
use std::io::Read;
use std::io::Write;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use uuid::Uuid;

use crate::state::CONFIG;
use crate::state::DATABASE;

#[derive(Serialize, Deserialize)]
pub struct RapdDatabase {
    files: Vec<RapdAudioFile>,
    playlists: Vec<RapdPlaylist>,
    last_rebuild: String,
}

#[derive(Serialize, Deserialize)]
pub struct RapdAudioFile {
    file: String,
    id: String,
}

#[derive(Serialize, Deserialize)]
pub struct RapdPlaylist {
    files: Vec<String>, // list of file UUID's in that playlist
    create_date: i32,
    playlist_name: String,
    playlist_desc: String,
}

impl RapdDatabase {
    /// Read the rapd database from a path
    pub fn read(&mut self, path: String) {
        info!("Reading rapd database from path: {}", path);

        // read file
        trace!("Reading database from disk");

        let mut file = std::fs::File::open(path).expect("Failed to open database!");
        let mut db_txt = String::new();

        file.read_to_string(&mut db_txt)
            .expect("Failed to read from database file");

        let db: RapdDatabase = serde_json::from_str(&db_txt).expect("Failed to parse database");

        self.files = db.files;
        self.playlists = db.playlists;
        self.last_rebuild = db.last_rebuild;
    }

    /// Creates an empty database
    pub fn empty() -> RapdDatabase {
        RapdDatabase {
            files: Vec::new(),
            playlists: Vec::new(),
            last_rebuild: String::from("0"),
        }
    }

    /// Creates a json readable format of the database
    pub fn dump(&mut self) -> String {
        let time = SystemTime::now();
        let dist = time
            .duration_since(UNIX_EPOCH)
            .expect("Failed to calculate time since UNIX epoch")
            .as_millis();

        self.last_rebuild = dist.to_string();

        serde_json::to_string_pretty(self).expect("Failed to dump self into json")
    }

    /// Adds an audio file to the database
    pub fn add_file(&mut self, file: String) {
        info!("Adding file to database: {}", file);

        let id = Uuid::new_v4().to_string();

        let f = RapdAudioFile { file, id };

        self.files.push(f);
    }


    /// Removes all files from the database
    pub fn clear_files(&mut self){
        self.files.clear();
    }

    /// Get files in database
    pub fn get_files(&self) -> &Vec<RapdAudioFile> {
        &self.files
    }
}

/// Gets the location of the rapd database file
pub fn get_db_file() -> String {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("rapd").unwrap();

    if xdg_dirs.find_data_file("database").is_none() {
        info!("No database file found, creating new one");
        let config_path = xdg_dirs
            .place_data_file("database")
            .expect("cannot create configuration directory");
        let mut config_file = std::fs::File::create(config_path).expect("Failed to create db file");
        write!(&mut config_file, "{}", RapdDatabase::empty().dump())
            .expect("Failed to write to config file");
    }

    return xdg_dirs
        .get_data_file("database")
        .as_os_str()
        .to_os_string()
        .to_str()
        .unwrap()
        .to_string();
}

pub fn load_db() {
    let db_file = get_db_file();

    DATABASE.lock().read(db_file);
}

pub fn save_db() {
    info!("Writing database to disk");
    let db_file = get_db_file();
    let dump = DATABASE.lock().dump();

    let mut file = std::fs::File::create(db_file).expect("Failed to open database file");
    write!(&mut file, "{}", dump).expect("Failed to write to database");

    info!("Wrote database to disk");
}

pub fn rebuild_db() {
    info!("Clearing database of files");
    DATABASE.lock().clear_files();

    info!("Scanning files in music directory");
    let music_dir = CONFIG.lock().music_directory();

    let paths = std::fs::read_dir(music_dir).unwrap();

    for path in paths {
        let file = path.unwrap().path();

        if !file.is_dir() {
            let path_name = file.as_os_str().to_str().unwrap().to_string();

            DATABASE.lock().add_file(path_name);
        }
    }

    info!("Rebuilt database, dumping to disk");
    save_db();
}
