use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixStream,UnixListener};
use std::thread;

fn handle_client(stream: UnixStream) {
    let stream = BufReader::new(stream);
    for line in stream.lines() {
        debug!("Got data on socket");
        debug!("Parsing request json");
    }
}

pub fn start_server(){
    let socket_path = "/tmp/rapd.socket";
    info!("Rapd socket path is {}", socket_path);
    let listener = UnixListener::bind(socket_path).unwrap();
    info!("Started rapd server, awaiting connections...");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}
