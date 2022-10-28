use crate::commands::manager;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

fn write_to_stream(stream: &mut TcpStream, message: String) {
    // write the message to the tcp stream
    match stream.write(format!("{}\n", message).as_bytes()) {
        Ok(_) => trace!("Wrote message to stream"),
        Err(err) => {
            error!("Failed to write data to TcpStream: {}", message);
            error!("Error log: {}", err);
        }
    }
}

pub fn handle_client(stream: TcpStream) {
    let mut stream = BufReader::new(stream);
    let mut cmd_manager = manager::RapdCommandManager::new();
    manager::init_manager(&mut cmd_manager);

    loop {
        let mut buf = String::new();

        if stream.read_line(&mut buf).is_err() {
            error!("Failed to read from stream!");
            break;
        }

        if !buf.is_empty() {
            let req = crate::json::parse_msg(buf);
            let res = cmd_manager.handle_command(req);
            write_to_stream(
                stream.get_mut(),
                serde_json::to_string(&res)
                    .expect("Failed to create json output for command result"),
            );
        } else {
            break;
        }
    }
}
