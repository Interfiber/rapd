use soloud::*;
fn main() {
    let mut sl = Soloud::default().unwrap();
    let mut wav = audio::Wav::default();
    wav.load(&std::path::Path::new("ost.mp3")).expect("Failed to load audio data into memory");
    sl.play(&wav);
    while sl.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
