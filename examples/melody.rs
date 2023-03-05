use std::time::Duration;
use rodio::{OutputStream, Sink};
use musictools::audio::{WavetableOscillator, Waveforms};
use musictools::common::Fraction;
use musictools::note::Note;
use musictools::rhythm::{Rhythm, Beat};

fn main() {
    let mut rhythm = Rhythm::from(160.0, Fraction::new(5, 4), Vec::from(
        [Beat::QUARTER_DOTTED, Beat::QUARTER_DOTTED, Beat::QUARTER, Beat::QUARTER]
    ));
    let note_names = ["G4", "G4", "A#4", "C5", "G4", "G4", "F4", "F#4"];
    for index in 0..16 {
        let note = Note::from_string(note_names[index % note_names.len()]).unwrap();
        let mut oscillator = WavetableOscillator::new(128, 44100);
        oscillator.set_wave_function(Waveforms::SQUARE_WAVE, 1.0);
        oscillator.add_frequency(note.get_frequency());
        let stream_result = OutputStream::try_default();
        if stream_result.is_err() {
            println!("No sound card detected!");
            return;
        }
        let (_stream, stream_handle) = stream_result.unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(oscillator);
        let millisecond_duration = rhythm.get_duration_of_current_beat() * 1000.0;
        std::thread::sleep(Duration::from_millis(millisecond_duration as u64));
        rhythm.next_position();
    }
}