use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StateFile {
    pub state: String
}

pub fn set_state(state: String){

}
