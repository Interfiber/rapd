use soloud::*;
use std::thread::Builder;
use std::path::Path;
use crate::requests::AudioPlayRequest;
pub fn play_audio_file(file: &str) {
    let sl = Soloud::default().unwrap();
    let mut wav = audio::Wav::default();
    wav.load(&std::path::Path::new(file)).expect("Failed to load audio data into memor      y");
    sl.play(&wav);
    while sl.voice_count() > 0 {
        if Path::new("rapd.stop_player_thread").exists() {
            info!("File rapd.stop_player_thread exists, shutting down player thread");
            std::fs::remove_file("rapd.stop_player_thread").unwrap();
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

pub fn play_audio_from_request(request: AudioPlayRequest) {
    info!("Spawning audio thread...");    
    Builder::new().name("player".to_string()).spawn(move || play_audio_file(&request.audio_file_path));
}
