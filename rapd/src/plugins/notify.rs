// Name: notify plugin
// Description: Notify you via the system notification daemon when something changes
// Author: Interfiber <webmaster@interfiber.dev>
use crate::plugin_api::PluginApi;
use notify_rust::Notification;

// construct our plugin
pub struct NotifyPlugin {
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
    fn file_changed_hook(&mut self, file: String){
        info!("Sending notification");
        if file != "empty" {
            Notification::new()
                .summary("Rapd Notify")
                .body(&format!("Playing new file: {}", file))
                .show().expect("Failed to send notification");
        } else {
            Notification::new()
                .summary("Rapd Notify")
                .body("The player has stopped")
                .show().expect("Failed to send notification");
        }
    }
}

