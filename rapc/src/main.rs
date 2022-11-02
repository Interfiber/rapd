use clap::{arg, Arg, Command};

mod config;
mod database;
mod player;
mod server;
mod utils;

fn cli() -> Command {
    Command::new("rapc")
        .about("Rust audio player client")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("database")
                .about("Database operations")
                .subcommand(Command::new("rebuild").about("Rebuild database"))
                .subcommand(Command::new("files").about("Get database files")),
        )
        .subcommand(
            Command::new("config")
                .about("Config operations")
                .subcommand(
                    Command::new("set")
                        .about("Set config value")
                        .arg(Arg::new("key"))
                        .arg(Arg::new("value")),
                ),
        )
        .subcommand(
            Command::new("player")
                .about("Player operations")
                .subcommand(Command::new("file").about("Get playing file"))
                .subcommand(Command::new("state").about("Get the player state"))
                .subcommand(Command::new("stop").about("Stop the player"))
                .subcommand(Command::new("pause").about("Toggle the player pause"))
                .subcommand(Command::new("length").about("Get the length of the audio"))
                .subcommand(Command::new("time").about("Get the position in the audio"))
                .subcommand(Command::new("metadata").about("Get the metadata for the audio").arg(Arg::new("file")))
                .subcommand(
                    Command::new("play")
                        .arg(Arg::new("file"))
                        .arg(arg!(-l --loop "Enables looping of the audio"))
                        .about("Play a file"),
                ),
        )
        .subcommand(Command::new("ping").about("Ping the rapd server"))
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("database", sub_matches)) => {
            let operation = sub_matches.subcommand().unwrap_or(("help", sub_matches));

            match operation {
                ("rebuild", _) => {
                    database::rebuild();
                }
                ("files", _) => {
                    database::files();
                }
                _ => println!("Invalid command!"),
            }
        }
        Some(("config", sub_matches)) => {
            let operation = sub_matches.subcommand().unwrap_or(("help", sub_matches));

            match operation {
                ("set", args) => {
                    let key = args
                        .get_one::<String>("key")
                        .map(|s| s.as_str())
                        .expect("No key!");
                    let val = args
                        .get_one::<String>("value")
                        .map(|s| s.as_str())
                        .expect("No value!");

                    config::set_config_value(String::from(key), String::from(val));
                }
                _ => println!("Invalid command!"),
            }
        },
        Some(("ping", _)) => {
            utils::ping();
        }
        Some(("player", sub_matches)) => {
            let operation = sub_matches.subcommand().unwrap_or(("help", sub_matches));

            match operation {
                ("file", _) => {
                    player::file();
                }
                ("play", args) => {
                    let should_loop = args.get_one::<bool>("loop").unwrap_or(&false);
                    let file = args
                        .get_one::<String>("file")
                        .map(|s| s.as_str())
                        .expect("No file");

                    player::play(file, *should_loop);
                },
                ("metadata", args) => {
                    let file = args.get_one::<String>("file").map(|s| s.as_str()).unwrap_or("player");

                    if file == "player" {
                        player::metadata();
                    } else {
                        player::file_metadata(String::from(file));
                    }
                },
                ("time", _) => {
                    player::time();
                },
                ("length", _) => {
                    player::length();
                },
                ("pause", _) => {
                    player::pause();
                },
                ("stop", _) => {
                    player::stop();
                }
                _ => println!("Invalid command!"),
            }
        }
        _ => unreachable!(),
    }
}
