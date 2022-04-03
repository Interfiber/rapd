use crate::utils::{get_server_stream, read_from_server};
use serde_json::Value;
use std::io::Write;
use text_io::read;

pub fn get_music_db() -> Vec<Value> {
    let mut stream = get_server_stream();
    let json_request = json!({
        "request_type": "get_music"
    })
    .to_string();
    stream
        .write(format!("{}\n", json_request).as_bytes())
        .expect("Failed to write to stream");
    let result = read_from_server(stream);
    let result_json: Value = serde_json::from_str(&result).expect("Failed to parse json");
    if result_json["error"]
        .as_bool()
        .expect("Failed to convert to bool")
    {
        println!("Failed to get player music");
        return json!([]).as_array().unwrap().to_vec();
    } else {
        return result_json["message"]
            .as_array()
            .expect("Failed to convert message to array")
            .to_vec()
            .to_owned();
    }
}

// print the contents of the music db
pub fn print_music_db() {
    let music = get_music_db();
    let mut iterator = 1;
    for file in music.iter() {
        let file_path = file.to_string().replace("\"", "");
        println!("File {}: {}", iterator, file_path);
        iterator += 1;
    }
}

// rebuild the db
pub fn rebuild() {
    println!("Rebuilding music database...");
    let mut stream = get_server_stream();
    let json_request = json!({
        "request_type": "rebuild_music_db"
    })
    .to_string();
    stream
        .write(format!("{}\n", json_request).as_bytes())
        .expect("Failed to write to stream");
    let result = read_from_server(stream);
    let result_json: Value = serde_json::from_str(&result).expect("Failed to parse json");
    if result_json["error"]
        .as_bool()
        .expect("Failed to convert to bool")
    {
        println!("Failed to rebuild database");
        println!("Log: {}", result_json["message"].to_string());
    } else {
        println!("Server: {}", result_json["message"].to_string());
    }
}

// select music to play from the database
pub fn tui_select(loop_audio: bool) {
    let music = get_music_db();
    let mut i = 0;
    for file in music.iter() {
        i += 1;
        println!("{}    {}", i, file.to_string().replace("\"", ""));
    }
    println!("Select file number(1-{}):", i);
    let selected: usize = read!();
    if selected > music.len() {
        println!("Please select a valid number!");
        std::process::exit(1);
    } else {
        crate::player::play_file(
            music[(selected - 1)].to_string().replace("\"", ""),
            loop_audio,
        );
    }
}
