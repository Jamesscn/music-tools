use rodio::{OutputStream, Sink};
use musictools::audio::{WavetableOscillator, Waveforms};
use musictools::common::{Fraction, Beat};
use musictools::note::Note;
use musictools::track::Track;

fn main() {
    let stream_result = OutputStream::try_default();
    if stream_result.is_err() {
        println!("No sound card detected!");
        return;
    }
    let (_stream, stream_handle) = stream_result.unwrap();
    let sink_result = Sink::try_new(&stream_handle);
    if sink_result.is_err() {
        println!("Could not create a sink!");
        return;
    }
    let sink = sink_result.unwrap();
    let mut track = Track::new(160.0, Fraction::new(5, 4), 360);
    let beats = [Beat::QUARTER_DOTTED, Beat::QUARTER_DOTTED, Beat::QUARTER, Beat::QUARTER];
    let note_names = ["G4", "G4", "A#4", "C5", "G4", "G4", "F4", "F#4"];
    for index in 0..16 {
        let note = Note::from_string(note_names[index % note_names.len()]).unwrap();
        let duration = beats[index % beats.len()];
        track.add_note(note, duration);
    }
    let mut oscillator = WavetableOscillator::new(128, 44100);
    oscillator.set_wave_function(Waveforms::SQUARE_WAVE, 1.0);
    oscillator.play(track);
    sink.append(oscillator);
}