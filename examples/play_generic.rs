use music_tools::audio::common::ArpeggioDirection;
use music_tools::audio::player::AudioPlayer;
use music_tools::chord::Chord;
use music_tools::common::{Beat, TriadQuality};
use music_tools::interval::{PERFECT_FIFTH, TRITONE};
use music_tools::pitchclass::A_SHARP;
use music_tools::scale::MAJOR;

fn main() {
    let mut player = AudioPlayer::try_new().unwrap();
    player.push_arpeggiate(&MAJOR, &Beat::QUARTER, ArpeggioDirection::UpDown, 15);
    player.push(&Chord::from_triad(TriadQuality::Sus4), &Beat::HALF);
    player.push(&Chord::from_triad(TriadQuality::Major), &Beat::HALF);
    player.push(&Chord::from_triad(TriadQuality::Minor), &Beat::HALF);
    player.push(&TRITONE, &Beat::WHOLE);
    player.push(&PERFECT_FIFTH, &Beat::HALF);
    player.push(&A_SHARP, &Beat::QUARTER);
    player.play();
}
