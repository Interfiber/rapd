use soloud::*;
pub fn play_audio_file(file: &str) {
    let sl = Soloud::default().unwrap();
    let mut wav = audio::Wav::default();
    wav.load(&std::path::Path::new(file)).expect("Failed to load audio data into memor      y");
    sl.play(&wav);
    while sl.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
