mod player;
mod server;
#[macro_use]
extern crate log;

use env_logger::*;
fn main() {
    println!("Loading env_logger");
    // configure the logger
    let env = Env::default()
        .filter_or("RAPD_LOG_LEVEL", "verbose")
        .write_style_or("RAPD_LOG_STYLE", "always");
    // build the logger
    init_from_env(env);
    info!("Started env_logger");
    info!("Starting server");
    server::start_server();
}
