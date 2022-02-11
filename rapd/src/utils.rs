use std::path::Path;

pub fn file_exists(path: String) -> bool {
    if Path::new(&path).exists(){
        return true;
    } else {
        return false;
    }
}

pub fn get_default_music_dir() -> String {
    // the default music dir is ~/Audio
    let mut audio_dir = home::home_dir().unwrap();
    audio_dir.push("Audio");
    return audio_dir.into_os_string().into_string().unwrap();
}

pub fn is_directory(path: String) -> bool {
    return Path::new(&path).is_dir();
}
