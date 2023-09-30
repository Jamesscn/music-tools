use music_tools::audio::AudioPlayer;
use music_tools::common::Beat;
use music_tools::note::Note;
use std::str::FromStr;

fn main() {
    let mut player = AudioPlayer::new().unwrap();
    player.set_tempo(160.0);
    let beats = [
        Beat::QUARTER_DOTTED,
        Beat::QUARTER_DOTTED,
        Beat::QUARTER,
        Beat::QUARTER,
    ];
    let note_names = ["G4", "G4", "A#4", "C5", "G4", "G4", "F4", "F#4"];
    for index in 0..16 {
        let note = Note::from_str(note_names[index % note_names.len()]).unwrap();
        let duration = beats[index % beats.len()];
        player.play(note, duration);
    }
}
