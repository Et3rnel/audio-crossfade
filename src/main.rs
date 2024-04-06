use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use minimp3::{Decoder, Error, Frame};

const SAMPLE_RATE: u32 = 48000; // 48 kHz

fn list_mp3_files(dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = fs::read_dir(dir)
        .expect("Failed to read directory")
        .filter_map(|entry| {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("mp3") {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    for file in &files {
        println!("Found MP3 file: {:?}", file);
    }

    files.sort(); // Sorts the files alphabetically
    files
}

fn decode_mp3(file_path: &Path) -> Result<Vec<i16>, Error> {
    println!("Decoding MP3 file: {:?}", file_path);

    let mut decoder = Decoder::new(File::open(file_path)?);
    let mut samples: Vec<i16> = Vec::new();

    while let Ok(Frame { data, .. }) = decoder.next_frame() {
        samples.extend(data);
    }

    println!("Decoding complete: {:?}", file_path);
    Ok(samples)
}

fn apply_crossfade(tracks: Vec<Vec<i16>>, sample_rate: u32, crossfade_duration_ms: u32) -> Vec<i16> {
    println!("Starting crossfade process for {} tracks.", tracks.len());

    if tracks.len() < 2 {
        return tracks.into_iter().flatten().collect();
    }

    let crossfade_samples = (sample_rate as u32 * crossfade_duration_ms / 1000) as usize;
    let mut output = tracks[0].clone(); // Start with the first track

    for track in tracks.into_iter().skip(1) {
        let track_len = track.len();
        let fade_in_end = std::cmp::min(crossfade_samples, track_len);

        // Pre-calculate crossfade and next track remainder
        let mut crossfade = Vec::with_capacity(crossfade_samples);
        let next_track_remainder = if crossfade_samples < track_len { &track[crossfade_samples..] } else { &[][..] };

        for j in 0..crossfade_samples {
            let t = j as f32 / crossfade_samples as f32; // Transition factor from 0.0 to 1.0
            let sample_a = output.get(output.len() - crossfade_samples + j).copied().unwrap_or(0) as f32 * (1.0 - t);
            let sample_b = track.get(j).copied().unwrap_or(0) as f32 * t;
            crossfade.push((sample_a + sample_b) as i16);
        }

        // Apply changes to output
        output.truncate(output.len() - crossfade_samples); // Safe to truncate now
        output.extend(crossfade); // Apply pre-calculated crossfade
        output.extend_from_slice(next_track_remainder); // Append remainder of the track
    }

    println!("Crossfade process complete.");
    output
}
fn write_to_wav(output_path: &str, samples: Vec<i16>, sample_rate: u32) {
    println!("Writing to WAV file: {}", output_path);

    let spec = hound::WavSpec {
        channels: 2, // Stereo audio
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(output_path, spec).expect("Failed to create WAV writer");

    for sample in samples {
        writer.write_sample(sample).expect("Failed to write sample");
    }

    writer.finalize().expect("Failed to finalize WAV file");
    println!("WAV file written: {}", output_path);
}

fn main() {
    println!("Starting audio merge process...");

    let dir = Path::new("audio");
    let files = list_mp3_files(dir);
    let mut tracks = Vec::new();

    for file_path in files {
        let track_samples = decode_mp3(&file_path).expect("Error decoding track");
        tracks.push(track_samples);
    }

    let crossfade_duration_ms = 5000; // 5-second crossfade
    let merged_samples = apply_crossfade(tracks, SAMPLE_RATE, crossfade_duration_ms);

    write_to_wav("audio/output.wav", merged_samples, SAMPLE_RATE);

    println!("Audio merge process complete. Output written to audio/output.wav");
}
