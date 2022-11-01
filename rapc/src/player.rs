use crate::{
    server::{RapdCommand, RapdServer},
    utils::client_error,
};

pub fn file() {
    let mut server = RapdServer::new();

    server.connect();

    let cmd = RapdCommand::new(String::from("player_file"), vec![]);

    server.write_cmd(cmd);

    let line = server.read_line();

    if line.is_empty() {
        println!("{}", client_error(String::from("Line is empty")));
    } else {
        print!("{}", line);
    }
}

pub fn play(file: &str, loop_audio: bool) {
    let mut server = RapdServer::new();

    server.connect();

    let cmd = RapdCommand::new(
        String::from("play_file"),
        vec![String::from(file), loop_audio.to_string()],
    );

    server.write_cmd(cmd);

    let line = server.read_line();

    if line.is_empty() {
        println!("{}", client_error(String::from("Line is empty")));
    } else {
        print!("{}", line);
    }
}

pub fn metadata() {
    let mut server = RapdServer::new();
    server.connect();

    let cmd = RapdCommand::new(String::from("player_get_metadata"), vec![]);

    server.write_cmd(cmd);

    let line = server.read_line();

    if line.is_empty() {
        println!("{}", client_error(String::from("Line is empty")));
    } else {
        print!("{}", line);
    }
}

pub fn time(){
    let mut server = RapdServer::new();
    server.connect();

    let cmd = RapdCommand::new(String::from("player_time"), vec![]);

    server.write_cmd(cmd);

    let line = server.read_line();

    if line.is_empty() {
        println!("{}", client_error(String::from("Line is empty")));
    } else {
        print!("{}", line);
    }
}

pub fn length(){
    let mut server = RapdServer::new();
    server.connect();

    let cmd = RapdCommand::new(String::from("player_length"), vec![]);

    server.write_cmd(cmd);

    let line = server.read_line();

    if line.is_empty() {
        println!("{}", client_error(String::from("Line is empty")));
    } else {
        print!("{}", line);
    }
}

pub fn stop(){
    let mut server = RapdServer::new();
    server.connect();

    let cmd = RapdCommand::new(String::from("player_stop"), vec![]);

    server.write_cmd(cmd);

    let line = server.read_line();

    if line.is_empty() {
        println!("{}", client_error(String::from("Line is empty")));
    } else {
        print!("{}", line);
    }
}

pub fn pause(){
    let mut server = RapdServer::new();
    server.connect();

    let cmd = RapdCommand::new(String::from("player_toggle_pause"), vec![]);

    server.write_cmd(cmd);

    let line = server.read_line();

    if line.is_empty() {
        println!("{}", client_error(String::from("Line is empty")));
    } else {
        print!("{}", line);
    }
}
