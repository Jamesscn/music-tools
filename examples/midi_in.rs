use music_tools::audio::common::Waveforms;
use music_tools::audio::player::AudioPlayer;
use music_tools::audio::wavetable::WavetableOscillator;
use music_tools::midi::parser::MIDI;
use std::io::{self, Write};

fn main() {
    //The midi file is opened and read
    println!("Enter the path to the MIDI file:");
    print!("> ");
    io::stdout().flush().expect("Could not flush output!");
    let mut file_path: String = String::new();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Could not read input!");
    let mut midi = MIDI::import(file_path.trim()).expect("Could not import MIDI file");
    let synth = WavetableOscillator::new(Waveforms::SAWTOOTH_WAVE, 1.0, 128);
    let mut player = AudioPlayer::try_new().unwrap();

    println!("Would you like to set a custom tempo (leave blank if not):");
    print!("> ");
    io::stdout().flush().expect("Could not flush output!");
    let mut new_tempo_string: String = String::new();
    io::stdin()
        .read_line(&mut new_tempo_string)
        .expect("Could not read input!");
    let new_tempo = new_tempo_string.trim().parse::<f32>().ok();
    if let Some(tempo) = new_tempo {
        midi.set_custom_tempo(new_tempo);
        println!("Overriding MIDI tempo to {tempo} BPM...");
    }

    println!("Would you like to set a custom base frequency (leave blank for 440 Hz):");
    print!("> ");
    io::stdout().flush().expect("Could not flush output!");
    let mut new_frequency_string: String = String::new();
    io::stdin()
        .read_line(&mut new_frequency_string)
        .expect("Could not read input!");
    let new_frequency = new_frequency_string.trim().parse::<f32>().ok();
    if let Some(frequency) = new_frequency {
        midi.set_custom_base_frequency(new_frequency);
        println!("Overriding MIDI base frequency to {frequency} Hz...");
    }

    player.push_midi(&midi, synth).expect("could not play midi");
    player.play();
}
