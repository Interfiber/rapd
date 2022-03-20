use clap::{arg, App, AppSettings, Arg};
mod db;
mod metadata;
mod hook;
mod player;
mod state;
mod utils;

#[macro_use]
extern crate serde_json;

fn main() {
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
                        .multiple_values(false),
                )
                .setting(AppSettings::ArgRequiredElseHelp),
        )
        .subcommand(App::new("player_stop").about("Stop the player"))
        .subcommand(App::new("db_print").about("Print the files in the music database"))
        .subcommand(App::new("db_rebuild").about("Rebuild the music database"))
        .subcommand(
            App::new("db_select")
                .about("Select a file to play from the music database")
                .arg(
                    Arg::new("loop")
                        .short('l')
                        .long("loop")
                        .help("Toggle looping for the selected audio")
                        .takes_value(false)
                        .multiple_occurrences(false)
                        .multiple_values(false),
                ),
        )
        .subcommand(
            App::new("player_file")
                .about("Print the current playing file")
                .arg(
                    Arg::new("fullpath")
                        .short('f')
                        .long("full-path")
                        .help("Print the full path to the file")
                        .takes_value(false)
                        .multiple_occurrences(false)
                        .multiple_values(false),
                ),
        )
        .subcommand(App::new("shutdown_server").about("Shutdown the rapd server"))
        .subcommand(
            App::new("player_state")
                .about("Print the current player state")
                .arg(
                    Arg::new("readable")
                        .short('r')
                        .long("readable")
                        .help("Make the output text more readable to a human")
                        .takes_value(false)
                        .multiple_occurrences(false)
                        .multiple_values(false),
                ),
        )
        .subcommand(
            App::new("hook_add_player_start")
                .about("Add a hook that fires when the music player starts playing a file")
                .arg(arg!(<COMMAND> "Command to execute with /bin/sh on hook fire"))
        )
        .subcommand(
            App::new("metadata_set_title")
                .about("Set the title of a music file")
                .arg(arg!(<PATH> "Path to the file that will be effected"))
                .arg(arg!(<TITLE> "Title to be written,,")),
        )
        .subcommand(
            App::new("metadata_get_title")
                .about("Get the title of a music file")
                .arg(arg!(<PATH> "Path to the audio file")),
        )
        .get_matches();
    // match commands
    match matches.subcommand() {
        Some(("play", sub_matches)) => {
            let loop_audio = sub_matches.is_present("loop");
            player::play_file(
                sub_matches.value_of("PATH").unwrap().to_string(),
                loop_audio,
            );
        }
        Some(("player_stop", _)) => {
            player::stop_player();
        }
        Some(("db_print", _)) => {
            db::print_music_db();
        }
        Some(("db_rebuild", _)) => {
            db::rebuild();
        }
        Some(("db_select", sub_matches)) => {
            let looped = sub_matches.is_present("loop");
            db::tui_select(looped);
        }
        Some(("player_state", sub_matches)) => {
            let readable = sub_matches.is_present("readable");
            let state = state::get_state(readable);
            if readable {
                println!("Player State: {}", state);
            } else {
                println!("{}", state);
            }
        }
        Some(("player_file", sub_matches)) => {
            let full_path = sub_matches.is_present("fullpath");
            let file = player::get_playing_file(full_path);
            println!("{}", file);
        }
        Some(("shutdown_server", _)) => {
            player::shutdown_server();
        }
        Some(("metadata_set_title", sub_matches)) => {
            let title = sub_matches.value_of("TITLE").unwrap().to_string();
            let path = sub_matches.value_of("PATH").unwrap().to_string();
            metadata::set_title(path, title);
        }
        Some(("metadata_get_title", sub_matches)) => {
            let path = sub_matches.value_of("PATH").unwrap().to_string();
            metadata::get_title(path);
        },
        Some(("hook_add_player_start", sub_matches)) => {
            let cmd = sub_matches.value_of("COMMAND").unwrap().to_string();
            hook::add_player_start(cmd);
        }
        _ => unreachable!(),
    }
}
