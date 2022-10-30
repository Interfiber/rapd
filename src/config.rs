use crate::state::CONFIG;

pub struct RapdConfig {
    notifications_enabled: bool,
    music_dir: String
}

impl RapdConfig {

    /// Creates a new RapdConfig, with default values
    pub fn new() -> RapdConfig {
       RapdConfig { 
           notifications_enabled: false,
           music_dir: String::new()
       } 
    }

    /// Returns if notifications are enabled
    pub fn notifications(&self) -> bool {
        self.notifications_enabled
    }

    /// Returns the music directory
    pub fn music_directory(&self) -> String {
        self.music_dir.clone()
    }

    /// Set the music dir
    pub fn set_music_dir(&mut self, v: String) {
        self.music_dir = v;
    }

    /// Set if notifications are enabled
    pub fn set_notifications(&mut self, v: bool){
        self.notifications_enabled = v;
    }
}

pub fn set_value(key: &str, value: String) {

    match key {
        "notifications_enabled" => {
            CONFIG.lock().set_notifications(value.parse::<bool>().expect("Invalid boolean value"));
        },
        "music_dir" => {
            CONFIG.lock().set_music_dir(value.to_string());
        }
        _ => error!("Invalid config value key")
    }

    info!("Config key {} is now set to {}", key, value.to_string());
}
