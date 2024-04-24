use music_tools::audio::common::{SINE_WAVE, SQUARE_WAVE, TRIANGLE_WAVE};
use music_tools::audio::player::AudioPlayer;
use music_tools::audio::wavetable::WavetableOscillator;
use music_tools::common::Beat;
use music_tools::note::Note;

fn main() {
    let mut player = AudioPlayer::try_new().unwrap();
    player.set_tempo(160.0);
    let synths = [
        WavetableOscillator::new(SINE_WAVE, 1.0, 128),
        WavetableOscillator::new(TRIANGLE_WAVE, 1.0, 128),
        WavetableOscillator::new(|t| 2.0 * t - 1.0, 1.0, 128),
        WavetableOscillator::new(SQUARE_WAVE, 1.0, 128),
    ];
    for index in 0..32 {
        player.set_synth(synths[index % 4].clone());
        player.push(&Note::from_string("A4").unwrap(), &Beat::QUARTER);
    }
    player.play();
}
