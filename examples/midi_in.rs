use std::io::{self, Write};
use musictools::midi::MIDI;
use musictools::audio::{WavetableOscillator, Waveforms};

fn main() {
    println!("Enter the path to the midi file:");
    print!("> ");
    io::stdout().flush().expect("Could not flush output!");
    let mut file_path: String = String::new();
    io::stdin().read_line(&mut file_path).expect("Could not read input!");
    let mut oscillator = WavetableOscillator::new();
    let square_wave_channel = oscillator.add_channel(Waveforms::SQUARE_WAVE, 1.0);
    let midi_object = MIDI::import_from_file(file_path.trim());
    if midi_object.is_some() {
        let midi = midi_object.unwrap();
        let tracks = midi.get_tracks();
        let num_tracks = tracks.len();
        let mut current_track = 1;
        for track in tracks {
            println!("Playing track {} of {}...", current_track, num_tracks);
            oscillator.play_track(square_wave_channel, track);
            current_track += 1;
        }
        if num_tracks == 0 {
            println!("The MIDI file has no tracks or notes in it!");
        }
    }
}