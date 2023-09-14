use music_tools::audio::AudioPlayer;
use music_tools::chord::Chord;
use music_tools::common::{ArpeggioDirection, Beat};
use music_tools::pitchclass::PitchClasses;

fn main() {
    let mut player = AudioPlayer::new().unwrap();
    let tonic = PitchClasses::C;
    let progression = ["IV", "V", "iii", "vi", "I", "bVI", "bVII", "I"];
    let octaves = [4, 4, 4, 4, 4, 4, 4, 5];
    for (index, numeral) in progression.iter().enumerate() {
        let chord = Chord::from_numeral(numeral, tonic, Some(octaves[index])).unwrap();
        player.arpeggiate(chord, Beat::SIXTEENTH, ArpeggioDirection::Up, 8);
    }
}
