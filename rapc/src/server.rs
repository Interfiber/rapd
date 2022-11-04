use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RapdCommand {
    command: String,
    params: Vec<String>,
    client_name: String,
}

pub struct RapdServer {
    stream: Option<TcpStream>,
}

impl RapdCommand {
    pub fn new(command: String, params: Vec<String>) -> RapdCommand {
        RapdCommand {
            command,
            params,
            client_name: String::from("rapc"),
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
            Err(err) => {
                println!(
                    "{}",
                    crate::utils::connect_error(format!(
                        "Failed to connect to RAPD server, error: {}",
                        err
                    ))
                );
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
}
