use music_tools::audio::common::ArpeggioDirection;
use music_tools::audio::player::AudioPlayer;
use music_tools::chord::Chord;
use music_tools::common::{Beat, PentatonicType, ScaleType, TriadQuality};
use music_tools::interval::Intervals;
use music_tools::scale::Scale;

fn main() {
    let mut player = AudioPlayer::try_new().unwrap();
    player.push_arpeggiate(
        &Scale::try_new(ScaleType::Major, PentatonicType::None).unwrap(),
        &Beat::QUARTER,
        ArpeggioDirection::UpDown,
        15,
    );
    player.push(
        &Chord::from_triad(TriadQuality::Sus4, None, None),
        &Beat::HALF,
    );
    player.push(
        &Chord::from_triad(TriadQuality::Major, None, None),
        &Beat::HALF,
    );
    player.push(
        &Chord::from_triad(TriadQuality::Minor, None, None),
        &Beat::HALF,
    );
    player.push(&Intervals::TRITONE, &Beat::WHOLE);
    player.push(&Intervals::PERFECT_FIFTH, &Beat::WHOLE);
    player.play();
}
