use crate::utils::{get_server_stream, read_from_server};
use serde_json::Value;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpStream;

// Stop the player
pub fn stop_player() {
    println!("Sending stop request to player");
    let mut stream = get_server_stream();
    let json_request = json!({
        "request_type": "stop_player"
    })
    .to_string();
    stream
        .write(format!("{}\n", json_request).as_bytes())
        .expect("Failed to write to stream");
    let result = read_from_server(stream);
    let result_json: Value = serde_json::from_str(&result).expect("Failed to parse json");
    if result_json["error"]
        .as_bool()
        .expect("Failed to convert to bool")
    {
        println!("Failed to stop player");
        println!("Log: {}", result_json["message"].to_string());
    } else {
        println!("Server: {}", result_json["message"].to_string());
    }
}

// play a file
pub fn play_file(path: String, loop_audio: bool) {
    println!("Sending play request to player");
    println!("Audio file path: {}", path);
    let mut stream = get_server_stream();
    let json_request = json!({
        "request_type": "play_audio_file",
        "audio_file_path": path,
        "loop_audio": loop_audio
    })
    .to_string();
    stream
        .write(format!("{}\n", json_request).as_bytes())
        .expect("Failed to write to stream");
    let result = read_from_server(stream);
    let result_json: Value = serde_json::from_str(&result).expect("Failed to parse json");
    if result_json["error"]
        .as_bool()
        .expect("Failed to convert to bool")
    {
        println!("Failed to start player");
        println!("Log: {}", result_json["message"].to_string());
    } else {
        println!("Server: {}", result_json["message"].to_string());
    }
}

// get the current playing file
pub fn get_playing_file(full_path: bool) -> String {
    let mut stream = get_server_stream();
    let json_request = json!({
        "request_type": "current_file",
        "full_path": full_path
    })
    .to_string();
    stream
        .write(format!("{}\n", json_request).as_bytes())
        .expect("Failed to write to stream");
    let result = read_from_server(stream);
    let result_json: Value = serde_json::from_str(&result).expect("Failed to parse json");
    if result_json["error"]
        .as_bool()
        .expect("Failed to convert to bool")
    {
        println!("Failed to get current file");
        return String::from("error");
    } else {
        return result_json["message"].to_string().replace("\"", "");
    }
}

pub fn shutdown_server() {
    println!("Sending stop request to player");
    let mut stream = get_server_stream();
    let json_request = json!({
        "request_type": "server_shutdown"
    })
    .to_string();
    stream
        .write(format!("{}\n", json_request).as_bytes())
        .expect("Failed to write to stream");
    let result = read_from_server(stream);
    let result_json: Value = serde_json::from_str(&result).expect("Failed to parse json");
    if result_json["error"]
        .as_bool()
        .expect("Failed to convert to bool")
    {
        println!("Failed to stop server");
        println!("Log: {}", result_json["message"].to_string());
    } else {
        println!("Server: {}", result_json["message"].to_string());
    }
}

pub fn pause_player() {
    let mut stream = TcpStream::connect("127.0.0.1:8932").expect("Failed to connect");
    let _len = stream.write(b"{\"request_type\":\"pause_player\"}\n");
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    let _len = reader.read_line(&mut line);
    println!("Got from server: {}", &line);
}

pub fn unpause_player() {
    let mut stream = TcpStream::connect("127.0.0.1:8932").expect("Failed to connect");
    stream
        .write(b"{\"request_type\":\"unpause_player\"}\n")
        .expect("Failed to write to stream");
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("Failed to read line from stream");
    println!("{}", &line);
}
