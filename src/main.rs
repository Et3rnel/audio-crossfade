use std::fs;
use std::path::{Path, PathBuf};

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

    files.sort(); // Sorts the files alphabetically
    files
}

fn decode_mp3(file_path: &str) -> Vec<i16> {
    // Temporary dummy return value
    Vec::new() // Returns an empty vector of i16
}

fn apply_crossfade(
    samples_a: Vec<i16>,
    samples_b: Vec<i16>,
    crossfade_duration_ms: usize,
) -> Vec<i16> {
    // Temporary dummy return value
    Vec::new() // Returns an empty vector of i16
}

fn write_to_wav(output_path: &str, samples: Vec<i16>, sample_rate: u32) {
    // Since this function does not return a value, no changes are needed here.
}

fn main() {
    let dir = Path::new("audio");
    let files = list_mp3_files(dir);
    for file in files {
        println!("{:?}", file);
    }
}

