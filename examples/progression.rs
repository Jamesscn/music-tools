use std::time::Duration;
use rodio::{OutputStream, Sink};
use musictools::audio::WavetableOscillator;
use musictools::chord::Chord;
use musictools::pitchclass::PitchClasses;
use musictools::note::Note;

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
    let octaves = [4, 4, 4, 4, 4, 4, 4, 5];
    for (index, numeral) in progression.iter().enumerate() {
        let notes = Chord::to_notes_from_numeral(numeral, Note::from(tonic, octaves[index])).unwrap();
        play_notes(notes, 1.0);
    }
}