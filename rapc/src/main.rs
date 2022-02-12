use clap::{arg, App, AppSettings};
mod player;
mod utils;

#[macro_use]
extern crate serde_json;

fn main(){
    let matches = App::new("rapc")
        .about("Rust Audio Player Client")
        .author("Interfiber <webmaster@interfiber.dev>")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            App::new("play")
                .about("Play a audio file directly from disk")
                .arg(arg!(<PATH> "Path to the audio file to play"))
                .setting(AppSettings::ArgRequiredElseHelp)
        )
        .subcommand(
            App::new("player_stop")
                .about("Stop the player")
        )
        .get_matches();
    // match commands
    match matches.subcommand() {
        Some(("play", sub_matches)) => {
            player::play_file(sub_matches.value_of("PATH").unwrap().to_string());
        },
        Some(("player_stop", _)) => {
            player::stop_player();
        }
        _ => unreachable!()
    }
}
