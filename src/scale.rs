pub use crate::pitchclass::{PitchClass, PitchClasses, get_pitch_class_at_increment, get_letter_at_increment};
pub use crate::enums::{ScaleType, Pentatonic};

pub struct Scale {
    pitch_classes: Vec<&'static PitchClass>,
    scale: ScaleType,
    pentatonic: Pentatonic
}

impl Scale {
    pub fn get_consecutive_pitch_class_names(&self) -> Vec<&'static str> {
        for tonic_name in self.pitch_classes[0].get_all_names() {
            let mut names: Vec<&'static str> = Vec::new();
            let mut current_name = tonic_name;
            names.push(current_name);
            for pitch_class in &self.pitch_classes[1..] {
                for pitch_class_name in pitch_class.get_all_names() {
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

    pub fn get_pitch_classes(&self) -> &Vec<&'static PitchClass> {
        return &self.pitch_classes;
    }

    pub fn get_scale(&self) -> &ScaleType {
        return &self.scale;
    }

    pub fn get_pentatonic(&self) -> &Pentatonic {
        return &self.pentatonic;
    }

    pub fn is_diatonic(&self) -> bool {
        if self.pitch_classes.len() == 8 {
            return true;
        }
        return false;
    }
}

pub fn is_scale_type_diatonic(scale_type: ScaleType) -> bool {
    return match scale_type {
        ScaleType::Major | ScaleType::Ionian | ScaleType::Minor | ScaleType::Aeolian |
        ScaleType::NaturalMinor | ScaleType::DescendingMelodicMinor | ScaleType::Dorian |
        ScaleType::Phrygian | ScaleType::Lydian | ScaleType::Mixolydian | ScaleType::Locrian |
        ScaleType::HarmonicMinor | ScaleType::AscendingMelodicMinor |
        ScaleType::PhrygianDominant => true,
        _ => false
    }
}

pub fn get_scale(tonic: &'static PitchClass, scale_type: ScaleType, pentatonic: Pentatonic) -> Scale {
    let mut pitch_classes: Vec<&'static PitchClass> = Vec::new();
    let scale_steps: Vec<i8> = match scale_type {
        ScaleType::Major | ScaleType::Ionian => Vec::from([2, 2, 1, 2, 2, 2, 1]),
        ScaleType::Minor | ScaleType::Aeolian | ScaleType::NaturalMinor | ScaleType::DescendingMelodicMinor => Vec::from([2, 1, 2, 2, 1, 2, 2]),
        ScaleType::Dorian => Vec::from([2, 1, 2, 2, 2, 1, 2]),
        ScaleType::Phrygian => Vec::from([1, 2, 2, 2, 1, 2, 2]),
        ScaleType::Lydian => Vec::from([2, 2, 2, 1, 2, 2, 1]),
        ScaleType::Mixolydian => Vec::from([2, 2, 1, 2, 2, 1, 2]),
        ScaleType::Locrian => Vec::from([1, 2, 2, 1, 2, 2, 2]),
        ScaleType::HarmonicMinor => Vec::from([2, 1, 2, 2, 1, 3, 1]),
        ScaleType::AscendingMelodicMinor => Vec::from([2, 1, 2, 2, 2, 2, 1]),
        ScaleType::PhrygianDominant => Vec::from([1, 3, 1, 2, 1, 2, 2]),
        ScaleType::Whole => Vec::from([2, 2, 2, 2, 2, 2]),
        ScaleType::Chromatic => Vec::from([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1])
    };
    pitch_classes.push(tonic);
    let mut current_pitch_class = tonic;
    for step in scale_steps {
        let next_pitch_class = get_pitch_class_at_increment(current_pitch_class, step);
        pitch_classes.push(next_pitch_class);
        current_pitch_class = next_pitch_class;
    }
    let diatonic_scale_type = is_scale_type_diatonic(scale_type);
    if pentatonic != Pentatonic::None && !diatonic_scale_type {
        panic!("Cannot create a pentatonic scale out of a scale which is not diatonic.");    
    }
    if pentatonic == Pentatonic::Major {
        pitch_classes.remove(6);
        pitch_classes.remove(3);
    } else if pentatonic == Pentatonic::Minor {
        pitch_classes.remove(5);
        pitch_classes.remove(1);
    }
    return Scale {
        pitch_classes: pitch_classes,
        scale: scale_type,
        pentatonic: pentatonic
    };
}