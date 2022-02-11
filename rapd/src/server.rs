use std::io::{BufRead, BufReader};
use crate::requests;
use std::io::Write;
use std::thread;
use crate::requests::AudioPlayRequest;
use crate::requests::audio_play_status_request_string;
use crate::json::parse_json_audio_play;
use crate::player::play_audio_from_request;
use crate::player::stop_player;
use std::net::{TcpListener, TcpStream};
use crate::json::parse_json_raw;

fn write_to_stream(stream: BufReader<TcpStream>, message: String){
    // write the message to the tcp stream
    match stream.get_ref().write(message.as_bytes()){
        Ok(_) => {},
        Err(err) => {
            error!("Failed to write data to TcpStream: {}", message);
            error!("Error log: {}", err);
        }
    }
}
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
                write_to_stream(stream, requests::get_request_rejected_string("Request type not given"));
                break;
            } else {
                // match/switch the request type
                match json["request_type"].to_string().replace("\"", "").as_str() {
                    "play_audio_file" => {
                        let audio_req: AudioPlayRequest = parse_json_audio_play(json.to_string());
                        let status = play_audio_from_request(audio_req);
                        let request_string = audio_play_status_request_string(status);
                        write_to_stream(stream, request_string);
                        break;
                    },
                    "stop_player" => {
                        stop_player();
                        write_to_stream(stream, requests::get_request_ok_string("Sent stop request to player"));
                        break;
                    },
                    "rebuild_music_db" => {
                        let music_db_status = crate::db::rebuild_music_database();
                        let status_string = requests::db_rebuild_status_request_string(music_db_status);
                        write_to_stream(stream, status_string);
                        break;
                    },
                    _ => {
                        warn!("Rejecting request: request type invalid");
                        write_to_stream(stream, requests::get_request_rejected_string("Invalid request type"));
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
