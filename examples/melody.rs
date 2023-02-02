extern crate musictools;

pub use std::time::Duration;
pub use rodio::{OutputStream, OutputStreamHandle, Sink};

pub use musictools::audio::WavetableOscillator;
pub use musictools::common::Fraction;
pub use musictools::note::Note;
pub use musictools::rhythm::{Rhythm, Beat};

fn main() {
    let mut rhythm = Rhythm::from(160.0, Fraction::new(5, 4), Vec::from(
        [Beat::quarter_dotted(), Beat::quarter_dotted(), Beat::quarter(), Beat::quarter()]
    ));
    let note_names = ["G4", "G4", "A#4", "C5", "G4", "G4", "F4", "F#4"];
    for index in 0..16 {
        let note = Note::from_string(note_names[index % note_names.len()]).unwrap();
        let mut oscillator = WavetableOscillator::new(128, 44100);
        oscillator.add_frequency(note.get_frequency());
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(oscillator);
        let millisecond_duration = rhythm.get_duration_of_current_beat() * 1000.0;
        std::thread::sleep(Duration::from_millis(millisecond_duration as u64));
        rhythm.next_position();
    }
}