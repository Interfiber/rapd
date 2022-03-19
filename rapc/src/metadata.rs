use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpStream;

pub fn set_title(path: String, title: String){
    println!("Setting title of music...");
    let mut stream = TcpStream::connect("127.0.0.1:8932").expect("Failed to connect");
    let _len = stream.write(format!("{{\"request_type\":\"metadata_set_title\", \"path\": \"{}\", \"new_value\": \"{}\"}}\n", path, title).as_bytes());
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    let _len = reader.read_line(&mut line);
    println!("Got from server: {}", &line);
}

pub fn get_title(path: String) {
    let mut stream = TcpStream::connect("127.0.0.1:8932").expect("Failed to connect");
    let _len = stream.write(format!("{{\"request_type\":\"metadata_get_title\", \"path\": \"{}\"}}\n", path).as_bytes());
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    let _len = reader.read_line(&mut line);
    let msg: serde_json::Value = serde_json::from_str(&line).expect("Failed to parse json from server!");
    println!("{}", msg["message"].to_string().replace("\"", ""));
}
