use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpStream;

pub fn add_player_start(cmd: String) {
    let mut stream = TcpStream::connect("127.0.0.1:8932").expect("Failed to connect");
    let _len = stream.write(format!("{{\"request_type\":\"hook_add\", \"hook_type\": \"player_start\", \"command\": \"{}\"}}\n", cmd).as_bytes());
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    let _len = reader.read_line(&mut line);
    println!("Got from server: {}", &line);
}

pub fn add_server_shutdown(cmd: String) {
    let mut stream = TcpStream::connect("127.0.0.1:8932").expect("Failed to connect");
    let _len = stream.write(format!("{{\"request_type\":\"hook_add\", \"hook_type\": \"server_shutdown\", \"command\": \"{}\"}}\n", cmd).as_bytes());
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    let _len = reader.read_line(&mut line);
    println!("Got from server: {}", &line);
}

pub fn add_player_pause(cmd: String) {
    let mut stream = TcpStream::connect("127.0.0.1:8932").expect("Failed to connect");
    let _len = stream.write(format!("{{\"request_type\":\"hook_add\", \"hook_type\": \"player_pause\", \"command\": \"{}\"}}\n", cmd).as_bytes());
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    let _len = reader.read_line(&mut line);
    println!("Got from server: {}", &line);
}

pub fn add_player_unpause(cmd: String) {
    let mut stream = TcpStream::connect("127.0.0.1:8932").expect("Failed to connect");
    let _len = stream.write(format!("{{\"request_type\":\"hook_add\", \"hook_type\": \"player_unpause\", \"command\": \"{}\"}}\n", cmd).as_bytes());
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    let _len = reader.read_line(&mut line);
    println!("Got from server: {}", &line);
}
