use crate::enums::MetadataEditState;
use audiotags::Tag;
// returns the title of the sound file given, unknown if empty
pub fn get_title(path: String) -> String {
    // TODO: Send an error message to the client
    info!("Reading metadata from on disk file...");
    let tag = Tag::new()
        .read_from_path(path)
        .expect("Failed to read music file from disk");
    info!("Attempting to get title from Tag");
    let title = tag.title();
    if title.is_none() {
        warn!("Got nothing as the result, returning 'unknown'");
        return String::from("unknown");
    } else {
        return title.unwrap().to_string();
    }
}

pub fn set_title(path: String, value: String) {
    info!("Attempting to set title of {}", path);
    let mut tag = Tag::new()
        .read_from_path(&path)
        .expect("Failed to read music file from disk");
    tag.set_title(&value);
    // save the metadata
    match tag.write_to_path(&path) {
        Ok(_) => info!("Wrote metadata to disk"),
        Err(err) => {
            error!("Failed to write metadata to disk");
            error!("Error log: {}", err);
        }
    }
    info!("Set title to {}", value);
    // TODO: Make sure the title was set
}

pub fn set_author(path: String, value: String) -> MetadataEditState {
    info!("Attempting to set author of {}", path);
    let mut tag = Tag::new()
        .read_from_path(&path)
        .expect("Failed to read file from disk");
    tag.set_artist(&value);
    match tag.write_to_path(&path) {
        Ok(_) => {
            info!("Wrote metadata to disk");
            return MetadataEditState::Wrote;
        }
        Err(err) => {
            error!("Failed to write metadata to disk");
            error!("Error log: {}", err);
            return MetadataEditState::MetadataWriteError;
        }
    }
}

pub fn get_author(path: String) -> String {
    let tag = Tag::new()
        .read_from_path(&path)
        .expect("Failed to read file from disk");
    let author = tag.artist();
    if author.is_none() {
        warn!("Got nothing, returning 'unknown'");
        return String::from("unknown");
    } else {
        return author.unwrap().to_string();
    }
}
