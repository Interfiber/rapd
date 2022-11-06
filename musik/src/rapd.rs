use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct RapdCommand {
    command: String,
    params: Vec<String>,
    client_name: String,
}

pub struct RapdServer {
    stream: Option<TcpStream>,
}

#[derive(Serialize, Deserialize)]
pub struct RapdMetadata {
    pub file: String,
    pub length: RapdPlayerTime,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_art: String,
}

#[derive(Serialize, Deserialize)]
pub struct RapdPlayerTime {
    pub hour: u64,
    pub min: u64,
    pub second: u64,
}

#[derive(Serialize, Deserialize)]
pub struct RapdPlaylist {
    pub files: Vec<String>, // list files in the playlist
    pub create_date: i32,
    pub playlist_name: String,
    pub playlist_desc: String,
}

impl RapdCommand {
    pub fn new(command: String, params: Vec<String>) -> RapdCommand {
        RapdCommand {
            command,
            params,
            client_name: String::from("musik"),
        }
    }

    pub fn deserialize(&self) -> String {
        serde_json::to_string(self).expect("Failed to convert RapdCommand into json string")
    }
}

impl RapdServer {
    /// Creates a new RapdServer
    pub fn new() -> RapdServer {
        RapdServer {
            stream: Default::default(),
        }
    }

    /// Makes a new connection to the RAPD server
    pub fn connect(&mut self) {
        self.stream = match TcpStream::connect("127.0.0.1:6702") {
            Ok(s) => Some(s),
            Err(_err) => {
                println!("Connection failed!");
                std::process::exit(1);
            }
        };
    }

    /// Write a command to the server
    pub fn write_cmd(&mut self, cmd: RapdCommand) {
        self.stream
            .as_ref()
            .expect("Not connected to RAPD server")
            .write_all(format!("{}\n", cmd.deserialize()).as_bytes())
            .expect("Failed to write to RAPD server");
    }

    /// Reads the next line from the server, blocks the thread until we get a new line
    pub fn read_line(&mut self) -> String {
        let mut buf_reader =
            BufReader::new(self.stream.as_ref().expect("Not connected to RAPD server"));

        let mut line = String::new();

        buf_reader
            .read_line(&mut line)
            .expect("Failed to read next line from server");

        line
    }

    /// Close the connection
    pub fn close(&self) {
        self.stream
            .as_ref()
            .expect("Not connected to RAPD server")
            .shutdown(std::net::Shutdown::Both)
            .expect("Failed to shutdown IO");
    }
}

pub fn metadata_for_file(server: &mut RapdServer, file: String) -> RapdMetadata {
    let cmd = RapdCommand::new(String::from("player_get_metadata"), vec![file]);

    server.write_cmd(cmd);

    let result = server.read_line();

    if result.is_empty() {
        println!("Line empty!");
        std::process::exit(1);
    } else {
        let t: Value = serde_json::from_str(&result).expect("Failed to parse message");
        let r: RapdMetadata =
            serde_json::from_value(t["message"].to_owned()).expect("Failed to parse message");

        r
    }
}

pub fn get_length() -> RapdPlayerTime {
    let mut server = RapdServer::new();
    server.connect();

    let cmd = RapdCommand::new(String::from("player_length"), vec![]);

    server.write_cmd(cmd);

    let result = server.read_line();

    if result.is_empty() {
        println!("Line empty!");
        std::process::exit(1);
    } else {
        let t: Value = serde_json::from_str(&result).expect("Failed to parse message");
        let len: RapdPlayerTime =
            serde_json::from_value(t["message"].to_owned()).expect("Failed to parse message");

        server.close();
        len
    }
}

pub fn get_time() -> RapdPlayerTime {
    let mut server = RapdServer::new();
    server.connect();

    let cmd = RapdCommand::new(String::from("player_time"), vec![]);

    server.write_cmd(cmd);

    let result = server.read_line();

    if result.is_empty() {
        println!("Line empty!");
        std::process::exit(1);
    } else {
        let t: Value = serde_json::from_str(&result).expect("Failed to parse message");
        let pt: RapdPlayerTime =
            serde_json::from_value(t["message"].to_owned()).expect("Failed to parse message");

        pt
    }
}

pub fn database_files() -> Vec<RapdMetadata> {
    let mut server = RapdServer::new();
    server.connect();

    let cmd = RapdCommand::new(String::from("db_get_files"), vec![]);

    server.write_cmd(cmd);

    let result = server.read_line();

    if result.is_empty() {
        println!("Server returned empty line");
        std::process::exit(1);
    } else {
        let full_json: Value =
            serde_json::from_str(&result).expect("Failed to parse info from server");
        let result_json: Vec<Value> =
            serde_json::from_value(full_json["message"].clone()).expect("Failed to parse message");

        let mut final_result = vec![];

        for item in result_json {
            final_result.push(metadata_for_file(
                &mut server,
                item["file"].to_string().replace('\"', ""),
            ));
        }

        server.close();

        final_result
    }
}

pub fn play_file(file: String, loop_audio: bool) {
    let mut server = RapdServer::new();

    server.connect();

    let cmd = RapdCommand::new(
        String::from("play_file"),
        vec![file, loop_audio.to_string()],
    );

    server.write_cmd(cmd);

    server.close();
    // idc about the result
}

pub fn stop() {
    let mut server = RapdServer::new();
    server.connect();

    let cmd = RapdCommand::new(String::from("player_stop"), vec![]);

    server.write_cmd(cmd);
    server.close();
    // idc about the result
}

pub fn pause() {
    let mut server = RapdServer::new();
    server.connect();

    let cmd = RapdCommand::new(String::from("player_toggle_pause"), vec![]);

    server.write_cmd(cmd);
    server.close();
    // idc about the result
}

pub fn playlist_info(name: String, server: &mut RapdServer) -> RapdPlaylist {
    let cmd = RapdCommand::new(String::from("db_get_files_in_playlist"), vec![name]);

    server.write_cmd(cmd);

    let line = server.read_line();

    if line.is_empty() {
        println!("Line empty!");
        std::process::exit(1);
    } else {
        let r: Value = serde_json::from_str(&line).expect("Failed to parse result!");

        if r["failed"].as_bool().unwrap() {
            println!("Failed to get playlist: {}", r);
            std::process::exit(1);
        } else {
            let p: RapdPlaylist =
                serde_json::from_value(r["message"].to_owned()).expect("Failed to parse result!");
            p
        }
    }
}

pub fn playlists() -> Vec<RapdPlaylist> {
    let mut server = RapdServer::new();
    server.connect();

    let cmd = RapdCommand::new(String::from("db_get_playlists"), vec![]);

    server.write_cmd(cmd);

    let line = server.read_line();

    if line.is_empty() {
        println!("Line empty!");
        std::process::exit(1);
    } else {
        let r: Value = serde_json::from_str(&line).expect("Failed to parse result!");
        let p: Vec<Value> =
            serde_json::from_value(r["message"].to_owned()).expect("Failed to parse result!");
        let mut result: Vec<RapdPlaylist> = vec![];

        for name in p.iter() {
            // get playlist info
            result.push(playlist_info(
                name.to_string().replace('"', ""),
                &mut server,
            ));
        }

        server.close();

        result
    }
}

pub fn add_file_to_playlist(name: String, file: String) {
    let mut server = RapdServer::new();
    server.connect();

    let cmd = RapdCommand::new(String::from("db_add_file_to_playlist"), vec![name, file]);

    server.write_cmd(cmd);

    // idc about the result
    server.close();
}

pub fn remove_playlist(name: String) {
    let mut server = RapdServer::new();
    server.connect();

    let cmd = RapdCommand::new(String::from("db_remove_playlist"), vec![name]);

    server.write_cmd(cmd);

    // wait for the delete to be confirmed
    server.read_line();

    // idc about the result
    server.close();
}
