use crate::player::get_current_playing_file;
// plugin api for rapd

// plugin base trait
pub trait PluginApi {
    fn start(&mut self) {
        info!("Enabled plugin");
    }
    fn hooks(&mut self) -> Vec<String>;
    fn file_changed_hook(&mut self, file: String);
    fn spawn(&mut self) {
        let mut old_file = get_current_playing_file(true);
        loop {
            let hooks = self.hooks();
            if hooks.contains(&String::from("file_changed")) {
                if old_file != get_current_playing_file(true) {
                    info!("File changed");
                    self.file_changed_hook(get_current_playing_file(true));
                }
                // store the current file
                old_file = get_current_playing_file(true);
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
    }
}
