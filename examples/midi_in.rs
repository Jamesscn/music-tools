use music_tools::audio::AudioPlayer;
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
    let midi = MIDI::import_from_file(file_path.trim()).expect("Could not import MIDI file");
    let num_tracks = midi.get_num_tracks();
    if num_tracks == 0 {
        println!("The MIDI file has no tracks or notes in it!");
    } else {
        let mut player = AudioPlayer::new().unwrap();
        player.play_midi(midi);
    }
}
