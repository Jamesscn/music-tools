use music_tools::audio::WavetableOscillator;
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
        track.add_chord(chord, Beat::HALF);
    }
    let mut oscillator = WavetableOscillator::new();
    let sine_wave_channel = oscillator.add_channel(f32::sin, 2.0 * std::f32::consts::PI);
    oscillator.play_single_track(sine_wave_channel, track);
}
