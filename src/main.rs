pub mod note;
pub mod audio;
pub mod chord;
pub mod scale;
pub mod enums;
pub mod pitchclass;
pub use std::time::Duration;
pub use rodio::{OutputStream, OutputStreamHandle, Sink};
pub use crate::note::Note;
pub use crate::audio::{WavetableOscillator, Source};

pub use chord::get_chord_with_quality;
pub use pitchclass::get_pitch_class;
pub use enums::{ChordQuality, Seventh};

fn play_note(note: &'static str, stream_handle_ref: &OutputStreamHandle, sinks: &mut Vec<Sink>) {
    let current_note = Note::new(&note);
    let oscillator = WavetableOscillator::new(128, current_note.frequency, 44100);
    let sink = Sink::try_new(stream_handle_ref).unwrap();
    sink.append(oscillator);
    sinks.push(sink);
}

fn main() {
    let tonic = get_pitch_class(String::from("C"));
    let chord = get_chord_with_quality(tonic, ChordQuality::Minor, Seventh::Minor, 0);
    let chord_pitch_classes = chord.get_pitch_classes();
    for pitch_class in chord_pitch_classes {
        let mut sinks: Vec<Sink> = Vec::new();
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let mut note_name: String = pitch_class.names[0].0.to_owned();
        let octave: &str = "4";
        note_name.push_str(octave);
        let note_name_str = Box::leak(note_name.into_boxed_str());
        println!("{0}", note_name_str);
        play_note(note_name_str, &stream_handle, &mut sinks);
        std::thread::sleep(Duration::from_secs(1));
    }
}