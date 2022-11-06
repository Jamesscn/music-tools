pub mod note;
pub mod audio;
pub mod chord;
pub mod scale;
pub mod rhythm;
pub mod common;
pub mod pitchclass;

pub use std::time::Duration;
pub use rodio::{OutputStream, OutputStreamHandle, Sink};
pub use audio::{WavetableOscillator, Source};

pub use chord::get_chord_with_quality;
pub use pitchclass::{PitchClass, PitchClasses};
pub use common::{ChordQuality, Seventh, Fraction};
pub use note::Note;
pub use scale::{get_scale, ScaleType};
pub use rhythm::{Rhythm, Beat};

fn play_note(note: &Note, stream_handle_ref: &OutputStreamHandle, sinks: &mut Vec<Sink>) {
    let oscillator = WavetableOscillator::new(128, note.frequency, 44100);
    let sink = Sink::try_new(stream_handle_ref).unwrap();
    sink.append(oscillator);
    sinks.push(sink);
}

fn _get_notes_with_octave(pitch_classes: Vec<&'static PitchClass>, starting_octave: u8) -> Vec<Note> {
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
    let mut rhythm = Rhythm::from(160.0, Fraction::new(5, 4), Vec::from(
        [Beat::quarter_dotted(), Beat::quarter_dotted(), Beat::quarter(), Beat::quarter()]
    ));
    let note_names = ["G4", "G4", "A#4", "C5", "G4", "G4", "F4", "F#4"];
    for index in 0..16 {
        let mut sinks: Vec<Sink> = Vec::new();
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let (_beat, seconds) = rhythm.get_next_beat();
        let note = Note::from_name(note_names[index % note_names.len()]);
        play_note(&note, &stream_handle, &mut sinks);
        std::thread::sleep(Duration::from_millis((seconds * 1000.0) as u64));
    }
}