pub mod note;
pub mod audio;
pub mod scale;
pub mod pitchclass;
pub use std::time::Duration;
pub use rodio::{OutputStream, OutputStreamHandle, Sink};
pub use crate::note::Note;
pub use crate::audio::{WavetableOscillator, Source};

pub use crate::scale::get_scale;
pub use crate::pitchclass::get_pitch_class;

fn _play_note(note: &'static str, stream_handle_ref: &OutputStreamHandle, sinks: &mut Vec<Sink>) {
    let current_note = Note::new(&note);
    let oscillator = WavetableOscillator::new(128, current_note.frequency, 44100);
    let sink = Sink::try_new(stream_handle_ref).unwrap();
    sink.append(oscillator);
    sinks.push(sink);
}

fn main() {
    let mut sinks: Vec<Sink> = Vec::new();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    play_note("C4", &stream_handle, &mut sinks);
    play_note("E4", &stream_handle, &mut sinks);
    play_note("G4", &stream_handle, &mut sinks);
    std::thread::sleep(Duration::from_secs(3));
}