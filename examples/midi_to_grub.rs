use music_tools::midi::parser::MIDI;
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
    let _midi = MIDI::import(file_path.trim()).expect("Could not import MIDI file");
    todo!();
}
