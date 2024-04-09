use music_tools::audio::common::SAWTOOTH_WAVE;
use music_tools::audio::player::AudioPlayer;
use music_tools::audio::wavetable::WavetableOscillator;
use music_tools::common::PythagoreanTuning;
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
    let midi = MIDI::import(file_path.trim()).expect("Could not import MIDI file");
    let synth = WavetableOscillator::new(SAWTOOTH_WAVE, 1.0, 128);
    let mut player = AudioPlayer::try_new().unwrap();

    println!("Would you like to set a playback speed? (default: 1):");
    print!("> ");
    io::stdout().flush().expect("Could not flush output!");
    let mut new_speed_string: String = String::new();
    io::stdin()
        .read_line(&mut new_speed_string)
        .expect("Could not read input!");
    let new_speed = new_speed_string.trim().parse::<f32>().ok();
    if let Some(new_speed) = new_speed {
        if new_speed >= 0f32 {
            player.set_speed(new_speed);
            println!("Setting playback speed to {new_speed}x...");
        } else {
            println!("Ignoring invalid playback speed...");
        }
    }

    println!("Would you like to set a custom base frequency? (default: 440):");
    print!("> ");
    io::stdout().flush().expect("Could not flush output!");
    let mut new_frequency_string: String = String::new();
    io::stdin()
        .read_line(&mut new_frequency_string)
        .expect("Could not read input!");
    let new_frequency = new_frequency_string.trim().parse::<f32>().ok();
    if let Some(new_frequency) = new_frequency {
        if new_frequency >= 0f32 {
            player.set_base_frequency(new_frequency);
            println!("Overriding base frequency to {new_frequency} Hz...");
        } else {
            println!("Ignoring invalid base frequency...");
        }
    }

    println!("Would you like to change the tuning scheme to Pythagorean tuning? (y/N):");
    print!("> ");
    io::stdout().flush().expect("Could not flush output!");
    let mut new_tuning_string: String = String::new();
    io::stdin()
        .read_line(&mut new_tuning_string)
        .expect("Could not read input!");
    let new_frequency = new_tuning_string.trim();
    if new_frequency.to_ascii_lowercase().starts_with('y') {
        player.set_tuning(PythagoreanTuning::new(12));
        println!("Using Pythagorean tuning...");
    }

    player
        .push_midi(&midi, &[synth])
        .expect("could not play midi");
    player.play();
}
