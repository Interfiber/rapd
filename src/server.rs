use std::io::{BufRead, BufReader};
use crate::requests;
use std::io::Write;
use std::thread;
use crate::requests::AudioPlayRequest;
use crate::json::parse_json_audio_play;
use crate::player::play_audio_from_request;
use std::net::{TcpListener, TcpStream};
use crate::json::parse_json_raw;

fn handle_client(stream: TcpStream) {
    let mut stream = BufReader::new(stream);
    loop {
        let mut buf = String::new();
        if stream.read_line(&mut buf).is_err() {
            break;
        }
        if buf != "" {
            info!("Parsing client data");
            let json = parse_json_raw(buf);
            // check if we have a request type
            if json["request_type"] == serde_json::Value::Null {
                warn!("Rejecting request: no request_type given in request");
                stream.get_ref().write(requests::get_request_rejected_string("Request type not given").as_bytes()).expect("Failed to write to socket");
                break;
            } else {
                // match/switch the request type
                match json["request_type"].to_string().replace("\"", "").as_str() {
                    "play_audio_file" => {
                        let audio_req: AudioPlayRequest = parse_json_audio_play(json.to_string());
                        play_audio_from_request(audio_req);
                        stream.get_ref().write(requests::get_request_ok_string("Playing audio file").as_bytes()).expect("Failed to write to socket");
                        break;
                    }
                    _ => {
                        warn!("Rejecting request: request type invalid");
                        stream.get_ref().write(requests::get_request_rejected_string("Invalid request type").as_bytes()).expect("Failed to write to socket");
                        break;
                    }
                }
            }
        }
    }
}

pub fn start_server(){
    let addr = "127.0.0.1:8932";
    let listener = TcpListener::bind(addr).expect("Failed to bind");
    info!("Started server at: {}", addr);
    for stream in listener.incoming() {
        info!("Client connected to rapd server");
        let stream = stream.unwrap();
        // spawn a new thread with the name rapd_client_handler
        // this thread will handle the current connection for the client
        thread::Builder::new().name("rapd_client_handler".to_string()).spawn(move || handle_client(stream)).expect("Failed to spawn thread to client");
    }
} 
