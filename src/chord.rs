pub use regex::Regex;
pub use crate::common::{ScaleType, TriadQuality, PitchQuality};
pub use crate::pitchclass::PitchClass;
pub use crate::scale::get_pitch_class_at_increment;
pub use crate::scale::{Scale, get_scale};

pub struct Chord {
    pitch_classes: Vec<&'static PitchClass>,
    inversion: u8
}

impl Chord {
    pub fn empty() -> Chord {
        return Chord {
            pitch_classes: Vec::new(),
            inversion: 0
        }
    }

    pub fn from_triad(tonic: &'static PitchClass, triad_quality: TriadQuality) -> Chord {
        let major_scale_obj = get_scale(tonic, ScaleType::Major, PitchQuality::None).unwrap();
        let minor_scale_obj = get_scale(tonic, ScaleType::Minor, PitchQuality::None).unwrap();
        let whole_scale_obj = get_scale(tonic, ScaleType::Whole, PitchQuality::None).unwrap();
        let locrian_scale_obj = get_scale(tonic, ScaleType::Locrian, PitchQuality::None).unwrap();
        let major_scale = major_scale_obj.get_pitch_classes();
        let minor_scale = minor_scale_obj.get_pitch_classes();
        let whole_scale = whole_scale_obj.get_pitch_classes();
        let locrian_scale = locrian_scale_obj.get_pitch_classes();
        let mut pitch_classes: Vec<&'static PitchClass> = match triad_quality {
            TriadQuality::Major => vec![major_scale[0], major_scale[2], major_scale[4]],
            TriadQuality::Minor => vec![minor_scale[0], minor_scale[2], minor_scale[4]],
            TriadQuality::Sus2 => vec![major_scale[0], major_scale[1], major_scale[4]],
            TriadQuality::Sus4 => vec![major_scale[0], major_scale[3], major_scale[4]],
            TriadQuality::Augmented => vec![whole_scale[0], whole_scale[2], whole_scale[4]],
            TriadQuality::Diminished => vec![locrian_scale[0], locrian_scale[2], locrian_scale[4]]
        };
        return Chord {
            pitch_classes,
            inversion: 0
        }
    }

    pub fn from_numeral(tonic: &'static PitchClass, input_numeral: &str) -> Option<Chord> {
        let numeral_array = ["I", "II", "III", "IV", "V", "VI", "VII"];
        let numeral_regex = Regex::new(r"^(b|\#)?(I|II|III|IV|V|VI|VII|i|ii|iii|iv|v|vi|vii)(°|\+)?(maj7|7)?$").unwrap();
        if !numeral_regex.is_match(&input_numeral) {
            return None;
        }
        let regex_capture_groups = numeral_regex.captures(&input_numeral).unwrap();
        let accidental = regex_capture_groups.get(1).map_or("", |m| m.as_str());
        let numeral = regex_capture_groups.get(2).map_or("", |m| m.as_str());
        let quality = regex_capture_groups.get(3).map_or("", |m| m.as_str());
        let seventh = regex_capture_groups.get(4).map_or("", |m| m.as_str());
        let numeral_value = numeral_array.iter().position(|&x| x == numeral.to_ascii_uppercase()).unwrap();
        let triad_quality: TriadQuality;
        let chord_seventh: PitchQuality;
        if numeral.chars().all(char::is_uppercase) {
            if quality == "+" {
                triad_quality = TriadQuality::Augmented;
            } else if quality == "°" {
                return None;
            } else {
                triad_quality = TriadQuality::Major;
            }
        } else {
            if quality == "°" {
                triad_quality = TriadQuality::Diminished;
            } else if quality == "+" {
                return None;
            } else {
                triad_quality = TriadQuality::Minor;
            }
        }
        if seventh == "maj7" {
            chord_seventh = PitchQuality::Major;
        } else if seventh == "7" {
            chord_seventh = PitchQuality::Minor;
        } else {
            chord_seventh = PitchQuality::None;
        }
        let mut increment = match numeral_value {
            0 => 0,
            1 => 2,
            2 => 4,
            3 => 5,
            4 => 7,
            5 => 9,
            6 => 11,
            _ => return None
        };
        if accidental == "b" {
            increment = match numeral_value {
                1 => 1,
                2 => 3,
                4 => 6,
                5 => 8,
                6 => 10,
                _ => return None
            };
        } else if accidental == "#" {
            increment = match numeral_value {
                0 => 1,
                1 => 3,
                3 => 6,
                4 => 8,
                5 => 10,
                _ => return None
            };
        }
        let chord_tonic = get_pitch_class_at_increment(tonic, increment);
        let mut chord = Chord::from_triad(chord_tonic, triad_quality);
        chord.add_seventh(chord_seventh);
        return Some(chord);
    }

    pub fn add_seventh(&mut self, seventh: PitchQuality) {
        if seventh == PitchQuality::Major {
            let major_seventh = get_pitch_class_at_increment(self.get_tonic(), 11);
            self.pitch_classes.push(major_seventh);
        } else if seventh == PitchQuality::Minor {
            let minor_seventh = get_pitch_class_at_increment(self.get_tonic(), 10);
            self.pitch_classes.push(minor_seventh);
        }
    }

    pub fn add_ninth(&mut self, ninth: PitchQuality) {
        if ninth == PitchQuality::Major {
            let major_ninth = get_pitch_class_at_increment(self.get_tonic(), 14);
            self.pitch_classes.push(major_ninth);
        } else if ninth == PitchQuality::Minor {
            let minor_ninth = get_pitch_class_at_increment(self.get_tonic(), 13);
            self.pitch_classes.push(minor_ninth);
        }
    }

    pub fn add_thirteenth(&mut self, thirteenth: PitchQuality) {
        if thirteenth == PitchQuality::Major {
            let major_thirteenth = get_pitch_class_at_increment(self.get_tonic(), 21);
            self.pitch_classes.push(major_thirteenth);
        } else if thirteenth == PitchQuality::Minor {
            let minor_thirteenth = get_pitch_class_at_increment(self.get_tonic(), 20);
            self.pitch_classes.push(minor_thirteenth);
        }
    }

    pub fn add_pitch_class(&mut self, pitch_class: &'static PitchClass) {
        self.pitch_classes.push(pitch_class);
    }

    pub fn add_pitch_class_at_increment(&mut self, increment: i8) {
        let pitch_class = get_pitch_class_at_increment(self.get_tonic(), increment);
        self.pitch_classes.push(pitch_class);
    }

    pub fn set_inversion(&mut self, inversion: u8) {
        self.inversion = inversion % self.pitch_classes.len() as u8;
    }

    pub fn get_pitch_classes(&self) -> Vec<&'static PitchClass> {
        let mut pitch_classes = Vec::from(&self.pitch_classes[self.inversion as usize..]);
        let mut second_half = Vec::from(&self.pitch_classes[..self.inversion as usize]);
        pitch_classes.append(&mut second_half);
        return pitch_classes;
    }

    pub fn get_inversion(&self) -> u8 {
        return self.inversion;
    }

    pub fn get_tonic(&self) -> &'static PitchClass {
        return self.pitch_classes[self.inversion as usize];
    }
}