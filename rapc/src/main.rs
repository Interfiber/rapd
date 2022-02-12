use clap::{arg, App, AppSettings, Arg};
mod player;
mod db;
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
                .arg(
                    Arg::new("loop")
                        .short('l')
                        .long("loop")
                        .help("Toggle looping for the audio")
                        .takes_value(false)
                        .multiple_occurrences(false)
                        .multiple_values(false)
                )
                .setting(AppSettings::ArgRequiredElseHelp)
        )
        .subcommand(
            App::new("player_stop")
                .about("Stop the player")
        )
        .subcommand(
            App::new("db_print")
                .about("Print the files in the music database")
        )
        .subcommand(
            App::new("db_rebuild")
                .about("Rebuild the music database")
        )
        .subcommand(
            App::new("db_select")
                .about("Select a file to play from the music database")
        )
        .get_matches();
    // match commands
    match matches.subcommand() {
        Some(("play", sub_matches)) => {
            let loop_audio = sub_matches.is_present("loop");
            player::play_file(sub_matches.value_of("PATH").unwrap().to_string(), loop_audio);
        },
        Some(("player_stop", _)) => {
            player::stop_player();
        },
        Some(("db_print", _)) => {
            db::print_music_db();
        },
        Some(("db_rebuild", _)) => {
            db::rebuild();
        }
        _ => unreachable!()
    }
}
