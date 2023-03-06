use musictools::audio::{WavetableOscillator, Waveforms, play};
use musictools::common::{Fraction, Beat};
use musictools::note::Note;
use musictools::track::Track;

fn main() {
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
    play(oscillator, track);
}