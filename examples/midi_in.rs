use music_tools::audio::{Waveforms, WavetableOscillator};
use music_tools::midi::MIDI;
use std::io::{self, Write};

fn main() {
    //The midi file is opened and read
    println!("Enter the path to the midi file:");
    print!("> ");
    io::stdout().flush().expect("Could not flush output!");
    let mut file_path: String = String::new();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Could not read input!");
    let midi_object = MIDI::import_from_file(file_path.trim());

    //The oscillator that will play the midi file is created
    let mut oscillator = WavetableOscillator::new();

    //All notes of the midi file will be played as square waves
    let square_wave_channel = oscillator.add_channel(Waveforms::SQUARE_WAVE, 1.0);

    if midi_object.is_some() {
        let midi = midi_object.unwrap();
        let num_tracks = midi.get_num_tracks();
        if num_tracks == 0 {
            println!("The MIDI file has no tracks or notes in it!");
        } else {
            let mut channels: Vec<usize> = Vec::new();
            channels.resize(num_tracks, square_wave_channel);

            //The midi file is played if it is valid
            oscillator.play_midi(channels, midi);
        }
    }
}
