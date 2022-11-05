use crate::{audio::AudioBackend, metadata::RapdMetadata};
use flume::Receiver;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::state::PLAYER;

lazy_static! {
    pub static ref BACKEND: Mutex<crate::audio::backends::symphonia::SymphoniaAudioBackend> =
        Mutex::new(crate::audio::backends::symphonia::SymphoniaAudioBackend::new());
}

#[derive(PartialEq, Eq, Clone)]
pub enum PlayerState {
    Playing,
    Paused,
    Idle,
}

impl PlayerState {
    pub fn serialize(&self) -> String {
        match self {
            PlayerState::Idle => String::from("playerstate::idle"),
            PlayerState::Paused => String::from("playerstate::paused"),
            PlayerState::Playing => String::from("playerstate::playing"),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RapdPlayerTime {
    pub hour: u64,
    pub min: u64,
    pub second: u64,
}

/// Handles audio playing for the server, in order for the main thread to interact with it, it must
/// be passed a std::sync::mpsc::channel.
pub struct RapdPlayer {
    state: PlayerState,
    time: RapdPlayerTime,
    file: String,
    metadata: Option<RapdMetadata>,
}

impl RapdPlayer {
    pub fn new() -> RapdPlayer {
        RapdPlayer {
            state: PlayerState::Idle,
            time: RapdPlayerTime {
                hour: 0,
                min: 0,
                second: 0,
            },
            metadata: Default::default(),
            file: String::from("No file"),
        }
    }

    /// Plays an audio file from disk, assumes server already checked if an audio file is already
    /// playing
    fn play_file(&mut self, file: &str, loop_audio: bool) {
        // load file
        info!("Starting audio playback for file: {}", file);

        while std::path::Path::new("/tmp/.rapd_backend_lock").exists() {
            warn!("Waiting for lock to be removed from backend");
            std::thread::sleep(Duration::from_millis(500));
        }

        let mut meta = RapdMetadata::new(String::from(file));
        meta.open(); // read metadata
        self.metadata = Some(meta);
        self.time = RapdPlayerTime {
            hour: 0,
            min: 0,
            second: 0,
        };

        self.file = String::from(file);

        let audio_file = file.to_string();

        std::thread::Builder::new()
            .name(String::from("audio_player"))
            .spawn(move || {
                let mut b = BACKEND.lock();
                unsafe {
                    BACKEND.force_unlock();
                }

                // load audio file
                b.load_file(audio_file.to_string());

                if loop_audio {
                    // play audio for the first time
                    b.play_audio();

                    while !b.stopped {
                        // reload file into memory
                        b.load_file(audio_file.to_string());

                        // replay the audio
                        b.play_audio();
                    }
                } else {
                    // play audio
                    b.play_audio();
                }

                // update states
                b.paused = false;
                b.stopped = true;

                info!("Audio playback done");
            })
            .expect("Failed to spawn audio player thread");

        self.state = PlayerState::Playing;

        crate::notifications::alert_play_file();
        info!("Started audio playback");
    }

    /// Get position in song
    pub fn get_time(&self) -> RapdPlayerTime {
        self.time.to_owned()
    }

    /// Get the total length of the song
    pub fn get_length(&self) -> &RapdPlayerTime {
        if self.metadata.is_none() {
            &RapdPlayerTime {
                hour: 0,
                min: 0,
                second: 0,
            }
        } else {
            self.metadata.as_ref().unwrap().get_length()
        }
    }

    /// Get the current state of the player
    pub fn get_state(&self) -> PlayerState {
        self.state.to_owned()
    }

    /// Get the current playing file
    pub fn get_file(&self) -> String {
        self.file.to_owned()
    }

    /// Updates the position in the track, and updates states as needed
    fn update_audio(&mut self) {
        let b = BACKEND.lock();

        if b.stopped {
            self.state = PlayerState::Idle;
            self.file = String::from("No file");
        }

        if b.paused {
            self.state = PlayerState::Paused;
            self.file = String::from("No file");
        }

        if self.state != PlayerState::Playing {
            return;
        }

        let len = self.get_length();

        if self.time.hour >= len.hour && self.time.min >= len.min && self.time.second >= len.second
        {
            self.time = RapdPlayerTime {
                hour: 0,
                min: 0,
                second: 0,
            };
        }

        self.time.second += 1;

        if self.time.second > 60 {
            self.time.second = 0;
            self.time.min += 1;
        }

        if self.time.min > 60 {
            self.time.min = 0;
            self.time.hour += 1;
        }
    }

    /// Pauses the audio player
    fn pause_player(&mut self) {
        self.state = PlayerState::Paused;
        BACKEND.lock().pause_audio();
        crate::notifications::alert_paused_player();
        info!("Paused player");
    }

    // Unpauses the audio player
    fn unpause_player(&mut self) {
        self.state = PlayerState::Playing;
        BACKEND.lock().resume_audio();
        crate::notifications::alert_paused_player();
        info!("Unpaused player");
    }

    /// Stops the player
    fn stop_player(&mut self) {
        self.state = PlayerState::Idle;
        self.time = RapdPlayerTime {
            hour: 0,
            min: 0,
            second: 0,
        };

        BACKEND.lock().stop_audio();
        crate::notifications::alert_player_stop();
        info!("Stopped player");
    }

    /// Get the metadata for the current player
    pub fn get_metadata(&self) -> RapdMetadata {
        if self.metadata.is_none() {
            RapdMetadata::new(String::from("No file"))
        } else {
            self.metadata.as_ref().unwrap().to_owned()
        }
    }

    /// Start the RapdPlayer, should be called on another thread seperate from the main thread in
    /// order to not block it
    pub fn start(&mut self, receiver: Receiver<String>) {
        warn!("Using unsafe PLAYER.force_unlock()");

        unsafe {
            PLAYER.force_unlock();
        }

        info!("Force unlocked player");

        loop {
            let received = receiver.recv_timeout(Duration::from_secs(1));
            if let Ok(text) = received {
                info!("Player received command: {}", text);

                // parse command
                let cmd: Vec<&str> = text.split(':').collect();

                if cmd.is_empty() || cmd.len() == 1 {
                    error!("Invalid command: Length is 0");
                    break;
                }

                let params: Vec<&str> = cmd[1].split(',').collect();

                // big if statement of doom

                if cmd[0] == "log" {
                    match params[0] {
                        "start" => info!("Player thread started!"),
                        _ => error!("Got unknown param for log command"),
                    }
                } else if cmd[0] == "play_file" {
                    self.play_file(
                        &params[0].replace("\\COMMA", ","),
                        params[1].parse::<bool>().unwrap(),
                    );
                } else if cmd[0] == "pause_player" {
                    self.pause_player();
                } else if cmd[0] == "unpause_player" {
                    self.unpause_player();
                } else if cmd[0] == "stop_player" {
                    self.stop_player();
                }
            } else {
                // update audio loop
                self.update_audio();
            }
        }
    }
}
