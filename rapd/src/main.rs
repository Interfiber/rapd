// modules
mod player;
mod state;
// only compile plugins if we have it enabled
#[cfg(feature = "plugins")]
mod plugins;
#[cfg(feature = "plugins")]
mod plugin_api;
mod config;
mod requests;
mod db;
mod enums;
mod utils;
mod server;
mod json;
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
    }).expect("Error setting Ctrl-C handler");
    info!("Starting server");
    server::start_server();
}
