use crate::state::{CONFIG, PLAYER};
use notify_rust::Notification;

pub fn alert_play_file() {
    let cfg = CONFIG.lock();

    if cfg.notifications() {
        let player = PLAYER.lock();
        let metadata = player.get_metadata();

        let body = format!(
            "Playing {}\nby {}\nfrom {}",
            metadata.get_title(),
            metadata.get_artist(),
            metadata.get_album()
        );

        Notification::new()
            .summary("Rust Audio Player Daemon")
            .body(&body)
            .image_path(&metadata.get_album_art_file())
            .show()
            .expect("Failed to show notification");
    } else {
        debug!("Not showing notification, notifications disabled in config");
    }
}
