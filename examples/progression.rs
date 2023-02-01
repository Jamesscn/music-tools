extern crate musictools;

pub use std::time::Duration;
pub use rodio::{OutputStream, OutputStreamHandle, Sink};

pub use musictools::audio::{WavetableOscillator, Source};
pub use musictools::chord::Chord;
pub use musictools::pitchclass::{PitchClass, PitchClasses};
pub use musictools::common::PitchQuality;
pub use musictools::note::Note;
pub use musictools::scale::{get_scale, ScaleType};

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

fn get_notes_with_octave(pitch_classes: Vec<&'static PitchClass>, starting_octave: u8) -> Vec<Note> {
    let mut notes: Vec<Note> = Vec::new();
    let mut octave: u8 = starting_octave;
    let mut last_value: i8 = -1;
    for pitch_class in pitch_classes {
        let pitch_class_value = pitch_class.get_value();
        if pitch_class_value as i8 <= last_value {
            octave += 1;
        }
        let note = Note::from(pitch_class, octave);
        notes.push(note);
        last_value = pitch_class_value as i8;
    }
    return notes;
}

fn main() {
    let tonic = PitchClasses::C;
    let progression = ["IV", "V", "iii", "vi", "I", "bVI", "bVII", "I"];
    let octaves = [4, 4, 4, 4, 4, 4, 4, 5];
    for (index, numeral) in progression.iter().enumerate() {
        let chord = Chord::from_numeral(&tonic, numeral).unwrap();
        let octave = octaves[index];
        //println!("Playing {}{}", chord.get_short_name(), octave);
        let notes = get_notes_with_octave(chord.get_pitch_classes(), octave);
        play_notes(notes, 1.0);
    }
}