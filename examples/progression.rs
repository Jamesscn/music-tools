extern crate musictools;

pub use std::time::Duration;
pub use rodio::{OutputStream, OutputStreamHandle, Sink};

pub use musictools::audio::WavetableOscillator;
pub use musictools::chord::Chord;
pub use musictools::pitchclass::{PitchClass, PitchClasses};
pub use musictools::note::Note;
pub use musictools::scale::Scale;

fn play_notes(notes: Vec<Note>, seconds: f32) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut oscillator = WavetableOscillator::new(128, 44100);
    for note in notes {
        oscillator.add_frequency(note.get_frequency());
    }
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(oscillator);
    std::thread::sleep(Duration::from_millis((seconds * 1000.0) as u64));
}

fn main() {
    let tonic = PitchClasses::C;
    let progression = ["IV", "V", "iii", "vi", "I", "bVI", "bVII", "I"];
    let octaves = [4, 4, 4, 4, 4, 4, 4, 4];
    for (index, numeral) in progression.iter().enumerate() {
        let chord = Chord::from_numeral(tonic, numeral).unwrap();
        play_notes(chord.to_notes(octaves[index]), 1.0);
    }
}