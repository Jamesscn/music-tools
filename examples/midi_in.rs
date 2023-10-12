use music_tools::audio::common::Waveforms;
use music_tools::audio::player::AudioPlayer;
use music_tools::audio::wavetable::WavetableOscillator;
use music_tools::midi::processor::MIDI;
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
    let midi = MIDI::import_from_file(file_path.trim()).expect("Could not import MIDI file");
    let num_tracks = midi.get_num_tracks();
    if num_tracks == 0 {
        println!("The MIDI file has no tracks or notes in it!");
    } else {
        let synth = WavetableOscillator::new(Waveforms::SAWTOOTH_WAVE, 1.0, 128);
        let mut player = AudioPlayer::try_new().unwrap();
        let tempo = midi.get_tracks()[0].get_tempo();
        println!("Tempo detected: {tempo} BPM");
        println!("Would you like to set a custom tempo (leave blank if not):");
        print!("> ");
        io::stdout().flush().expect("Could not flush output!");
        let mut new_tempo_string: String = String::new();
        io::stdin()
            .read_line(&mut new_tempo_string)
            .expect("Could not read input!");
        let new_tempo = new_tempo_string.trim().parse::<f32>().unwrap_or(tempo);
        println!("Playing MIDI at {new_tempo} BPM...");
        player.push_midi(&midi, synth, Some(new_tempo));
        player.play();
    }
}
