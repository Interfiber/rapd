use std::thread::Builder;
// a new function must be added for every plugin in the plugins folder
#[cfg(feature = "notifyplugin" )]
use crate::plugins::notify;

#[cfg(feature = "plugins" )]
use crate::plugin_api::PluginApi;

pub fn notify_load_plugin(){
    info!("Attemping to load notify...");
    let mut notify_plugin = notify::NotifyPlugin {
    };
    notify_plugin.start();
    // spawn a new thread with our plugin loop
    Builder::new().name("notify_plugin".to_string()).spawn(move || {
        notify_plugin.spawn();
    }).expect("Failed to spawn plugin thread for notify plugin!");
}
