// notify plugin
#[cfg(feature = "notifyplugin")]
pub mod notify;

// discord plugin
#[cfg(feature = "discordplugin")]
pub mod discord;

// plugin loader, we build this by default
pub mod loader;
