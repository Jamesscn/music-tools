use music_tools::audio::AudioPlayer;
use std::time::Duration;

fn main() {
    let player = AudioPlayer::new().unwrap();
    player.play_frequencies(vec![100.0], Duration::from_secs(1));
    player.play_frequencies(vec![200.0], Duration::from_secs(1));
    player.play_frequencies(vec![300.0], Duration::from_secs(1));
    player.play_frequencies(vec![400.0], Duration::from_secs(1));
    player.play_frequencies(vec![400.0, 440.0], Duration::from_secs(1));
    player.play_frequencies(vec![400.0, 440.0, 800.0], Duration::from_secs(1));
}
