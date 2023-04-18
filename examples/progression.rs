use music_tools::audio::{Waveforms, WavetableOscillator};
use music_tools::chord::Chord;
use music_tools::common::{Beat, Fraction};
use music_tools::pitchclass::PitchClasses;
use music_tools::track::Track;

fn main() {
    let tonic = PitchClasses::C;
    let progression = ["IV", "V", "iii", "vi", "I", "bVI", "bVII", "I"];
    let octaves = [4, 4, 4, 4, 4, 4, 4, 5];
    let mut track = Track::new(120.0, Fraction::new(4, 4));
    for (index, numeral) in progression.iter().enumerate() {
        let chord = Chord::from_numeral(numeral, tonic, Some(octaves[index])).unwrap();
        track.add_chord(chord, Beat::HALF).unwrap();
    }
    let mut oscillator = WavetableOscillator::new();
    let sine_wave_channel = oscillator.add_channel(Waveforms::SINE_WAVE, 1.0);
    oscillator
        .play_tracks(vec![sine_wave_channel], vec![track])
        .expect("Could not play the example progression!");
}
