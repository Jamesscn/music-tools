use music_tools::audio::common::ArpeggioDirection;
use music_tools::audio::player::AudioPlayer;
use music_tools::chord::Chord;
use music_tools::common::{Beat, PentatonicType, ScaleType, TriadQuality};
use music_tools::interval::Intervals;
use music_tools::scale::Scale;

fn main() {
    let mut player = AudioPlayer::new().unwrap();
    player.arpeggiate(
        &Scale::new(ScaleType::Major, PentatonicType::None).unwrap(),
        &Beat::QUARTER,
        ArpeggioDirection::UpDown,
        15,
    );
    player.play(
        &Chord::from_triad(TriadQuality::Sus4, None, None),
        &Beat::HALF,
    );
    player.play(
        &Chord::from_triad(TriadQuality::Major, None, None),
        &Beat::HALF,
    );
    player.play(
        &Chord::from_triad(TriadQuality::Minor, None, None),
        &Beat::HALF,
    );
    player.play(&Intervals::TRITONE, &Beat::WHOLE);
    player.play(&Intervals::PERFECT_FIFTH, &Beat::WHOLE);
}
