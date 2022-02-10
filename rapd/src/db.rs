use std::fs::File;

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
pub fn create_db(){
    info!("Creating rapd database and config files");
    let xdg_dirs = xdg::BaseDirectories::with_prefix("rapd").unwrap();
    let db_path = xdg_dirs.place_data_file("db.json").expect("Failed to place database file");
    let state_path = xdg_dirs.place_data_file("statefile").expect("Failed to place state file");
    let config_path = xdg_dirs.place_config_file("config.toml").expect("Failed to place config file");
    info!("Database path: {}", db_path.as_path().display());
    info!("Config path: {}", config_path.as_path().display());
    info!("State path: {}", state_path.as_path().display());
    info!("Writing files to disk...");
    File::create(db_path).expect("Failed to write to db_file");
    File::create(config_path).expect("Failed to write to config_file");
    File::create(state_path).expect("Failed to create state file");
    info!("Created database and config files");
}
