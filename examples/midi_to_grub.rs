use music_tools::common::{AudioDuration, EqualTemperament};
use music_tools::midi::parser::MIDI;
use music_tools::midi::track::TrackItem;
use music_tools::{audio::common::Playable, midi::common::MIDIEvent};
use ordered_float::OrderedFloat;
use std::collections::HashSet;
use std::io::{self, Write};

fn main() {
    println!("Enter the path to the midi file:");
    print!("> ");
    io::stdout().flush().expect("Could not flush output!");
    let mut file_path: String = String::new();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Could not read input!");
    let tuning = EqualTemperament::new();
    let midi = MIDI::import(file_path.trim()).expect("Could not import MIDI file");
    let mut sequence: String = String::from("60000");
    let mut tempo = 120;
    let mut curr_frequency: f32 = 0f32;
    let mut active_notes: HashSet<OrderedFloat<f32>> = HashSet::new();
    for (_, track_item) in midi.iter_track_items() {
        match track_item {
            TrackItem::Event(event) => match event {
                MIDIEvent::NoteOn(note) => {
                    let frequency = note.get_frequencies(&tuning, 440f32)[0];
                    active_notes.insert(OrderedFloat(frequency));
                    curr_frequency = f32::max(curr_frequency, frequency);
                }
                MIDIEvent::NoteOff(note) => {
                    let frequency = note.get_frequencies(&tuning, 440f32)[0];
                    active_notes.remove(&OrderedFloat(frequency));
                    let mut highest_frequency = 0f32;
                    for active_frequency in active_notes.iter() {
                        highest_frequency = f32::max(highest_frequency, active_frequency.0);
                    }
                    curr_frequency = highest_frequency;
                }
                MIDIEvent::SetTempo(new_tempo) => tempo = new_tempo,
                _ => {}
            },
            TrackItem::Rest(beat) => {
                sequence.push_str(&format!(
                    " {} {}",
                    curr_frequency as usize,
                    beat.get_duration(tempo as f32).as_millis()
                ));
            }
        }
    }
    println!("{sequence}");
}
