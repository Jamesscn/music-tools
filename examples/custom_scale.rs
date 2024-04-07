use music_tools::scale::Scale;

fn main() {
    let my_custom_scale = Scale::new(
        &[0_usize, 1_usize, 11_usize, 12_usize],
        &Vec::<&str>::new(),
        "My custom scale",
    );
    let notes = my_custom_scale.to_notes("A4".try_into().unwrap());
    println!("My custom scale in A4 has the following notes:");
    for note in notes {
        println!("{note}");
    }
}
