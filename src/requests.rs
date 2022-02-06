use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AudioPlayRequest { 
    request_type: String,
    audio_file_path: String,
    push_to_top: bool,
    title: String,
    author: String,
    playlist_id: i32
}


// functions
pub fn get_request_rejected_string(why: &str) -> String {
    return json!({
        "request_type": "Rejection",
        "error": true,
        "message": why
    }).to_string()
}
pub fn get_request_ok_string(why: &str) -> String {
    return json!({
        "request_type": "Succeeded",
        "error": false,
        "message": why
    }).to_string();
}
