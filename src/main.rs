use crate::state::{PLAYER, PLAYER_SENDER};

#[macro_use]
extern crate log;

mod audio;
mod client;
mod commands;
mod config;
mod database;
mod json;
mod metadata;
mod notifications;
mod player;
mod server;
mod state;

fn main() {
    // init logger
    pretty_env_logger::init();

    info!("Starting player...");

    let (sender, receiver) = flume::unbounded();
    let r1 = receiver;
    PLAYER_SENDER.lock().set_sender(sender);

    trace!("Created two channels for main thread to player thread communication");

    std::thread::Builder::new()
        .name(String::from("player"))
        .spawn(move || {
            PLAYER.lock().start(r1);
        })
        .expect("Failed to spawn player thread");

    info!("Loading database for first time...");
    database::load_db();

    info!("Running autostart...");
    config::autostart();

    info!("Starting TCP server...");
    let server = server::RapdServer::new(6702);
    server.start();
}
