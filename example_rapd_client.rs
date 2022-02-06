use std::io::BufReader;
use std::io::Write;
use std::io::BufRead;
use std::net::TcpStream;

fn main(){
    let mut stream = TcpStream::connect("127.0.0.1:8932").expect("Failed to connect");
    let len = stream.write(b"{\"request_type\":\"play_audio_file\",\"audio_file_path\":\"sample.mp3\",\"push_to_top\":false,\"title\":\"bob\",\"author\":\"aw\",\"playlist_id\":0}\n");
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    let len = reader.read_line(&mut line);
    println!("Got from server: {}", &line);
}
