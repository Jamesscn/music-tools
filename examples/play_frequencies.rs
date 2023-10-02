use music_tools::audio::player::AudioPlayer;
use std::time::Duration;

fn main() {
    let mut player = AudioPlayer::new().unwrap();
    player.push(&100.0, &Duration::from_secs(1));
    player.push(&200.0, &Duration::from_secs(1));
    player.push(&300.0, &Duration::from_secs(1));
    player.push(&400.0, &Duration::from_secs(1));
    player.push(&vec![400.0, 440.0], &Duration::from_secs(1));
    player.push(&vec![400.0, 440.0, 800.0], &Duration::from_secs(1));
    player.play();
}
