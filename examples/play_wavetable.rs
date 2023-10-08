use music_tools::audio::common::Waveforms;
use music_tools::audio::player::AudioPlayer;
use music_tools::audio::wavetable::WavetableOscillator;
use music_tools::common::Beat;
use music_tools::note::Note;

fn main() {
    let mut player = AudioPlayer::try_new().unwrap();
    player.set_tempo(160.0);
    for index in 0..32 {
        match index % 4 {
            0 => {
                let sine_table = WavetableOscillator::new(Waveforms::SINE_WAVE, 1.0, 128);
                player.set_synth(sine_table);
            }
            1 => {
                let triangle_table = WavetableOscillator::new(Waveforms::TRIANGLE_WAVE, 1.0, 128);
                player.set_synth(triangle_table);
            }
            2 => {
                let saw_table = WavetableOscillator::new(|t| 2.0 * t - 1.0, 1.0, 128);
                player.set_synth(saw_table);
            }
            _ => {
                let square_table = WavetableOscillator::new(Waveforms::SQUARE_WAVE, 1.0, 128);
                player.set_synth(square_table);
            }
        }
        player.push(&Note::from_string("A4").unwrap(), &Beat::QUARTER);
    }
    player.play();
}
