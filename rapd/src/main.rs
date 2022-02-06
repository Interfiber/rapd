mod player;
mod requests;
mod server;
mod json;
#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_json;

use env_logger::*;
fn main() {
    println!("Loading env_logger");
    // configure the logger
    let env = Env::default()
        .filter_or("RAPD_LOG_LEVEL", "debug")
        .write_style_or("RAPD_LOG_STYLE", "always");
    // build the logger
    init_from_env(env);
    info!("Started env_logger");
    info!("Starting server");
    server::start_server();
}
