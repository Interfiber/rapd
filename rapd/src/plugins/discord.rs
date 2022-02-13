// Name: Discord plugin
// Description: Change you're discord status for the file thats playing, pretty useless plugin tbh 
// Author: Interfiber <webmaster@interfiber.dev>

use crate::plugin_api::PluginApi;
use std::path::Path;
use discord_rich_presence::{activity, new_client, DiscordIpc};

// construct our plugin
pub struct DiscordPlugin {
}

// thread that runs in the background to update discord
fn update(){
  let mut client = new_client("942481156971057213").expect("Failed to connect to discord");
  match client.connect(){
      Ok(_) => info!("Connected to discord IPC"),
      Err(err) => {
          error!("Failed to connect to discord");
          error!("Error log: {}", err);
      }
  }
  loop {
    let audio_file;
    match std::fs::read_to_string("/tmp/rapd.discord.audio_file") {
        Ok(data) => audio_file = data,
        Err(err) => {
            error!("Failed to read state");
            error!("Error log: {}", err);
            audio_file = "failed".to_string();
        }
    };
    let state = format!("Audio name: {}", audio_file);
    let payload = activity::Activity::new().state(&state).details("Listening to audio...");
    match client.set_activity(payload){
        Ok(_) => print!(""),
        Err(err) => {
            error!("Failed to update discord");
            error!("Error log: {}", err);
        }
    }
    std::thread::sleep(std::time::Duration::from_secs(2));
  }
}
fn get_filename_from_path_string(path_string: String) -> String {
    let path = Path::new(&path_string);
    let file_name = path.file_name();
    if file_name.is_none(){
        return String::from("filename.convert.failed");
    } else {
        let file_name_raw = file_name.expect("Failed to unwrap");
        return file_name_raw.to_os_string().into_string().expect("Failed to convert to string");
    }
}

impl PluginApi for DiscordPlugin {
    fn start(&mut self) {
        info!("Discord plugin loaded");
        // test if we still have state files, if so remove them

        std::thread::Builder::new().name("discord_plugin_background_process".to_string()).spawn(move || {
            update();
        }).expect("Failed to spawn updater thread");
        debug!("Spawned update thread");
    }
    fn hooks(&mut self) -> Vec<String> {
        let mut hooks = Vec::new();
        hooks.push("file_changed".to_string());
        return hooks;
    }
    fn file_changed_hook(&mut self, file: String){
        info!("Updating discord status");
        let nice_file_path = get_filename_from_path_string(file);
        match std::fs::write("/tmp/rapd.discord.audio_file", nice_file_path) {
            Ok(_) => info!("Updated state file"),
            Err(err) => {
                error!("Failed to update rapd.discord.state");
                error!("Error log: {}", err);
            }
        }     
    }
}
