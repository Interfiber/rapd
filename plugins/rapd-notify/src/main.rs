use notify_rust::{Notification, Hint};
use serde_json::Value;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpStream;

fn get_playing_file() -> String {
    let mut stream = TcpStream::connect("127.0.0.1:8932").expect("Failed to connect");
    let _len = stream.write(b"{\"request_type\":\"current_file\", \"full_path\": false}\n");
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    let _len = reader.read_line(&mut line);
    println!("Got from server: {}", &line);
    let parsed: Value = serde_json::from_str(&line).expect("Failed to parse json!");
    return parsed["message"].to_string().replace("\"", "");
}



fn main() {
Notification::new()
    .summary("Playing music")
    .body(&format!("Playing file: {}", get_playing_file()))
    .appname("rapd-notify")
    .hint(Hint::Category("music".to_owned()))
    .show().expect("Failed to send"); 
}
