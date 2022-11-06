use crate::{
    server::{RapdCommand, RapdServer},
    utils::client_error,
};

pub fn create(name: String, desc: String) {
    let mut server = RapdServer::new();
    server.connect();

    let cmd = RapdCommand::new(String::from("db_create_playlist"), vec![name, desc]);

    server.write_cmd(cmd);

    let line = server.read_line();

    if line.is_empty() {
        println!("{}", client_error(String::from("Line is empty")));
    } else {
        print!("{}", line);
    }
}

pub fn add_file(name: String, file: String) {
    let mut server = RapdServer::new();
    server.connect();

    let cmd = RapdCommand::new(String::from("db_add_file_to_playlist"), vec![name, file]);

    server.write_cmd(cmd);

    let line = server.read_line();

    if line.is_empty() {
        println!("{}", client_error(String::from("Line is empty")));
    } else {
        print!("{}", line);
    }
}

pub fn remove(name: String) {
    let mut server = RapdServer::new();
    server.connect();

    let cmd = RapdCommand::new(String::from("db_remove_playlist"), vec![name]);

    server.write_cmd(cmd);

    let line = server.read_line();

    if line.is_empty() {
        println!("{}", client_error(String::from("Line is empty")));
    } else {
        print!("{}", line);
    }
}

pub fn remove_file(name: String, file: String) {
    let mut server = RapdServer::new();
    server.connect();

    let cmd = RapdCommand::new(
        String::from("db_remove_file_from_playlist"),
        vec![name, file],
    );

    server.write_cmd(cmd);

    let line = server.read_line();

    if line.is_empty() {
        println!("{}", client_error(String::from("Line is empty")));
    } else {
        print!("{}", line);
    }
}

pub fn list() {
    let mut server = RapdServer::new();
    server.connect();

    let cmd = RapdCommand::new(String::from("db_get_playlists"), vec![]);

    server.write_cmd(cmd);

    let line = server.read_line();

    if line.is_empty() {
        println!("{}", client_error(String::from("Line is empty")));
    } else {
        print!("{}", line);
    }
}

pub fn files(name: String) {
    let mut server = RapdServer::new();
    server.connect();

    let cmd = RapdCommand::new(String::from("db_get_files_in_playlist"), vec![name]);

    server.write_cmd(cmd);

    let line = server.read_line();

    if line.is_empty() {
        println!("{}", client_error(String::from("Line is empty")));
    } else {
        print!("{}", line);
    }
}
