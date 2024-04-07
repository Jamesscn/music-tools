use crate::common::InputError;
use regex::Regex;
use std::fmt;
use std::hash::Hash;

pub trait PitchClass: fmt::Display + Clone {
    fn get_num_classes() -> usize;
    /// The value of the class used for the base frequency (e.g. twelve tone's base frequency is A4,
    /// so its base frequency class would be A).
    fn base_frequency_class_value() -> usize;
    fn get_value(&self) -> usize;
    fn offset(&self, offset: isize) -> Self
    where
        Self: Sized;
    fn next(&self) -> Self
    where
        Self: Sized,
    {
        self.offset(1)
    }
    fn prev(&self) -> Self
    where
        Self: Sized,
    {
        self.offset(-1)
    }
}

/// Represents the twelve tone system of notes with pitch classes from A to G including sharps,
/// flats, double sharps and double flats.
#[derive(Copy, Clone, Debug)]
pub struct TwelveTone {
    name: &'static str,
}

impl TwelveTone {
    pub fn from_value(value: impl Into<usize>) -> Result<Self, InputError> {
        Ok(*TONES.get(value.into()).ok_or(InputError {
            message: String::from("the value provided must be an integer between 0 and 11"),
        })?)
    }

    pub fn from_string(string: &str) -> Result<Self, InputError> {
        let regex = Regex::new(r"^([A-Ga-g])(♮|x|X|b{1,2}|♭{1,2}|\#{1,2}|♯{1,2})?$").unwrap();
        if !regex.is_match(string) {
            return Err(InputError {
                message: String::from("string does not conform to expected pitch class format"),
            });
        }
        let regex_capture_groups = regex.captures(string).unwrap();
        let pitch_class_letter = regex_capture_groups.get(1).map_or("", |x| x.as_str());
        let accidental = regex_capture_groups.get(2).map_or("", |x| x.as_str());
        Ok(match accidental {
            "" | "♮" => match pitch_class_letter {
                "A" | "a" => A,
                "B" | "b" => B,
                "C" | "c" => C,
                "D" | "d" => D,
                "E" | "e" => E,
                "F" | "f" => F,
                "G" | "g" => G,
                _ => unreachable!(),
            },
            "b" | "♭" => match pitch_class_letter {
                "A" | "a" => A_FLAT,
                "B" | "b" => B_FLAT,
                "C" | "c" => C_FLAT,
                "D" | "d" => D_FLAT,
                "E" | "e" => E_FLAT,
                "F" | "f" => F_FLAT,
                "G" | "g" => G_FLAT,
                _ => unreachable!(),
            },
            "bb" | "♭♭" | "b♭" | "♭b" => match pitch_class_letter {
                "A" | "a" => A_DOUBLE_FLAT,
                "B" | "b" => B_DOUBLE_FLAT,
                "C" | "c" => C_DOUBLE_FLAT,
                "D" | "d" => D_DOUBLE_FLAT,
                "E" | "e" => E_DOUBLE_FLAT,
                "F" | "f" => F_DOUBLE_FLAT,
                "G" | "g" => G_DOUBLE_FLAT,
                _ => unreachable!(),
            },
            "#" | "♯" => match pitch_class_letter {
                "A" | "a" => A_SHARP,
                "B" | "b" => B_SHARP,
                "C" | "c" => C_SHARP,
                "D" | "d" => D_SHARP,
                "E" | "e" => E_SHARP,
                "F" | "f" => F_SHARP,
                "G" | "g" => G_SHARP,
                _ => unreachable!(),
            },
            "##" | "♯♯" | "#♯" | "♯#" | "x" | "X" => match pitch_class_letter {
                "A" | "a" => A_DOUBLE_SHARP,
                "B" | "b" => B_DOUBLE_SHARP,
                "C" | "c" => C_DOUBLE_SHARP,
                "D" | "d" => D_DOUBLE_SHARP,
                "E" | "e" => E_DOUBLE_SHARP,
                "F" | "f" => F_DOUBLE_SHARP,
                "G" | "g" => G_DOUBLE_SHARP,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        })
    }

    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn get_letter(&self) -> char {
        self.get_name().chars().next().unwrap()
    }

    pub fn is_sharp(&self) -> bool {
        let mut char_iter = self.get_name().chars();
        char_iter.next();
        let has_symbol = match char_iter.next() {
            Some(accidental) => accidental == '♯',
            None => false,
        };
        has_symbol && char_iter.next().is_none()
    }

    pub fn is_flat(&self) -> bool {
        let mut char_iter = self.get_name().chars();
        char_iter.next();
        let has_symbol = match char_iter.next() {
            Some(accidental) => accidental == '♭',
            None => false,
        };
        has_symbol && char_iter.next().is_none()
    }

    pub fn is_double_sharp(&self) -> bool {
        let mut char_iter = self.get_name().chars();
        char_iter.next();
        let has_first_symbol = match char_iter.next() {
            Some(accidental) => accidental == '♯',
            None => false,
        };
        let has_second_symbol = match char_iter.next() {
            Some(accidental) => accidental == '♯',
            None => false,
        };
        has_first_symbol && has_second_symbol
    }

    pub fn is_double_flat(&self) -> bool {
        let mut char_iter = self.get_name().chars();
        char_iter.next();
        let has_first_symbol = match char_iter.next() {
            Some(accidental) => accidental == '♭',
            None => false,
        };
        let has_second_symbol = match char_iter.next() {
            Some(accidental) => accidental == '♭',
            None => false,
        };
        has_first_symbol && has_second_symbol
    }

    pub fn swap_accidental(&self) -> Option<Self> {
        let swapped = match self.get_name() {
            "A♭" => G_SHARP,
            "A♯" => B_FLAT,
            "B♭" => A_SHARP,
            "B♯" => C_FLAT,
            "C♭" => B_SHARP,
            "C♯" => D_FLAT,
            "D♭" => C_SHARP,
            "D♯" => E_FLAT,
            "E♭" => D_SHARP,
            "E♯" => F_FLAT,
            "F♭" => E_SHARP,
            "F♯" => G_FLAT,
            "G♭" => F_SHARP,
            "G♯" => A_FLAT,
            _ => return None,
        };
        Some(swapped)
    }
}

pub const A_DOUBLE_FLAT: TwelveTone = TwelveTone { name: "A♭♭" };
pub const A_FLAT: TwelveTone = TwelveTone { name: "A♭" };
pub const A: TwelveTone = TwelveTone { name: "A" };
pub const A_SHARP: TwelveTone = TwelveTone { name: "A♯" };
pub const A_DOUBLE_SHARP: TwelveTone = TwelveTone { name: "A♯♯" };
pub const B_DOUBLE_FLAT: TwelveTone = TwelveTone { name: "B♭♭" };
pub const B_FLAT: TwelveTone = TwelveTone { name: "B♭" };
pub const B: TwelveTone = TwelveTone { name: "B" };
pub const B_SHARP: TwelveTone = TwelveTone { name: "B♯" };
pub const B_DOUBLE_SHARP: TwelveTone = TwelveTone { name: "B♯♯" };
pub const C_DOUBLE_FLAT: TwelveTone = TwelveTone { name: "C♭♭" };
pub const C_FLAT: TwelveTone = TwelveTone { name: "C♭" };
pub const C: TwelveTone = TwelveTone { name: "C" };
pub const C_SHARP: TwelveTone = TwelveTone { name: "C♯" };
pub const C_DOUBLE_SHARP: TwelveTone = TwelveTone { name: "C♯♯" };
pub const D_DOUBLE_FLAT: TwelveTone = TwelveTone { name: "D♭♭" };
pub const D_FLAT: TwelveTone = TwelveTone { name: "D♭" };
pub const D: TwelveTone = TwelveTone { name: "D" };
pub const D_SHARP: TwelveTone = TwelveTone { name: "D♯" };
pub const D_DOUBLE_SHARP: TwelveTone = TwelveTone { name: "D♯♯" };
pub const E_DOUBLE_FLAT: TwelveTone = TwelveTone { name: "E♭♭" };
pub const E_FLAT: TwelveTone = TwelveTone { name: "E♭" };
pub const E: TwelveTone = TwelveTone { name: "E" };
pub const E_SHARP: TwelveTone = TwelveTone { name: "E♯" };
pub const E_DOUBLE_SHARP: TwelveTone = TwelveTone { name: "E♯♯" };
pub const F_DOUBLE_FLAT: TwelveTone = TwelveTone { name: "F♭♭" };
pub const F_FLAT: TwelveTone = TwelveTone { name: "F♭" };
pub const F: TwelveTone = TwelveTone { name: "F" };
pub const F_SHARP: TwelveTone = TwelveTone { name: "F♯" };
pub const F_DOUBLE_SHARP: TwelveTone = TwelveTone { name: "F♯♯" };
pub const G_DOUBLE_FLAT: TwelveTone = TwelveTone { name: "G♭♭" };
pub const G_FLAT: TwelveTone = TwelveTone { name: "G♭" };
pub const G: TwelveTone = TwelveTone { name: "G" };
pub const G_SHARP: TwelveTone = TwelveTone { name: "G♯" };
pub const G_DOUBLE_SHARP: TwelveTone = TwelveTone { name: "G♯♯" };

const TONES: [TwelveTone; 12] = [
    C, C_SHARP, D, D_SHARP, E, F, F_SHARP, G, G_SHARP, A, A_SHARP, B,
];

impl PitchClass for TwelveTone {
    fn get_value(&self) -> usize {
        match self.name {
            "C♭♭" => 10,
            "C♭" => 11,
            "C" => 0,
            "C♯" => 1,
            "C♯♯" => 2,
            "D♭♭" => 0,
            "D♭" => 1,
            "D" => 2,
            "D♯" => 3,
            "D♯♯" => 4,
            "E♭♭" => 2,
            "E♭" => 3,
            "E" => 4,
            "E♯" => 5,
            "E♯♯" => 6,
            "F♭♭" => 3,
            "F♭" => 4,
            "F" => 5,
            "F♯" => 6,
            "F♯♯" => 7,
            "G♭♭" => 5,
            "G♭" => 6,
            "G" => 7,
            "G♯" => 8,
            "G♯♯" => 9,
            "A♭♭" => 7,
            "A♭" => 8,
            "A" => 9,
            "A♯" => 10,
            "A♯♯" => 11,
            "B♭♭" => 9,
            "B♭" => 10,
            "B" => 11,
            "B♯" => 0,
            "B♯♯" => 1,
            _ => unimplemented!(),
        }
    }

    fn get_num_classes() -> usize {
        12 // There are twelve pitch classes in the twelve tone system
    }

    fn base_frequency_class_value() -> usize {
        A.get_value() // The base frequency is A4, so we return the value for A
    }

    fn offset(&self, offset: isize) -> Self {
        TONES[(self.get_value() as isize + offset).rem_euclid(12) as usize]
    }
}

impl Default for TwelveTone {
    fn default() -> Self {
        C
    }
}

impl PartialEq for TwelveTone {
    fn eq(&self, other: &Self) -> bool {
        self.get_value() == other.get_value()
    }
}

impl Eq for TwelveTone {}

impl Hash for TwelveTone {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_value().hash(state);
    }
}

impl fmt::Display for TwelveTone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl TryFrom<&str> for TwelveTone {
    type Error = InputError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_string(value)
    }
}

impl TryFrom<String> for TwelveTone {
    type Error = InputError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_string(&value)
    }
}

impl TryFrom<usize> for TwelveTone {
    type Error = InputError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::from_value(value)
    }
}

impl From<TwelveTone> for String {
    fn from(value: TwelveTone) -> Self {
        value.to_string()
    }
}

impl From<TwelveTone> for usize {
    fn from(value: TwelveTone) -> Self {
        value.get_value()
    }
}
