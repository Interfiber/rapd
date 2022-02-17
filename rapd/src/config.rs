use toml::Value;

// get the config path
pub fn get_config_path() -> String {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("rapd").unwrap();
    let mut config_dir = xdg_dirs.get_config_home();
    config_dir.push("config.toml");
    return config_dir.into_os_string().into_string().unwrap();
}

// get the config Value
pub fn get_config() -> Value {
    // read from the config file
    let config_path = get_config_path();
    let config_data =
        std::fs::read_to_string(config_path).expect("Failed to read from config file");
    let config = config_data.parse::<Value>().unwrap();
    return config;
}
