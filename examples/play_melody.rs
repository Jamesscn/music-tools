use music_tools::audio::player::AudioPlayer;
use music_tools::common::Beat;
use music_tools::note::Note;

fn main() {
    let mut player = AudioPlayer::try_new().unwrap();
    player.set_tempo(160.0);
    let rhythm = Vec::from([
        Beat::QUARTER_DOTTED,
        Beat::QUARTER_DOTTED,
        Beat::QUARTER,
        Beat::QUARTER,
    ]);
    let notes = ["G4", "G4", "A#4", "C5", "G4", "G4", "F4", "F#4"]
        .iter()
        .map(|name| Note::from_string(name).unwrap())
        .collect::<Vec<Note>>();
    player.push_rhythm(&notes, rhythm, 16);
    player.play();
}
