use music_tools::{audio::player::AudioPlayer, pitchclass::TwelveTone};
use std::time::Duration;

fn main() {
    let mut player = AudioPlayer::<TwelveTone>::try_new().unwrap();
    player.push(&100f32, &Duration::from_secs(1));
    player.push(&200f32, &Duration::from_secs(1));
    player.push(&300f32, &Duration::from_secs(1));
    player.push(&400f32, &Duration::from_secs(1));
    player.push(&[400f32, 440f32].as_slice(), &Duration::from_secs(1));
    player.push(
        &[400f32, 440f32, 800f32].as_slice(),
        &Duration::from_secs(1),
    );
    player.play();
}
