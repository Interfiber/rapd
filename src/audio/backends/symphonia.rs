// Justification: Fields on DecoderOptions and FormatOptions may change at any time, but
// symphonia-play doesn't want to be updated every time those fields change, therefore always fill
// in the remaining fields with default values.

#![allow(clippy::needless_update)]

use crate::audio::{output, AudioBackend};

use std::fs::File;
use std::path::Path;
use std::time::Duration;

use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::formats::{FormatOptions, Track};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::{Hint, ProbeResult};

pub struct SymphoniaAudioBackend {
    pub probe: Option<ProbeResult>,
    pub decode_opts: Option<DecoderOptions>,
    pub stopped: bool,
    pub paused: bool,
}

fn first_supported_track(tracks: &[Track]) -> Option<&Track> {
    tracks
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
}

/// Audio backend for Symphonia
impl AudioBackend for SymphoniaAudioBackend {
    fn load_file(&mut self, file_path: String) {
        let mut hint = Hint::new();

        trace!("Creating source");
        let source = {
            trace!("Creating source path");
            let path = Path::new(&file_path);

            // Provide the file extension as a hint.
            trace!("Adding file extensions as hint");
            if let Some(extension) = path.extension() {
                if let Some(extension_str) = extension.to_str() {
                    hint.with_extension(extension_str);
                }
            }

            trace!("Opening audio file for reading");
            Box::new(File::open(path).unwrap())
        };

        trace!("Creating MediaSourceStream");
        let mss = MediaSourceStream::new(source, Default::default());

        let format_opts = FormatOptions {
            enable_gapless: true,
            ..Default::default()
        };
        trace!("Using FormatOptions: {:#?}", format_opts);

        let metadata_opts: MetadataOptions = Default::default();

        trace!("Using MetadataOptions: {:#?}", metadata_opts);

        let decode_opts = DecoderOptions {
            verify: false,
            ..Default::default()
        };

        trace!("Using DecoderOptions: {:#?}", decode_opts);

        trace!("Probing media source stream for metadata, and format reader");
        match symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts) {
            Ok(probed) => {
                trace!("Updating global probe info");
                self.probe = Some(probed);
                self.decode_opts = Some(decode_opts);
                trace!("Finished probing");
            }
            Err(err) => {
                error!("Failed to probe media source, error: {}", err);
            }
        }

        trace!("Getting length ");
    }

    fn play_audio(&mut self) {
        trace!("Starting audio playback");
        self.stopped = false;
        self.paused = false;
        let probe = self.probe.as_mut().unwrap();

        trace!("Finding first support track");
        let track = first_supported_track(probe.format.tracks());

        trace!("Finding track ID");
        let track_id = match track {
            Some(track) => track.id,
            _ => {
                error!("Failed to get track ID");
                return;
            }
        };

        let seek_time = 0;

        let mut audio_output: Option<Box<dyn output::AudioOutput>> = None;

        let track = match probe
            .format
            .tracks()
            .iter()
            .find(|track| track.id == track_id)
        {
            Some(track) => track,
            _ => {
                error!("Failed to get track");
                std::process::exit(-1);
            }
        };

        trace!("Found track: {:#?}", track);

        trace!("Creating decoder");
        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &self.decode_opts.unwrap())
            .expect("Failed to create decoder");

        trace!("Starting audio decoding");

        loop {
            trace!("Pause state: {}", self.paused);
            #[allow(clippy::while_immutable_condition)]
            while self.paused {
                info!("Playback paused");
                self.stopped = false;
                std::thread::sleep(Duration::from_millis(500));
            }

            if self.stopped {
                info!("Audio stopped");
                self.paused = false;
                self.stopped = true;
                break;
            }

            let packet = match probe.format.next_packet() {
                Ok(packet) => packet,
                Err(err) => {
                    error!("Failed to get next packet, error: {}", err);
                    break;
                }
            };

            if packet.track_id() != track_id {
                info!("Skipping packet with invalid track ID");
                continue;
            }

            match decoder.decode(&packet) {
                Ok(decoded) => {
                    if audio_output.is_none() {
                        trace!("Opening audio output");

                        let spec = *decoded.spec();
                        let duration = decoded.capacity() as u64;

                        audio_output.replace(
                            output::try_open(spec, duration).expect("Failed to open audio output"),
                        );

                        trace!("Opened audio output");
                    }

                    if packet.ts() >= seek_time {
                        if let Some(ref mut audio_output) = audio_output {
                            audio_output
                                .write(decoded)
                                .expect("Failed to write audio packet");
                        }
                    }
                }
                Err(err) => {
                    error!("Failed to decode packet: {}", err);
                }
            }
        }
    }

    fn stop_audio(&mut self) {
        info!("Signaling for audio decoding backend to stop");
        self.stopped = true;
    }

    fn pause_audio(&mut self) {
        info!("Backend is pausing audio");
        self.paused = true;
    }

    fn resume_audio(&mut self) {
        self.paused = false;
        self.stopped = false;
    }

    fn new() -> Self {
        Self {
            probe: Default::default(),
            decode_opts: Default::default(),
            paused: false,
            stopped: true,
        }
    }
}
