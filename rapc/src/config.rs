use crate::{
    server::{RapdCommand, RapdServer},
    utils::client_error,
};

pub fn set_config_value(key: String, value: String) {
    let mut server = RapdServer::new();
    server.connect();

    let cmd = RapdCommand::new(String::from("config_set"), vec![key, value]);

    server.write_cmd(cmd);

    let line = server.read_line();

    if line.is_empty() {
        print!("{}", client_error(String::from("Line is empty")));
    } else {
        print!("{}", line);
    }
}
