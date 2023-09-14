use music_tools::audio::AudioPlayer;
use std::time::Duration;

fn main() {
    let mut player = AudioPlayer::new().unwrap();
    player.play(100.0, Duration::from_secs(1));
    player.play(200.0, Duration::from_secs(1));
    player.play(300.0, Duration::from_secs(1));
    player.play(400.0, Duration::from_secs(1));
    player.play(vec![400.0, 440.0], Duration::from_secs(1));
    player.play(vec![400.0, 440.0, 800.0], Duration::from_secs(1));
}
