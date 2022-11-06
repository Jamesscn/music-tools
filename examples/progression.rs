extern crate musictools;

pub use std::time::Duration;
pub use rodio::{OutputStream, OutputStreamHandle, Sink};

pub use musictools::audio::{WavetableOscillator, Source};
pub use musictools::chord::{get_chord_from_numeral};
pub use musictools::pitchclass::{PitchClass, PitchClasses};
pub use musictools::common::Pentatonic;
pub use musictools::note::Note;
pub use musictools::scale::{get_scale, ScaleType};

fn play_note(note: &Note, stream_handle_ref: &OutputStreamHandle, sinks: &mut Vec<Sink>) {
    let oscillator = WavetableOscillator::new(128, note.get_frequency(), 44100);
    let sink = Sink::try_new(stream_handle_ref).unwrap();
    sink.append(oscillator);
    sinks.push(sink);
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
    let scale = get_scale(tonic, ScaleType::Major, Pentatonic::None);
    let progression = ["IV", "V", "iii", "vi", "I", "bVI", "bVII", "I"];
    let octaves = [4, 4, 4, 4, 4, 4, 4, 5];
    for (index, numeral) in progression.iter().enumerate() {
        let chord = get_chord_from_numeral(&scale, numeral);
        let octave = octaves[index];
        println!("Playing {}{}", chord.get_short_name(), octave);
        let mut notes_played = 0;
        'measure: loop {
            let notes = get_notes_with_octave(chord.get_pitch_classes(), octave);
            for note in notes {
                let mut sinks: Vec<Sink> = Vec::new();
                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                play_note(&note, &stream_handle, &mut sinks);
                std::thread::sleep(Duration::from_millis(150));
                notes_played += 1;
                if notes_played == 8 {
                    break 'measure;
                }
            }
        }
    }
}