pub use crate::pitchclass::{PitchClass, PITCH_CLASSES, get_pitch_class_at_increment, get_letter_at_increment};

pub struct Scale {
    pub pitch_classes: Vec<&'static PitchClass>,
    pub scale: &'static str,
    pub pentatonic: &'static str
}

impl Scale {
    pub fn get_consecutive_pitch_class_names(&self) -> Vec<&'static str> {
        for tonic_names in self.pitch_classes[0].names {
            let mut names: Vec<&'static str> = Vec::new();
            let mut current_name = tonic_names.0;
            names.push(current_name);
            for pitch_class in &self.pitch_classes[1..] {
                for pitch_class_names in pitch_class.names {
                    let pitch_class_name = pitch_class_names.0;
                    if pitch_class_name.as_bytes()[0] as char == get_letter_at_increment(current_name.as_bytes()[0] as char, 1) {
                        names.push(pitch_class_name);
                        current_name = pitch_class_name;
                        break;
                    }
                }
            }
            if names.len() == self.pitch_classes.len() {
                return names;
            }
        }
        panic!("Could not find a consecutive list of pitch class names for the current scale.")
    }
}

pub fn get_scale(tonic: &'static PitchClass, scale: &'static str, pentatonic: &'static str) -> Scale {
    let mut pitch_classes: Vec<&'static PitchClass> = Vec::new();
    let scale_steps: Vec<i8> = match scale {
        "major" | "ionian" => Vec::from([2, 2, 1, 2, 2, 2, 1]),
        "minor" | "aeolian" | "natural minor" | "descending melodic minor" => Vec::from([2, 1, 2, 2, 1, 2, 2]),
        "dorian" => Vec::from([2, 1, 2, 2, 2, 1, 2]),
        "phrygian" => Vec::from([1, 2, 2, 2, 1, 2, 2]),
        "lydian" => Vec::from([2, 2, 2, 1, 2, 2, 1]),
        "mixolydian" => Vec::from([2, 2, 1, 2, 2, 1, 2]),
        "locrian" => Vec::from([1, 2, 2, 1, 2, 2, 2]),
        "harmonic minor" => Vec::from([2, 1, 2, 2, 1, 3, 1]),
        "ascending melodic minor" => Vec::from([2, 1, 2, 2, 2, 2, 1]),
        "phrygian dominant" => Vec::from([1, 3, 1, 2, 1, 2, 2]),
        "whole" => Vec::from([2, 2, 2, 2, 2, 2]),
        "chromatic" => Vec::from([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
        _ => panic!("Scale quality/mode {0} is not valid.", scale)
    };
    pitch_classes.push(tonic);
    let mut current_pitch_class = tonic;
    for step in scale_steps {
        let next_pitch_class = get_pitch_class_at_increment(current_pitch_class, step);
        pitch_classes.push(next_pitch_class);
        current_pitch_class = next_pitch_class;
    }
    if pentatonic == "major" {
        pitch_classes.remove(6);
        pitch_classes.remove(3);
    } else if pentatonic == "minor" {
        pitch_classes.remove(5);
        pitch_classes.remove(1);
    }
    return Scale {
        pitch_classes: pitch_classes,
        scale: scale,
        pentatonic: pentatonic
    };
}