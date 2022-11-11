pub use regex::Regex;
pub use crate::common::{ScaleType, TriadQuality, Pentatonic, Seventh};
pub use crate::pitchclass::PitchClass;
pub use crate::scale::get_pitch_class_at_increment;
pub use crate::scale::{Scale, get_scale};

pub struct Chord {
    pitch_classes: Vec<&'static PitchClass>,
    inversion: u8,
    triad_quality: TriadQuality,
    seventh: Seventh
}

impl Chord {
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

    pub fn get_triad_quality(&self) -> TriadQuality {
        return self.triad_quality;
    }

    pub fn get_seventh(&self) -> Seventh {
        return self.seventh;
    }

    pub fn get_tonic(&self) -> &'static PitchClass {
        return self.pitch_classes[self.inversion as usize];
    }

    pub fn get_short_name(&self) -> String {
        let mut short_name: String = String::new();
        if self.inversion != 0 {
            short_name.push_str(self.pitch_classes[0].get_name());
            short_name.push_str("/");
        }
        short_name.push_str(self.get_tonic().get_name());
        let quality_name: &'static str = match self.triad_quality {
            TriadQuality::Minor => "m",
            TriadQuality::Augmented => "+",
            TriadQuality::Diminished => "°",
            _ => ""
        };
        short_name.push_str(quality_name);
        let seventh_name: &'static str;
        if self.seventh == Seventh::Major {
            if self.triad_quality == TriadQuality::Minor {
                seventh_name = "(maj7)";
            } else {
                seventh_name = "maj7";
            }
        } else if self.seventh == Seventh::Minor {
            seventh_name = "7"
        } else {
            seventh_name = ""
        }
        short_name.push_str(seventh_name);
        return short_name;
    }
}

pub fn get_chord_with_quality(tonic: &'static PitchClass, triad_quality: TriadQuality, seventh: Seventh, inversion: u8) -> Chord {
    let major_scale_obj = get_scale(tonic, ScaleType::Major, Pentatonic::None).unwrap();
    let minor_scale_obj = get_scale(tonic, ScaleType::Minor, Pentatonic::None).unwrap();
    let whole_scale_obj = get_scale(tonic, ScaleType::Whole, Pentatonic::None).unwrap();
    let locrian_scale_obj = get_scale(tonic, ScaleType::Locrian, Pentatonic::None).unwrap();
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
    if seventh == Seventh::Major {
        pitch_classes.push(major_scale[6]);
    } else if seventh == Seventh::Minor {
        pitch_classes.push(minor_scale[6]);
    }
    let num_pitch_classes = pitch_classes.len() as u8;
    return Chord {
        pitch_classes,
        inversion: inversion % num_pitch_classes,
        triad_quality,
        seventh
    }
}

pub fn get_chord_from_numeral(scale: &Scale, input_numeral: &str) -> Option<Chord> {
    if !scale.is_diatonic() {
        return None;
    }
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
    let chord_seventh: Seventh;
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
        chord_seventh = Seventh::Major;
    } else if seventh == "7" {
        chord_seventh = Seventh::Minor;
    } else {
        chord_seventh = Seventh::None;
    }
    let tonic_without_accidental: &PitchClass = scale.get_pitch_classes()[numeral_value];
    let chord_tonic: &PitchClass;
    if accidental == "b" {
        chord_tonic = get_pitch_class_at_increment(tonic_without_accidental, -1);
    } else if accidental == "#" {
        chord_tonic = get_pitch_class_at_increment(tonic_without_accidental, 1);
    } else {
        chord_tonic = tonic_without_accidental;
    }
    return Some(get_chord_with_quality(chord_tonic, triad_quality, chord_seventh, 0));
}