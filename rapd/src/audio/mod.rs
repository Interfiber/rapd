// Audio backends for RAPD
// Example code:
// let backend = crate::audio::Backend::new();
// backend.load_file("/home/user/Music/test.mp3");
// backend.play_audio();

pub mod backends;
pub mod output;

pub trait AudioBackend {
    fn load_file(&mut self, file_path: String);
    fn play_audio(&mut self);
    fn pause_audio(&mut self);
    fn resume_audio(&mut self);
    fn stop_audio(&mut self);
    fn new() -> Self;
}
