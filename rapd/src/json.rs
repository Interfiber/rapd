use crate::requests::*;
use serde_json::Value;
pub fn parse_json_raw(data: String) -> Value {
    let v: Value = serde_json::from_str(&data).expect("Failed to parse json");
    return v;
}
pub fn parse_json_audio_play(data: String) -> AudioPlayRequest {
    let v: AudioPlayRequest = serde_json::from_str(&data).expect("Failed to parse json");
    return v;
}

pub fn parse_json_current_file(data: String) -> CurrentFileRequest {
    let v: CurrentFileRequest = serde_json::from_str(&data).expect("Failed to parse json");
    return v;
}

pub fn parse_json_metadata_get(data: String) -> MetadataGetRequest {
    let v: MetadataGetRequest = serde_json::from_str(&data).expect("Failed to parse json");
    return v;
}

pub fn parse_json_metadata_set(data: String) -> MetadataSetRequest {
    let v: MetadataSetRequest = serde_json::from_str(&data).expect("Failed to parse json");
    return v;
}

pub fn parse_json_hook_add(data: String) -> HookAddRequest {
    let v: HookAddRequest = serde_json::from_str(&data).expect("Failed to parse json");
    return v;
}
