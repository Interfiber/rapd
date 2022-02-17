use std::thread::Builder;
// a new function must be added for every plugin in the plugins folder
#[cfg(feature = "notifyplugin")]
use crate::plugins::notify;

#[cfg(feature = "discordplugin")]
use crate::plugins::discord;

#[cfg(feature = "plugins")]
use crate::plugin_api::PluginApi;

#[cfg(feature = "notifyplugin")]
pub fn notify_load_plugin() {
    info!("Attemping to load notify...");
    let mut notify_plugin = notify::NotifyPlugin {};
    notify_plugin.start();
    // spawn a new thread with our plugin loop
    Builder::new()
        .name("notify_plugin".to_string())
        .spawn(move || {
            notify_plugin.spawn();
        })
        .expect("Failed to spawn plugin thread for notify plugin!");
}
#[cfg(feature = "discordplugin")]
pub fn discord_load_plugin() {
    info!("Attemping to load discord...");
    let mut discord_plugin = discord::DiscordPlugin {};
    discord_plugin.start();
    // spawn a new thread with our plugin loop
    Builder::new()
        .name("discord_plugin".to_string())
        .spawn(move || {
            discord_plugin.spawn();
        })
        .expect("Failed to spawn plugin thread for discord plugin!");
}
