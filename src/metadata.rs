use std::path::Path;

use lofty::{AudioFile, Probe, Accessor, PictureType, MimeType};
use serde::{Serialize, Deserialize};

use crate::player::RapdPlayerTime;

#[derive(Serialize, Deserialize)]
pub struct RapdMetadata {
    file: String,
    length: RapdPlayerTime,
    title: String,
    artist: String,
    album: String,
    album_art: String
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
            title: String::from("Unknown title"),
            artist: String::from("Unknown artist"),
            album: String::from("Unknown album"),
            album_art: String::from("no_art")
        }
    }

    fn get_album_art_cache() -> String {
        let xdg_dir = xdg::BaseDirectories::with_prefix("rapd").unwrap();
        let cache_home = xdg_dir.get_cache_home().to_str().unwrap().to_string();

        if xdg_dir.find_cache_file("album_art").is_none() {
            error!("No album art cache folder found, creating one");

            std::fs::create_dir_all(format!("{}/album_art", cache_home)).expect("Failed to create album art cache");
        }

        format!("{}/album_art", cache_home)
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

        trace!("Getting primary tag");
        let tag = match tagged_file.primary_tag() {
            Some(primary_tag) => primary_tag,
            None => tagged_file.first_tag().expect("Failed to get first metadata tag for file")
        };

        trace!("Updating metadata");

        self.length = RapdPlayerTime {
            hour: (duration.as_secs() / 60) / 60,
            min: duration.as_secs() / 60,
            second: duration.as_secs() % 60,
        };

        self.title = String::from(tag.title().unwrap_or("Unknown title"));
        self.artist = String::from(tag.artist().unwrap_or("Unknown artist"));
        self.album = String::from(tag.album().unwrap_or("Unknown album"));

        if tag.pictures().len() > 0 {

            debug!("Extracting album art");
            let pic_data = tag.pictures()[0].data();
            let album_art_dir = Self::get_album_art_cache();
            let mime = tag.pictures()[0].mime_type();
            let mut ext = "png";

            if mime == &MimeType::Jpeg {
                ext = "jpg";
            } else if mime == &MimeType::None {
                error!("Invalid mime type for image!");
                return;
            }

            let album_art_file = format!("{}/{}_{}_{}.{}", album_art_dir, self.title, self.artist, self.album, ext);

            trace!("Checking for file: {}", album_art_file);

            if !Path::new(&album_art_file).exists() {
                trace!("Writing extracted data to disk");

                std::fs::write(album_art_file.to_string(), pic_data).expect("Failed to write image");
            }

            self.album_art = album_art_file;
        }
    }

    pub fn get_length(&self) -> &RapdPlayerTime {
        &self.length
    }
}
