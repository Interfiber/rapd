use crate::server::{RapdCommand, RapdServer};
use serde_json::{json, Value};

pub fn client_error(msg: String) -> Value {
    json!({
        "message": msg,
        "failed": true,
        "client_detected": true
    })
}

pub fn connect_error(msg: String) -> Value {
    json!({
        "message": msg,
        "failed": true,
        "client_error": true,
    })
}

pub fn ping() {
    let mut server = RapdServer::new();

    server.connect();

    let cmd = RapdCommand::new(String::from("ping"), vec![]);

    server.write_cmd(cmd);

    let line = server.read_line();

    if line.is_empty() {
        println!("{}", client_error(String::from("Line is empty")));
    } else {
        print!("{}", line);
    }
}
