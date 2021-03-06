use crate::hook::add_hook;
use crate::json::{
    parse_json_audio_play, parse_json_current_file, parse_json_hook_add, parse_json_metadata_get,
    parse_json_metadata_set, parse_json_raw,
};
use crate::player::{play_audio_from_request, stop_player};
use crate::requests::{
    self, hook_add_request_string, HookAddRequest, MetadataGetRequest, MetadataSetRequest,
};
use crate::requests::{
    audio_play_status_request_string, metadata_edit_request_string, AudioPlayRequest,
    CurrentFileRequest,
};
use crate::state::{get_state, state_to_string};
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn write_to_stream(stream: BufReader<TcpStream>, message: String) {
    // write the message to the tcp stream
    match stream.get_ref().write(message.as_bytes()) {
        Ok(_) => {}
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
                write_to_stream(
                    stream,
                    requests::get_request_rejected_string("Request type not given"),
                );
                break;
            } else {
                // match/switch the request type
                match json["request_type"].to_string().replace("\"", "").as_str() {
                    // play an audio file
                    "play_audio_file" => {
                        let audio_req: AudioPlayRequest = parse_json_audio_play(json.to_string());
                        let status = play_audio_from_request(audio_req);
                        let request_string = audio_play_status_request_string(status);
                        write_to_stream(stream, request_string);
                        break;
                    }
                    // stop the player
                    "stop_player" => {
                        stop_player();
                        write_to_stream(
                            stream,
                            requests::get_request_ok_string("Sent stop request to player"),
                        );
                        break;
                    }
                    // rebuild the music database
                    "rebuild_music_db" => {
                        let music_db_status = crate::db::rebuild_music_database();
                        let status_string =
                            requests::db_rebuild_status_request_string(music_db_status);
                        write_to_stream(stream, status_string);
                        break;
                    }
                    // return an array of the music in the music db to the client
                    "get_music" => {
                        let music = crate::db::get_music();
                        let result = json!({
                            "request_type": "Success",
                            "error": false,
                            "message": music
                        })
                        .to_string();
                        write_to_stream(stream, result);
                        break;
                    }
                    // shutdown the server
                    "server_shutdown" => {
                        let result = json!({
                            "request_type": "Success",
                            "error": false,
                            "message": "Shutting down server"
                        })
                        .to_string();
                        write_to_stream(stream, result);
                        crate::utils::shutdown();
                        break;
                    }
                    // return the current player state string to the player
                    "get_state" => {
                        let current_state = get_state();
                        let state_string = state_to_string(current_state);
                        write_to_stream(stream, requests::get_request_ok_string(&state_string));
                        break;
                    }
                    // get the current playing file
                    "current_file" => {
                        let file_request: CurrentFileRequest =
                            parse_json_current_file(json.to_string());
                        let current_file =
                            crate::player::get_current_playing_file(file_request.full_path);
                        write_to_stream(
                            stream,
                            requests::current_file_request_string(current_file),
                        );
                        break;
                    }
                    // set metadata for a file
                    "metadata_set" => {
                        let metadata_set_request: MetadataSetRequest =
                            parse_json_metadata_set(json.to_string());
                        let stat = crate::metadata::set_from_request(metadata_set_request);
                        write_to_stream(stream, metadata_edit_request_string(stat));
                        break;
                    }
                    // get the metadata for a file
                    "metadata_get" => {
                        let metadata_get_request: MetadataGetRequest =
                            parse_json_metadata_get(json.to_string());
                        let result = crate::metadata::get_from_request(metadata_get_request);
                        let response = json!({
                            "request_type": "Success",
                            "error": false,
                            "message": result
                        })
                        .to_string();
                        write_to_stream(stream, response);
                        break;
                    }
                    // add a hook
                    "hook_add" => {
                        let hook_add_request: HookAddRequest =
                            parse_json_hook_add(json.to_string());
                        let hook_add_state = add_hook(hook_add_request);
                        let response = hook_add_request_string(hook_add_state);
                        write_to_stream(stream, response);
                        break;
                    }
                    // pause the player
                    "pause_player" => {
                        crate::state::set_state(crate::enums::PlayerState::Paused);
                        let response = json!({
                            "request_type": "Success",
                            "error": false,
                            "message": "Paused player"
                        });
                        write_to_stream(stream, response.to_string());
                        break;
                    }
                    "unpause_player" => {
                        crate::state::set_state(crate::enums::PlayerState::Unpaused);
                        let reponse = json!({
                            "request_type": "Success",
                            "error": false,
                            "message": "Unpaused player"
                        });
                        write_to_stream(stream, reponse.to_string());
                        break;
                    }
                    _ => {
                        warn!("Rejecting request: request type invalid");
                        write_to_stream(
                            stream,
                            requests::get_request_rejected_string("Invalid request type"),
                        );
                        break;
                    }
                }
            }
        }
    }
}

pub fn start_server() {
    let addr = "127.0.0.1:8932";
    let listener = TcpListener::bind(addr).expect("Failed to bind");
    info!("Started server at: {}", addr);
    for stream in listener.incoming() {
        info!("Client connected to rapd server");
        let stream = stream.unwrap();
        // spawn a new thread with the name rapd_client_handler
        // this thread will handle the current connection for the client
        thread::Builder::new()
            .name("rapd_client_handler".to_string())
            .spawn(move || handle_client(stream))
            .expect("Failed to spawn thread to client");
    }
}
