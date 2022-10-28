use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:6702").expect("Failed to connect");
    stream
        .write(b"{ \"command\": \"not_ping\", \"params\": [], \"client_name\": \"e\" }\n")
        .expect("Write failed");

    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    reader.read_line(&mut line).expect("Line read failed");
    println!("Got from server: {}", &line);
}
