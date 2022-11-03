pub use regex::Regex;
pub use crate::pitchclass::{PitchClass, get_pitch_class};

pub struct Note {
    pub pitch_class: &'static PitchClass,
    pub octave: u8,
    pub value: u32,
    pub frequency: f32
}

impl Note {
    pub fn new(name: &'static str) -> Note {
        let regex = Regex::new(r"^(A|A\#|Bb|B|C|C\#|Db|D|D\#|Eb|E|F|F\#|Gb|G|G\#|Ab)(\d)$").unwrap();
        assert!(regex.is_match(&name));
        let regex_capture_groups = regex.captures(&name).unwrap();
        let octave: u8 = (&regex_capture_groups[2]).parse().unwrap();
        let pitch_class_string = String::from(&regex_capture_groups[1]);
        let pitch_class = get_pitch_class(pitch_class_string);
        return Note {
            pitch_class: pitch_class,
            octave: octave,
            value: octave as u32 * 12 + pitch_class.value as u32,
            frequency: 27.5 as f32 * (2.0 as f32).powf(octave as f32 + (pitch_class.value as i8 - 9) as f32 / 12 as f32)
        };
    }
}