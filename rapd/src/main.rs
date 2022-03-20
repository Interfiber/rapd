// modules
mod config;
mod db;
mod enums;
mod hook;
mod json;
mod metadata;
mod player;
#[cfg(feature = "plugins")]
mod plugin_api;
#[cfg(feature = "plugins")]
mod plugins;
mod requests;
mod server;
mod state;
mod utils;
// imports
#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_json;

use env_logger::*;

fn main() {
    println!("Loading env_logger");
    // configure the logger
    let env = Env::default()
        .filter_or("RAPD_LOG_LEVEL", "trace")
        .write_style_or("RAPD_LOG_STYLE", "always");
    // build the logger
    init_from_env(env);
    if !db::db_exists() {
        db::create_db();
    }
    info!("Started env_logger");
    info!("Creating ctrlc hooks...");
    ctrlc::set_handler(move || {
        utils::shutdown();
    })
    .expect("Error setting Ctrl-C handler");
    info!("Starting server");
    server::start_server();
}
