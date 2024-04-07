use music_tools::audio::player::AudioPlayer;
use music_tools::interval::Interval;
use std::time::Duration;

fn main() {
    let mut player = AudioPlayer::try_new().unwrap();

    println!("- BEFORE -");
    Interval::new(28_usize, "Major Seventeenth", "M17").expect("could not create custom interval");
    println!("{:#?}", Interval::from_semitones(27_usize));
    println!("{:#?}", Interval::from_semitones(28_usize));

    println!("- AFTER -");
    let interval1 = Interval::new(27_usize, "Minor Seventeenth", "m17")
        .expect("could not create custom interval");
    let interval2 = Interval::new(28_usize, "Major Seventeenth (NEW!)", "M17n")
        .expect("could not create custom interval");
    println!("{:#?}", Interval::from_semitones(27_usize));
    println!("{:#?}", Interval::from_semitones(28_usize));

    player.push(&interval1, &Duration::from_secs(3));
    player.push(&interval2, &Duration::from_secs(3));

    player.play();
}
