use musictools::pitchclass::PitchClass;

fn main() {
    let a = PitchClass::from_name(String::from("A"));
    let b_flat = PitchClass::from_name(String::from("Bb"));
    let cx = PitchClass::from_name(String::from("Cx"));
}