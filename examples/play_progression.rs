use music_tools::audio::common::ArpeggioDirection;
use music_tools::audio::player::{AudioPlayer, BitsPerSample};
use music_tools::chord::Chord;
use music_tools::common::Beat;
use music_tools::pitchclass::PitchClasses;

fn main() {
    let mut player = AudioPlayer::try_new().unwrap();
    let tonic = PitchClasses::C;
    let progression = ["IV", "V", "iii", "vi", "I", "bVI", "bVII", "I"];
    let octaves = [4, 4, 4, 4, 4, 4, 4, 5];
    for (index, numeral) in progression.iter().enumerate() {
        let chord = Chord::from_numeral(numeral, tonic, Some(octaves[index])).unwrap();
        player.push_arpeggiate(&chord, &Beat::SIXTEENTH, ArpeggioDirection::Up, 8);
    }
    player.play();
    player
        .export_wav("example_play_progression.wav", BitsPerSample::TWENTYFOUR)
        .unwrap();
}
