// Name: notify plugin
// Description: Notify you via the system notification daemon when something changes
// Author: Interfiber <webmaster@interfiber.dev>
use crate::config;
use crate::plugin_api::PluginApi;
use notify_rust::Notification;
use std::path::Path;

// construct our plugin
pub struct NotifyPlugin {}

fn get_filename_from_path_string(path_string: String) -> String {
    let path = Path::new(&path_string);
    let file_name = path.file_name();
    if file_name.is_none() {
        return String::from("filename.convert.failed");
    } else {
        let file_name_raw = file_name.expect("Failed to unwrap");
        return file_name_raw
            .to_os_string()
            .into_string()
            .expect("Failed to convert to string");
    }
}

fn get_config_format() -> String {
    let config = config::get_config();
    let default_format = "Playing: FILE";
    let notify_config: &toml::Value;
    if config.get("notify").is_none() {
        error!("Failed to read notify config, using default config");
        return default_format.to_string();
    } else {
        notify_config = config.get("notify").unwrap();
    }
    if notify_config.get("format").is_none() {
        error!("Failed to read format from config, using default");
        return default_format.to_string();
    } else {
        return notify_config
            .get("format")
            .unwrap()
            .to_string()
            .replace("\"", "");
    }
}

impl PluginApi for NotifyPlugin {
    fn start(&mut self) {
        info!("Notify plugin loaded");
    }
    fn hooks(&mut self) -> Vec<String> {
        let mut hooks = Vec::new();
        hooks.push("file_changed".to_string());
        return hooks;
    }
    fn file_changed_hook(&mut self, file: String) {
        info!("Sending notification");
        if file != "empty" {
            let nice_file_name = get_filename_from_path_string(file.to_string());
            let title = crate::metadata::get_title(file.to_string());
            let mut format = get_config_format();
            // replace the data in the format
            format = format.replace("FULL_FILE", &file.to_string());
            format = format.replace("FILE_NAME", &nice_file_name);
            format = format.replace("TITLE", &title);
            format = format.replace("\\n", "\n");
            Notification::new()
                .summary("Rapd Notify")
                .body(&format)
                .show()
                .expect("Failed to send notification");
        } else {
            Notification::new()
                .summary("Rapd Notify")
                .body("The player has stopped")
                .show()
                .expect("Failed to send notification");
        }
    }
}
