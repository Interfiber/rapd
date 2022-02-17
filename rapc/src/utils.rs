use std::io::BufRead;
use std::io::BufReader;
use std::net::TcpStream;

// get a new TcpStream connecting to the server
pub fn get_server_stream() -> TcpStream {
    let stream = TcpStream::connect("127.0.0.1:8932").expect("Failed to connect");
    return stream;
}

// read data from the server
pub fn read_from_server(stream: TcpStream) -> String {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("Failed to read data from server");
    return line;
}
