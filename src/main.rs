pub mod note;
pub mod audio;
pub mod chord;
pub mod scale;
pub mod enums;
pub mod pitchclass;

pub use std::time::Duration;
pub use rodio::{OutputStream, OutputStreamHandle, Sink};
pub use audio::{WavetableOscillator, Source};

pub use chord::get_chord_with_quality;
pub use pitchclass::PitchClasses;
pub use enums::{ChordQuality, Seventh};
pub use note::Note;

fn play_note(note: &Note, stream_handle_ref: &OutputStreamHandle, sinks: &mut Vec<Sink>) {
    let oscillator = WavetableOscillator::new(128, note.frequency, 44100);
    let sink = Sink::try_new(stream_handle_ref).unwrap();
    sink.append(oscillator);
    sinks.push(sink);
}

fn main() {
    let tonic = PitchClasses::D_FLAT;
    let chord = get_chord_with_quality(tonic, ChordQuality::Minor, Seventh::Major, 1);
    let chord_pitch_classes = chord.get_pitch_classes();
    println!("Playing {0}", chord.get_short_name());
    for pitch_class in chord_pitch_classes {
        let mut sinks: Vec<Sink> = Vec::new();
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let note = Note::from(pitch_class, 4);
        play_note(&note, &stream_handle, &mut sinks);
        std::thread::sleep(Duration::from_secs(1));
    }
}