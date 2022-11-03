pub mod note;
pub mod audio;
pub mod pitchclass;
pub use std::time::Duration;
pub use rodio::OutputStream;
pub use crate::note::Note;
pub use crate::audio::{WavetableOscillator, Source};

fn main() {
    let notes = ["A4", "A#4", "B4", "C5", "C#5", "D5", "D#5", "E5", "F5", "F#5", "G5", "G#5", "A5"];
    for note in notes {
        let current_note = Note::new(note);
        let oscillator = WavetableOscillator::new(64, current_note.frequency, 44100);
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let _result = stream_handle.play_raw(oscillator.convert_samples());
        std::thread::sleep(Duration::from_secs(1));
    }
}