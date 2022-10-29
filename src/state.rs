use flume::Sender;
use lazy_static::lazy_static;
use parking_lot::Mutex;

use crate::{database::RapdDatabase, player::RapdPlayer};

pub struct RapdPlayerChannel {
    pub sender: Option<Sender<String>>,
}

impl RapdPlayerChannel {
    pub fn set_sender(&mut self, s: Sender<String>) {
        self.sender = Some(s);
    }
}

lazy_static! {
    pub static ref PLAYER: Mutex<RapdPlayer> = Mutex::new(RapdPlayer::new());
    pub static ref PLAYER_SENDER: Mutex<RapdPlayerChannel> = Mutex::new(RapdPlayerChannel {
        sender: Default::default()
    });
    pub static ref DATABASE: Mutex<RapdDatabase> = Mutex::new(RapdDatabase::empty());
}
