use regex::Regex;

use crate::chord::Chord;
use crate::common::{IncompleteChordError, InputError};
use std::fmt;
use std::hash::Hash;

/// A structure used to define one of the pitch classes of the twelve tone system.
#[derive(Copy, Clone, Debug, Default, Eq)]
pub enum PitchClass {
    /// The A♭♭ or Abb double flat pitch class.
    ADoubleFlat,
    /// The A♭ or Ab flat pitch class.
    AFlat,
    /// The A or A♮ natural pitch class.
    A,
    /// The A♯ or A# sharp pitch class.
    ASharp,
    /// The A♯♯ or A## or Ax double sharp pitch class.
    ADoubleSharp,
    /// The B♭♭ or Bbb double flat pitch class.
    BDoubleFlat,
    /// The B♭ or Bb flat pitch class.
    BFlat,
    /// The B or B♮ natural pitch class.
    B,
    /// The B♯ or B# sharp pitch class.
    BSharp,
    /// The B♯♯ or B## or Bx double sharp pitch class.
    BDoubleSharp,
    /// The C♭♭ or Cbb double flat pitch class.
    CDoubleFlat,
    /// The C♭ or Cb flat pitch class.
    CFlat,
    #[default]
    /// The C or C♮ natural pitch class.
    C,
    /// The C♯ or C# sharp pitch class.
    CSharp,
    /// The C♯♯ or C## or Cx double sharp pitch class.
    CDoubleSharp,
    /// The D♭♭ or Dbb double flat pitch class.
    DDoubleFlat,
    /// The D♭ or Db flat pitch class.
    DFlat,
    /// The D or D♮ natural pitch class.
    D,
    /// The D♯ or D# sharp pitch class.
    DSharp,
    /// The D♯♯ or D## or Dx double sharp pitch class.
    DDoubleSharp,
    /// The E♭♭ or Ebb double flat pitch class.
    EDoubleFlat,
    /// The E♭ or Eb flat pitch class.
    EFlat,
    /// The E or E♮ natural pitch class.
    E,
    /// The E♯ or E# sharp pitch class.
    ESharp,
    /// The E♯♯ or E## or Ex double sharp pitch class.
    EDoubleSharp,
    /// The F♭♭ or Fbb double flat pitch class.
    FDoubleFlat,
    /// The F♭ or Fb flat pitch class.
    FFlat,
    /// The F or F♮ natural pitch class.
    F,
    /// The F♯ or F# sharp pitch class.
    FSharp,
    /// The F♯♯ or F## or Fx double sharp pitch class.
    FDoubleSharp,
    /// The G♭♭ or Gbb double flat pitch class.
    GDoubleFlat,
    /// The G♭ or Gb flat pitch class.
    GFlat,
    /// The G or G♮ natural pitch class.
    G,
    /// The G♯ or G# sharp pitch class.
    GSharp,
    /// The G♯♯ or G## or Gx double sharp pitch class.
    GDoubleSharp,
}

impl PitchClass {
    /// Returns a [`Result`] which can contain any [`PitchClass`] given its value from 0 to 11,
    /// where 0 represents C, 1 represents C♯ and so on. If the index is greater than 11 then
    /// an [`InputError`] is returned.
    ///
    /// # Parameters
    ///
    /// - `value`: An integer from 0 to 11 representing the [`PitchClass`] to return.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use music_tools::pitchclass::PitchClass;
    ///
    /// let pitch_class = PitchClass::from_value(6).unwrap();
    /// assert_eq!(PitchClass::GFlat, pitch_class);
    /// ```
    pub fn from_value(value: impl Into<u8>) -> Result<Self, InputError> {
        match value.into() {
            0 => Ok(Self::C),
            1 => Ok(Self::CSharp),
            2 => Ok(Self::D),
            3 => Ok(Self::DSharp),
            4 => Ok(Self::E),
            5 => Ok(Self::F),
            6 => Ok(Self::FSharp),
            7 => Ok(Self::G),
            8 => Ok(Self::GSharp),
            9 => Ok(Self::A),
            10 => Ok(Self::ASharp),
            11 => Ok(Self::B),
            _ => Err(InputError {
                message: String::from("the value provided must be an integer between 0 and 11"),
            }),
        }
    }

    /// Returns a [`Result`] which can contain a [`PitchClass`] representing a pitch class given its
    /// name, or an [`InputError`] if the input string was invalid.
    ///
    /// # Parameters
    ///
    /// - `string`: A string representing the name of the pitch class to return. This string can
    ///   contain flats, sharps, double flats or double sharps.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use music_tools::pitchclass::PitchClass;
    ///
    /// let a = PitchClass::from_string("A");
    /// let b_flat = PitchClass::from_string("Bb");
    /// ```
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
                "A" | "a" => Self::A,
                "B" | "b" => Self::B,
                "C" | "c" => Self::C,
                "D" | "d" => Self::D,
                "E" | "e" => Self::E,
                "F" | "f" => Self::F,
                "G" | "g" => Self::G,
                _ => unreachable!(),
            },
            "b" | "♭" => match pitch_class_letter {
                "A" | "a" => Self::AFlat,
                "B" | "b" => Self::BFlat,
                "C" | "c" => Self::CFlat,
                "D" | "d" => Self::DFlat,
                "E" | "e" => Self::EFlat,
                "F" | "f" => Self::FFlat,
                "G" | "g" => Self::GFlat,
                _ => unreachable!(),
            },
            "bb" | "♭♭" => match pitch_class_letter {
                "A" | "a" => Self::ADoubleFlat,
                "B" | "b" => Self::BDoubleFlat,
                "C" | "c" => Self::CDoubleFlat,
                "D" | "d" => Self::DDoubleFlat,
                "E" | "e" => Self::EDoubleFlat,
                "F" | "f" => Self::FDoubleFlat,
                "G" | "g" => Self::GDoubleFlat,
                _ => unreachable!(),
            },
            "#" | "♯" => match pitch_class_letter {
                "A" | "a" => Self::ASharp,
                "B" | "b" => Self::BSharp,
                "C" | "c" => Self::CSharp,
                "D" | "d" => Self::DSharp,
                "E" | "e" => Self::ESharp,
                "F" | "f" => Self::FSharp,
                "G" | "g" => Self::GSharp,
                _ => unreachable!(),
            },
            "##" | "♯♯" | "x" | "X" => match pitch_class_letter {
                "A" | "a" => Self::ADoubleSharp,
                "B" | "b" => Self::BDoubleSharp,
                "C" | "c" => Self::CDoubleSharp,
                "D" | "d" => Self::DDoubleSharp,
                "E" | "e" => Self::EDoubleSharp,
                "F" | "f" => Self::FDoubleSharp,
                "G" | "g" => Self::GDoubleSharp,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        })
    }

    pub fn from_letter_and_value(letter: char, value: impl Into<u8>) -> Result<Self, InputError> {
        let value = value.into();
        let letter = letter.to_ascii_uppercase();
        let bad_letter_error = InputError {
            message: format!(
                "invalid letter {}, provided, must be between A and G",
                letter
            ),
        };
        let bad_value_provided = InputError {
            message: format!(
                "the letter {} does not have a note with value {}",
                letter, value
            ),
        };
        match letter {
            'A' => match value {
                7 => Ok(Self::ADoubleFlat),
                8 => Ok(Self::AFlat),
                9 => Ok(Self::A),
                10 => Ok(Self::ASharp),
                11 => Ok(Self::ADoubleSharp),
                _ => Err(bad_value_provided),
            },
            'B' => match value {
                9 => Ok(Self::BDoubleFlat),
                10 => Ok(Self::BFlat),
                11 => Ok(Self::B),
                0 => Ok(Self::BSharp),
                1 => Ok(Self::BDoubleSharp),
                _ => Err(bad_value_provided),
            },
            'C' => match value {
                10 => Ok(Self::CDoubleFlat),
                11 => Ok(Self::CFlat),
                0 => Ok(Self::C),
                1 => Ok(Self::CSharp),
                2 => Ok(Self::CDoubleSharp),
                _ => Err(bad_value_provided),
            },
            'D' => match value {
                0 => Ok(Self::DDoubleFlat),
                1 => Ok(Self::DFlat),
                2 => Ok(Self::D),
                3 => Ok(Self::DSharp),
                4 => Ok(Self::DDoubleSharp),
                _ => Err(bad_value_provided),
            },
            'E' => match value {
                2 => Ok(Self::EDoubleFlat),
                3 => Ok(Self::EFlat),
                4 => Ok(Self::E),
                5 => Ok(Self::ESharp),
                6 => Ok(Self::EDoubleSharp),
                _ => Err(bad_value_provided),
            },
            'F' => match value {
                3 => Ok(Self::FDoubleFlat),
                4 => Ok(Self::FFlat),
                5 => Ok(Self::F),
                6 => Ok(Self::FSharp),
                7 => Ok(Self::FDoubleSharp),
                _ => Err(bad_value_provided),
            },
            'G' => match value {
                5 => Ok(Self::GDoubleFlat),
                6 => Ok(Self::GFlat),
                7 => Ok(Self::G),
                8 => Ok(Self::GSharp),
                9 => Ok(Self::GDoubleSharp),
                _ => Err(bad_value_provided),
            },
            _ => Err(bad_letter_error),
        }
    }

    pub fn is_standard(&self) -> bool {
        matches!(
            self,
            Self::AFlat
                | Self::A
                | Self::ASharp
                | Self::BFlat
                | Self::B
                | Self::C
                | Self::CSharp
                | Self::DFlat
                | Self::D
                | Self::DSharp
                | Self::EFlat
                | Self::E
                | Self::F
                | Self::FSharp
                | Self::GFlat
                | Self::G
                | Self::GSharp
        )
    }

    /// Obtains a numeric value from 0 to 11 representing the pitch class, treating enharmonic
    /// equivalents as equal.
    pub fn get_value(&self) -> u8 {
        match self {
            Self::C | Self::BSharp | Self::DDoubleFlat => 0,
            Self::CSharp | Self::DFlat | Self::BDoubleSharp => 1,
            Self::D | Self::CDoubleSharp | Self::EDoubleFlat => 2,
            Self::DSharp | Self::EFlat | Self::FDoubleFlat => 3,
            Self::E | Self::FFlat | Self::DDoubleSharp => 4,
            Self::F | Self::ESharp | Self::GDoubleFlat => 5,
            Self::FSharp | Self::GFlat | Self::EDoubleSharp => 6,
            Self::G | Self::FDoubleSharp | Self::ADoubleFlat => 7,
            Self::GSharp | Self::AFlat => 8,
            Self::A | Self::GDoubleSharp | Self::BDoubleFlat => 9,
            Self::ASharp | Self::BFlat | Self::CDoubleFlat => 10,
            Self::B | Self::CFlat | Self::ADoubleSharp => 11,
        }
    }

    pub fn get_letter(&self) -> char {
        match self {
            Self::ADoubleFlat | Self::AFlat | Self::A | Self::ASharp | Self::ADoubleSharp => 'A',
            Self::BDoubleFlat | Self::BFlat | Self::B | Self::BSharp | Self::BDoubleSharp => 'B',
            Self::CDoubleFlat | Self::CFlat | Self::C | Self::CSharp | Self::CDoubleSharp => 'C',
            Self::DDoubleFlat | Self::DFlat | Self::D | Self::DSharp | Self::DDoubleSharp => 'D',
            Self::EDoubleFlat | Self::EFlat | Self::E | Self::ESharp | Self::EDoubleSharp => 'E',
            Self::FDoubleFlat | Self::FFlat | Self::F | Self::FSharp | Self::FDoubleSharp => 'F',
            Self::GDoubleFlat | Self::GFlat | Self::G | Self::GSharp | Self::GDoubleSharp => 'G',
        }
    }

    pub fn is_double_sharp(&self) -> bool {
        matches!(
            self,
            Self::ADoubleSharp
                | Self::BDoubleSharp
                | Self::CDoubleSharp
                | Self::DDoubleSharp
                | Self::EDoubleSharp
                | Self::FDoubleSharp
                | Self::GDoubleSharp
        )
    }

    pub fn is_double_flat(&self) -> bool {
        matches!(
            self,
            Self::ADoubleFlat
                | Self::BDoubleFlat
                | Self::CDoubleFlat
                | Self::DDoubleFlat
                | Self::EDoubleFlat
                | Self::FDoubleFlat
                | Self::GDoubleFlat
        )
    }

    pub fn offset_strict(&self, interval: i8, function: i8) -> Option<Self> {
        let letter = self.get_letter();
        let value = self.get_value();
        let next_letter = get_letter_at_offset(letter, function).unwrap();
        let next_value = (value as i8 + interval.rem_euclid(12)).rem_euclid(12) as u8;
        Self::from_letter_and_value(next_letter, next_value).ok()
    }

    pub fn next_strict(&self) -> Option<Self> {
        self.offset_strict(1, 1)
    }

    pub fn prev_strict(&self) -> Option<Self> {
        self.offset_strict(-1, -1)
    }

    // Guaranteed to return a standard pitch class
    pub fn offset(&self, offset: i8) -> Self {
        let mut curr = self.convert_to_standard();
        if offset < 0 {
            for _ in 0..-offset {
                curr = curr.prev();
            }
        } else {
            for _ in 0..offset {
                curr = curr.next();
            }
        }
        curr
    }

    pub fn next(&self) -> Self {
        let curr = self.convert_to_standard();
        // This function will never panic because the next strict value of a standard value is
        // always defined.
        let next = curr.next_strict().unwrap();
        if next.is_standard() {
            next
        } else {
            next.convert_to_standard()
        }
    }

    pub fn prev(&self) -> Self {
        let curr = self.convert_to_standard();
        // This function will never panic because the previous strict value of a standard value is
        // always defined.
        let prev = curr.prev_strict().unwrap();
        if prev.is_standard() {
            prev
        } else {
            prev.convert_to_standard()
        }
    }

    pub fn convert_to_standard(&self) -> Self {
        let mut new_pitch_class = if self.is_double_sharp() {
            self.offset_strict(0, 1).unwrap()
        } else if self.is_double_flat() {
            self.offset_strict(0, -1).unwrap()
        } else {
            *self
        };
        if !new_pitch_class.is_standard() {
            new_pitch_class = Self::from_value(self.get_value()).unwrap()
        }
        new_pitch_class
    }
}

impl PartialEq for PitchClass {
    fn eq(&self, other: &Self) -> bool {
        self.get_value() == other.get_value()
    }
}

impl Hash for PitchClass {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_value().hash(state);
    }
}

impl TryFrom<Chord> for Vec<PitchClass> {
    type Error = IncompleteChordError;

    fn try_from(value: Chord) -> Result<Self, Self::Error> {
        if value.get_tonic().is_none() {
            return Err(IncompleteChordError {
                needs_tonic: true,
                needs_octave: false,
                has_tonic: value.get_tonic().is_some(),
                has_octave: value.get_octave().is_some(),
            });
        }
        let mut pitch_classes: Vec<PitchClass> = Vec::new();
        for interval in value.get_intervals() {
            pitch_classes.push(
                value
                    .get_tonic()
                    .unwrap()
                    .offset(interval.get_semitones() as i8),
            );
        }
        Ok(pitch_classes)
    }
}

impl fmt::Display for PitchClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::ADoubleFlat => "A♭♭",
            Self::AFlat => "A♭",
            Self::A => "A",
            Self::ASharp => "A♯",
            Self::ADoubleSharp => "A♯♯",
            Self::BDoubleFlat => "B♭♭",
            Self::BFlat => "B♭",
            Self::B => "B",
            Self::BSharp => "B♯",
            Self::BDoubleSharp => "B♯♯",
            Self::CDoubleFlat => "C♭♭",
            Self::CFlat => "C♭",
            Self::C => "C",
            Self::CSharp => "C♯",
            Self::CDoubleSharp => "C♯♯",
            Self::DDoubleFlat => "D♭♭",
            Self::DFlat => "D♭",
            Self::D => "D",
            Self::DSharp => "D♯",
            Self::DDoubleSharp => "D♯♯",
            Self::EDoubleFlat => "E♭♭",
            Self::EFlat => "E♭",
            Self::E => "E",
            Self::ESharp => "E♯",
            Self::EDoubleSharp => "E♯♯",
            Self::FDoubleFlat => "F♭♭",
            Self::FFlat => "F♭",
            Self::F => "F",
            Self::FSharp => "F♯",
            Self::FDoubleSharp => "F♯♯",
            Self::GDoubleFlat => "G♭♭",
            Self::GFlat => "G♭",
            Self::G => "G",
            Self::GSharp => "G♯",
            Self::GDoubleSharp => "G♯♯",
        };
        write!(f, "{name}")
    }
}

fn get_letter_at_offset(letter: char, offset: i8) -> Result<char, InputError> {
    let letter = letter.to_ascii_uppercase();
    if !('A'..='G').contains(&letter) {
        Err(InputError {
            message: format!(
                "invalid letter {}, provided, must be between A and G",
                letter
            ),
        })
    } else {
        let numeric_value = letter as u8 - b'A';
        let offseted_value = (numeric_value as i8 + offset.rem_euclid(7)).rem_euclid(7);
        Ok((offseted_value as u8 + b'A') as char)
    }
}
