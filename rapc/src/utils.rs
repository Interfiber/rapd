use serde_json::{json, Value};

pub fn client_error(msg: String) -> Value {
    json!({
        "message": msg,
        "failed": true,
        "client_detected": true
    })
}
