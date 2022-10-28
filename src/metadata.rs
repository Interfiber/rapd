use std::path::Path;

use lofty::{AudioFile, Probe};

use crate::player::RapdPlayerTime;

pub struct RapdMetadata {
    file: String,
    length: RapdPlayerTime,
    title: String
}

impl RapdMetadata {
    pub fn new(path: String) -> RapdMetadata {
        RapdMetadata {
            file: path,
            length: RapdPlayerTime {
                hour: 0,
                min: 0,
                second: 0,
            },
            title: String::from("Unknown title")
        }
    }

    pub fn open(&mut self) {
        let path = Path::new(&self.file);

        trace!("Opening tagged file");
        let tagged_file = Probe::open(path)
            .expect("Failed to open path")
            .read(true)
            .expect("Failed to read file");

        trace!("Getting properties for file");
        let properties = tagged_file.properties();

        trace!("Getting duration from properties");
        let duration = properties.duration();

        trace!("Updating metadata");

        self.length = RapdPlayerTime {
            hour: (duration.as_secs() / 60) / 60,
            min: duration.as_secs() / 60,
            second: duration.as_secs() % 60,
        }
    }

    pub fn get_length(&self) -> &RapdPlayerTime {
        &self.length
    }
}
