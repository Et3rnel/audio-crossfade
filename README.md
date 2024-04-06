# Lo-Fi Music Merger with Crossfade

## Overview

This Rust project merges multiple lo-fi studying music tracks into a single audio file, applying a smooth crossfade effect between each track. It's designed for creators and enthusiasts looking to generate extended playlists for studying, relaxation, or background music. The program reads MP3 files, decodes them to PCM samples, applies crossfade effects, and outputs the merged audio as a WAV file.

## Features

- **MP3 to WAV Conversion**: Decodes MP3 files to WAV format for processing.
- **Crossfade Effects**: Smoothly transitions between tracks using a customizable crossfade duration.
- **Batch Processing**: Merges multiple tracks from a specified directory, processing them in alphabetical order.
- **High-Quality Output**: Supports 48 kHz sample rate for high-quality audio output.

## Requirements

- Rust Programming Language
- `minimp3` for decoding MP3 files
- `hound` for encoding WAV files

## Setup and Usage

1. **Installation**: Ensure Rust is installed on your system. Clone this repository and navigate to the project directory.

    ```bash
    git clone https://github.com/yourusername/lofi-music-merger.git
    cd lofi-music-merger
    ```

2. **Adding MP3 Files**: Place your MP3 tracks in the `audio` directory. The program processes files in alphabetical order.

3. **Configuration**: Adjust the sample rate and crossfade duration in `main.rs` if different from the default settings.

4. **Building and Running**:

   Compile the project with Cargo:

    ```bash
    cargo build --release
    ```

   Run the program:

    ```bash
    cargo run --release
    ```

   The merged output will be saved as `audio/output.wav`.

## Customization

- **Crossfade Duration**: Modify the `crossfade_duration_ms` constant in `main.rs` to change the length of the crossfade effect between tracks.

- **Sample Rate**: Adjust the `SAMPLE_RATE` constant for compatibility with tracks of different sample rates.

## Contributions

Contributions are welcome! Whether it's feature requests, bug reports, or pull requests, feel free to contribute to this project by opening an issue or submitting a pull request on GitHub.

## License

This project is open-source and available under the MIT License. See the LICENSE file for more details.
