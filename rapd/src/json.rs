use serde::{Deserialize, Serialize};
use std::time::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct RapdMessage {
    pub params: Vec<String>,
    pub command: String,
    pub client_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RapdCommandResponse {
    pub message: serde_json::Value,
    pub failed: bool,
    pub timestamp: u128,
}

impl RapdCommandResponse {
    pub fn new(message: serde_json::Value, failed: bool) -> RapdCommandResponse {
        // get current timestamp
        let start = SystemTime::now();
        let timestamp = start
            .duration_since(UNIX_EPOCH)
            .expect("Failed to find time since unix epoch")
            .as_millis();

        RapdCommandResponse {
            message,
            failed,
            timestamp,
        }
    }
}

pub fn parse_msg(msg: String) -> RapdMessage {
    let out: RapdMessage = serde_json::from_str(&msg).expect("Json parsing error!");
    out
}
