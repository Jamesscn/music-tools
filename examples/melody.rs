use music_tools::audio::{Waveforms, WavetableOscillator};
use music_tools::common::{Beat, Fraction};
use music_tools::note::Note;
use music_tools::track::Track;

fn main() {
    let mut track = Track::new(160.0, Fraction::new(5, 4));
    let beats = [
        Beat::QUARTER_DOTTED,
        Beat::QUARTER_DOTTED,
        Beat::QUARTER,
        Beat::QUARTER,
    ];
    let note_names = ["G4", "G4", "A#4", "C5", "G4", "G4", "F4", "F#4"];
    for index in 0..16 {
        let note = Note::from_string(note_names[index % note_names.len()]).unwrap();
        let duration = beats[index % beats.len()];
        track.add_note(note, duration);
    }
    let mut oscillator = WavetableOscillator::new();
    let square_wave_channel = oscillator.add_channel(Waveforms::SQUARE_WAVE, 1.0);
    oscillator
        .play_single_track(square_wave_channel, track)
        .unwrap();
}
