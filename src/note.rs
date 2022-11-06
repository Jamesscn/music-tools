pub use regex::Regex;
pub use crate::pitchclass::{PitchClass, get_pitch_class_from_name};

pub struct Note {
    pub pitch_class: &'static PitchClass,
    pub octave: u8,
    pub value: u32,
    base_frequency: f32
}

impl Note {
    pub fn from_name(name: &'static str) -> Note {
        let regex = Regex::new(r"^(A|A\#|Bb|B|C|C\#|Db|D|D\#|Eb|E|F|F\#|Gb|G|G\#|Ab)(\d)$").unwrap();
        assert!(regex.is_match(&name));
        let regex_capture_groups = regex.captures(&name).unwrap();
        let octave: u8 = (&regex_capture_groups[2]).parse().unwrap();
        let pitch_class_string = String::from(&regex_capture_groups[1]);
        let pitch_class = get_pitch_class_from_name(pitch_class_string);
        return Note {
            pitch_class,
            octave,
            value: octave as u32 * 12 + pitch_class.get_value() as u32,
            base_frequency: 440.0
        };
    }

    pub fn from(pitch_class: &'static PitchClass, octave: u8) -> Note {
        return Note {
            pitch_class,
            octave,
            value: octave as u32 * 12 + pitch_class.get_value() as u32,
            base_frequency: 440.0
        }
    }

    pub fn set_base_frequency(&mut self, base_frequency: f32) {
        self.base_frequency = base_frequency;
    }

    pub fn get_base_frequency(&self) -> f32 {
        return self.base_frequency;
    }

    pub fn get_octave(&self) -> u8 {
        return self.octave;
    }

    pub fn get_index_on_keyboard(&self) -> u32 {
        return self.value;
    }

    pub fn get_frequency(&self) -> f32 {
        return self.base_frequency as f32 * (2.0 as f32).powf(self.octave as f32 + (self.pitch_class.get_value() as i8 - 9) as f32 / 12 as f32 - 4.0)
    }
}