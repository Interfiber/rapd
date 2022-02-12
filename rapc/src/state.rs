use crate::utils::*;
use std::io::Write;
use serde_json::Value;

fn get_readable_state(state: String) -> String {
    match state.as_str() {
        "playerstate.idle" => {
            return "Player is idle".to_string();
        },
        "playerstate.playing" => {
            return "Playing audio".to_string();
        },
        "playerstate.killed" => {
            // use offline instead of killed because it makes more sense from a readable
            // perspective
            return "Player offline".to_string();
        },
        "playerstate.rebuilding" => {
            return "Rebuilding music database".to_string();
        },
        "playerstate.stop" => {
            return "Player is stopping".to_string();
        },
        _ => {
            return "Unknown State(Client)".to_string();
        }
    }
}

// get the state
pub fn get_state(readable: bool) -> String {
    let mut stream = get_server_stream();
    let json_request = json!({
        "request_type": "get_state"
    }).to_string();
    stream.write(format!("{}\n", json_request).as_bytes()).expect("Failed to write to stream");
    let result = read_from_server(stream);
    let result_json: Value = serde_json::from_str(&result).expect("Failed to parse json");
    if result_json["error"].as_bool().expect("Failed to convert to bool") {
        println!("Failed to get state, using playerstate.idle");
        return String::from("playerstate.idle");
    } else {
        if !readable {
            return result_json["message"].to_string().replace("\"", "");
        } else {
            return get_readable_state(result_json["message"].to_string().replace("\"", ""));
        }
    }
}
