pub use crate::pitchclass::{PitchClass, PitchClasses, get_pitch_class_at_increment, get_letter_at_increment};
pub use crate::chord::Chord;
pub use crate::common::{ScaleType, PitchQuality};

pub struct Scale {
    pitch_classes: Vec<&'static PitchClass>,
    scale: ScaleType,
    pentatonic: PitchQuality
}

impl Scale {
    pub fn get_consecutive_pitch_class_names(&self) -> Option<Vec<&'static str>> {
        for tonic_name in self.pitch_classes[0].get_all_names() {
            let mut names: Vec<&'static str> = Vec::new();
            let mut current_name = tonic_name;
            names.push(current_name);
            for pitch_class in &self.pitch_classes[1..] {
                for pitch_class_name in pitch_class.get_all_names() {
                    if pitch_class_name.as_bytes()[0] as char == get_letter_at_increment(current_name.as_bytes()[0] as char, 1).unwrap() {
                        names.push(pitch_class_name);
                        current_name = pitch_class_name;
                        break;
                    }
                }
            }
            if names.len() == self.pitch_classes.len() {
                return Some(names);
            }
        }
        return None;
    }

    pub fn get_pitch_classes(&self) -> &Vec<&'static PitchClass> {
        return &self.pitch_classes;
    }

    pub fn get_scale(&self) -> ScaleType {
        return self.scale;
    }

    pub fn get_pentatonic(&self) -> PitchQuality {
        return self.pentatonic;
    }

    pub fn is_diatonic(&self) -> bool {
        if self.pitch_classes.len() == 8 {
            return true;
        }
        return false;
    }

    pub fn get_tonic(&self) -> &'static PitchClass {
        return self.pitch_classes[0];
    }

    pub fn get_diatonic_chords(&self, with_seventh: bool) -> Option<Vec<Chord>> {
        let minor_numerals: [&str; 7];
        let ionian_numerals: [&str; 7];
        let dorian_numerals: [&str; 7];
        let phrygian_numerals: [&str; 7];
        let lydian_numerals: [&str; 7];
        let mixolydian_numerals: [&str; 7];
        let aeolian_numerals: [&str; 7];
        let locrian_numerals: [&str; 7];
        if with_seventh {
            minor_numerals = ["i7", "ii°7", "bIIImaj7", "iv7", "Vmaj7", "bVImaj7", "bVII7"];
            ionian_numerals = ["Imaj7", "ii7", "iii7", "IVmaj7", "V7", "vi7", "vii°7"];
            dorian_numerals = ["i7", "ii7", "bIIImaj7", "IV7", "v7", "vi°7", "bVIImaj7"];
            phrygian_numerals = ["i7", "bIImaj7", "bIII7", "iv7", "v°7", "bVImaj7", "bvii7"];
            lydian_numerals = ["Imaj7", "II7", "iii7", "#iv°7", "Vmaj7", "vi7", "vii7"];
            mixolydian_numerals = ["I7", "ii7", "iii°7", "IVmaj7", "v7", "vi7", "bVIImaj7"];
            aeolian_numerals = ["i7", "ii°7", "bIIImaj7", "iv7", "v7", "bVImaj7", "bVII7"];
            locrian_numerals = ["i°7", "bIImaj7", "biii7", "iv7", "bVmaj7", "bVI7", "bvii7"];
        } else {
            minor_numerals = ["i", "ii°", "bIII", "iv", "V", "bVI", "bVII"];
            ionian_numerals = ["I", "ii", "iii", "IV", "V", "vi", "vii°"];
            dorian_numerals = ["i", "ii", "bIII", "IV", "v", "vi°", "bVII"];
            phrygian_numerals = ["i", "bII", "bIII", "iv", "v°", "bVI", "bvii"];
            lydian_numerals = ["I", "II", "iii", "#iv°", "V", "vi", "vii"];
            mixolydian_numerals = ["I", "ii", "iii°", "IV", "v", "vi", "bVII"];
            aeolian_numerals = ["i", "ii°", "bIII", "iv", "v", "bVI", "bVII"];
            locrian_numerals = ["i°", "bII", "biii", "iv", "bV", "bVI", "bvii"];
        }
        let chords: Vec<Chord> = match self.scale {
            ScaleType::Minor => minor_numerals,
            ScaleType::Major | ScaleType::Ionian => ionian_numerals,
            ScaleType::Dorian => dorian_numerals,
            ScaleType::Phrygian => phrygian_numerals,
            ScaleType::Lydian => lydian_numerals,
            ScaleType::Mixolydian => mixolydian_numerals,
            ScaleType::Aeolian | ScaleType::NaturalMinor => aeolian_numerals,
            ScaleType::Locrian => locrian_numerals,
            _ => return None
        }.iter().map(|x| Chord::from_numeral(self.get_tonic(), x).unwrap()).collect();
        return Some(chords);
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

pub fn get_scale(tonic: &'static PitchClass, scale: ScaleType, pentatonic: PitchQuality) -> Option<Scale> {
    let scale_steps: Vec<i8> = match scale {
        ScaleType::Major | ScaleType::Ionian => vec![2, 2, 1, 2, 2, 2, 1],
        ScaleType::Minor | ScaleType::Aeolian | ScaleType::NaturalMinor | ScaleType::DescendingMelodicMinor => vec![2, 1, 2, 2, 1, 2, 2],
        ScaleType::Dorian => vec![2, 1, 2, 2, 2, 1, 2],
        ScaleType::Phrygian => vec![1, 2, 2, 2, 1, 2, 2],
        ScaleType::Lydian => vec![2, 2, 2, 1, 2, 2, 1],
        ScaleType::Mixolydian => vec![2, 2, 1, 2, 2, 1, 2],
        ScaleType::Locrian => vec![1, 2, 2, 1, 2, 2, 2],
        ScaleType::HarmonicMinor => vec![2, 1, 2, 2, 1, 3, 1],
        ScaleType::AscendingMelodicMinor => vec![2, 1, 2, 2, 2, 2, 1],
        ScaleType::PhrygianDominant => vec![1, 3, 1, 2, 1, 2, 2],
        ScaleType::Whole => vec![2, 2, 2, 2, 2, 2],
        ScaleType::Chromatic => vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
    };
    let mut pitch_classes: Vec<&'static PitchClass> = Vec::new();
    pitch_classes.push(tonic);
    let mut current_pitch_class = tonic;
    for step in scale_steps {
        let next_pitch_class = get_pitch_class_at_increment(current_pitch_class, step);
        pitch_classes.push(next_pitch_class);
        current_pitch_class = next_pitch_class;
    }
    let diatonic_scale_type = is_scale_type_diatonic(scale);
    if pentatonic != PitchQuality::None && !diatonic_scale_type {
        return None;
    }
    if pentatonic == PitchQuality::Major {
        pitch_classes.remove(6);
        pitch_classes.remove(3);
    } else if pentatonic == PitchQuality::Minor {
        pitch_classes.remove(5);
        pitch_classes.remove(1);
    }
    return Some(Scale {
        pitch_classes,
        scale,
        pentatonic
    });
}