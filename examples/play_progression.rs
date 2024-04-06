use music_tools::audio::common::ArpeggioDirection;
use music_tools::audio::player::{AudioPlayer, BitsPerSample};
use music_tools::chord::Chord;
use music_tools::common::Beat;

fn main() {
    let mut player = AudioPlayer::try_new().unwrap();
    for (index, numeral) in ["IV", "V", "iii", "vi", "I", "bVI", "bVII", "I"]
        .iter()
        .enumerate()
    {
        let base_note = if index < 7 { "C4" } else { "C5" };
        let chord = Chord::from_numeral(numeral, base_note).unwrap();
        player.push_arpeggiate(&chord, &Beat::SIXTEENTH, ArpeggioDirection::Up, 8);
    }
    player.play();
    player
        .export_wav("example_play_progression.wav", BitsPerSample::TWENTYFOUR)
        .unwrap();
}
