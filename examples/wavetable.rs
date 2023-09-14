use music_tools::audio::{AudioPlayer, Waveforms, WavetableOscillator};
use music_tools::common::Beat;
use music_tools::note::Note;

fn main() {
    let mut oscillator = WavetableOscillator::empty();
    let sine_table = oscillator.add_wavetable_from_function(Waveforms::SINE_WAVE, 1.0, 128);
    let triangle_table = oscillator.add_wavetable_from_function(Waveforms::TRIANGLE_WAVE, 1.0, 128);
    let saw_table = oscillator.add_wavetable_from_function(Waveforms::SAWTOOTH_WAVE, 1.0, 128);
    let square_table = oscillator.add_wavetable_from_function(Waveforms::SQUARE_WAVE, 1.0, 128);
    let mut player = AudioPlayer::new_from_wavetable(oscillator).unwrap();
    player.set_tempo(160.0);
    for index in 0..32 {
        match index % 4 {
            0 => player.set_wavetable_index(sine_table),
            1 => player.set_wavetable_index(triangle_table),
            2 => player.set_wavetable_index(saw_table),
            _ => player.set_wavetable_index(square_table),
        }
        player.play(Note::from_string("A4").unwrap(), Beat::QUARTER);
    }
}
