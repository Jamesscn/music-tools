use musictools::chord::Chord;
use musictools::track::Track;
use musictools::common::{Beat, Fraction};
use musictools::pitchclass::PitchClasses;
use musictools::audio::WavetableOscillator;

fn main() {
    let tonic = PitchClasses::C;
    let progression = ["IV", "V", "iii", "vi", "I", "bVI", "bVII", "I"];
    let octaves = [4, 4, 4, 4, 4, 4, 4, 5];
    let mut track = Track::new(120.0, Fraction::new(4, 4), 360);
    for (index, numeral) in progression.iter().enumerate() {
        let chord = Chord::from_numeral(numeral, tonic, Some(octaves[index])).unwrap();
        track.add_chord(chord, Beat::HALF);
    }
    let mut oscillator = WavetableOscillator::new(128, 44100);
    oscillator.play(track);
}