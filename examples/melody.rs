extern crate musictools;

pub use std::time::Duration;
pub use rodio::{OutputStream, OutputStreamHandle, Sink};

pub use musictools::audio::{WavetableOscillator, Source};
pub use musictools::common::Fraction;
pub use musictools::note::Note;
pub use musictools::rhythm::{Rhythm, Beat};

fn main() {
    let mut rhythm = Rhythm::from(160.0, Fraction::new(5, 4), Vec::from(
        [Beat::quarter_dotted(), Beat::quarter_dotted(), Beat::quarter(), Beat::quarter()]
    ));
    let note_names = ["G4", "G4", "A#4", "C5", "G4", "G4", "F4", "F#4"];
    for index in 0..16 {
        let (_beat, seconds) = rhythm.get_next_beat();
        let note = Note::from_name(note_names[index % note_names.len()]).unwrap();
        let oscillator = WavetableOscillator::new(128, note.get_frequency(), 44100);
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(oscillator);
        std::thread::sleep(Duration::from_millis((seconds * 1000.0) as u64));
    }
}