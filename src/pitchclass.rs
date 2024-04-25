use crate::common::InputError;
use crate::interval::Interval;
use concat_idents::concat_idents;
use regex::Regex;
use std::{cmp::Ordering, fmt};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PitchClass {
    letter_class: String,
    letter_class_semitones: usize,
    accidental: isize,
    offset_func: fn(&Self, isize, isize) -> Option<Self>,
    offset_lax_func: fn(&Self, isize) -> Self,
    num_classes_func: fn() -> usize,
}

impl PitchClass {
    pub fn offset(&self, semitone_offset: isize, letter_offset: isize) -> Option<Self> {
        (self.offset_func)(self, semitone_offset, letter_offset)
    }

    pub fn offset_lax(&self, semitone_offset: isize) -> Self {
        (self.offset_lax_func)(self, semitone_offset)
    }

    pub fn add_interval(&self, interval: Interval) -> Option<Self> {
        self.offset(
            interval.get_semitones() as isize,
            interval.get_letter_classes() as isize,
        )
    }

    pub fn subtract_interval(&self, interval: Interval) -> Option<Self> {
        self.offset(
            -(interval.get_semitones() as isize),
            -(interval.get_letter_classes() as isize),
        )
    }

    pub fn add_interval_lax(&self, interval: Interval) -> Self {
        self.offset_lax(interval.get_semitones() as isize)
    }

    pub fn subtract_interval_lax(&self, interval: Interval) -> Self {
        self.offset_lax(-(interval.get_semitones() as isize))
    }

    pub fn augment(&self) -> Option<Self> {
        self.offset(1, 0)
    }

    pub fn diminish(&self) -> Option<Self> {
        self.offset(-1, 0)
    }

    pub fn get_letter_class(&self) -> String {
        self.letter_class.clone()
    }

    pub fn get_semitones(&self) -> usize {
        let num_pitch_classes = (self.num_classes_func)();
        (self.letter_class_semitones as isize + self.accidental)
            .rem_euclid(num_pitch_classes as isize) as usize
    }

    pub fn get_accidental(&self) -> isize {
        self.accidental
    }

    pub fn is_natural(&self) -> bool {
        self.accidental == 0
    }

    pub fn is_flat(&self) -> bool {
        self.accidental == -1
    }

    pub fn is_sharp(&self) -> bool {
        self.accidental == 1
    }

    pub fn is_double_flat(&self) -> bool {
        self.accidental == -2
    }

    pub fn is_double_sharp(&self) -> bool {
        self.accidental == 2
    }
}

impl fmt::Display for PitchClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut name = self.letter_class.clone();
        let (accidental_char, repeat_times) = match self.accidental.cmp(&0) {
            Ordering::Greater => ('♯', self.accidental),
            Ordering::Equal => (' ', 0),
            Ordering::Less => ('♭', -self.accidental),
        };
        for _ in 0..repeat_times {
            name.push(accidental_char);
        }
        write!(f, "{name}")
    }
}

pub trait PitchClassSystem {
    fn offset(
        pitch_class: &PitchClass,
        semitone_offset: isize,
        letter_offset: isize,
    ) -> Option<PitchClass> {
        let (letter_semitone_difference, new_letter_class) =
            Self::get_letter_class_at_offset(&pitch_class.get_letter_class(), letter_offset);
        let new_semitones = (pitch_class.letter_class_semitones as isize + semitone_offset)
            .rem_euclid(Self::get_num_pitch_classes() as isize);
        let accidental = pitch_class.accidental - letter_semitone_difference + semitone_offset;
        // Avoid anything more than double sharps or flats. If you want to use strange accidentals
        // such as triple flats and so on, you can re-implement this function without this
        // conditional statement.
        if accidental.abs() > 2 {
            None
        } else {
            Some(PitchClass {
                letter_class: new_letter_class,
                letter_class_semitones: new_semitones as usize,
                accidental,
                ..pitch_class.clone()
            })
        }
    }
    fn offset_lax(pitch_class: &PitchClass, semitone_offset: isize) -> PitchClass;
    fn get_letter_class_at_offset(letter_class: &str, letter_offset: isize) -> (isize, String) {
        let letter_classes = Self::get_letter_classes();
        let letter_index = letter_classes
            .iter()
            .position(|letter| *letter == letter_class)
            .expect("invalid letter class");
        let next_letter_class = &letter_classes[(letter_index as isize + letter_offset)
            .rem_euclid(letter_classes.len() as isize)
            as usize];
        let mut semitone_diff = Self::get_semitones_for_letter_class(next_letter_class) as isize
            - Self::get_semitones_for_letter_class(letter_class) as isize;
        if semitone_diff.signum() != letter_offset.signum() {
            semitone_diff = (semitone_diff.abs() - Self::get_num_pitch_classes() as isize)
                * semitone_diff.signum();
        }
        semitone_diff += Self::get_num_pitch_classes() as isize
            * (letter_offset / letter_classes.len() as isize);
        (semitone_diff, next_letter_class.to_string())
    }
    fn get_semitones_for_letter_class(letter_class: &str) -> usize;
    fn get_letter_classes() -> Vec<String>;
    fn get_num_pitch_classes() -> usize;
}

pub struct TwelveTone;

impl PitchClassSystem for TwelveTone {
    fn offset_lax(pitch_class: &PitchClass, semitone_offset: isize) -> PitchClass {
        let semitones = (pitch_class.get_semitones() as isize + semitone_offset)
            .rem_euclid(Self::get_num_pitch_classes() as isize);
        Self::from_semitones(semitones as usize)
    }

    fn get_letter_classes() -> Vec<String> {
        ["A", "B", "C", "D", "E", "F", "G"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    fn get_num_pitch_classes() -> usize {
        12
    }

    fn get_semitones_for_letter_class(letter_class: &str) -> usize {
        match letter_class {
            "A" => 9,
            "B" => 11,
            "C" => 0,
            "D" => 2,
            "E" => 4,
            "F" => 5,
            "G" => 7,
            _ => panic!("invalid letter class"),
        }
    }
}

macro_rules! regex_match_case {
    ($pitch_class_letter: ident $(, $suffix:expr)*) => {
        match $pitch_class_letter {
            "A" | "a" => concat_idents!(fn_name = A, $($suffix),*{TwelveTone::fn_name()}),
            "B" | "b" => concat_idents!(fn_name = B, $($suffix),*{TwelveTone::fn_name()}),
            "C" | "c" => concat_idents!(fn_name = C, $($suffix),*{TwelveTone::fn_name()}),
            "D" | "d" => concat_idents!(fn_name = D, $($suffix),*{TwelveTone::fn_name()}),
            "E" | "e" => concat_idents!(fn_name = E, $($suffix),*{TwelveTone::fn_name()}),
            "F" | "f" => concat_idents!(fn_name = F, $($suffix),*{TwelveTone::fn_name()}),
            "G" | "g" => concat_idents!(fn_name = G, $($suffix),*{TwelveTone::fn_name()}),
            _ => unreachable!(),
        }
    };
}

impl TwelveTone {
    pub fn from_string(string: &str) -> Result<PitchClass, InputError> {
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
            "" | "♮" => regex_match_case!(pitch_class_letter),
            "b" | "♭" => regex_match_case!(pitch_class_letter, _FLAT),
            "bb" | "♭♭" | "b♭" | "♭b" => {
                regex_match_case!(pitch_class_letter, _DOUBLE_FLAT)
            }
            "#" | "♯" => regex_match_case!(pitch_class_letter, _SHARP),
            "##" | "♯♯" | "#♯" | "♯#" | "x" | "X" => {
                regex_match_case!(pitch_class_letter, _DOUBLE_SHARP)
            }
            _ => unreachable!(),
        })
    }

    pub fn from_semitones(semitones: usize) -> PitchClass {
        match semitones {
            0 => Self::C(),
            1 => Self::C_SHARP(),
            2 => Self::D(),
            3 => Self::D_SHARP(),
            4 => Self::E(),
            5 => Self::F(),
            6 => Self::F_SHARP(),
            7 => Self::G(),
            8 => Self::G_SHARP(),
            9 => Self::A(),
            10 => Self::A_SHARP(),
            11 => Self::B(),
            _ => unimplemented!(),
        }
    }
}

macro_rules! pitch_class {
    ($letter_class: ident) => {
        impl TwelveTone {
            #[allow(non_snake_case)]
            pub fn $letter_class() -> PitchClass {
                let letter_class = stringify!($letter_class);
                PitchClass {
                    letter_class: letter_class.to_string(),
                    letter_class_semitones: Self::get_semitones_for_letter_class(letter_class),
                    accidental: 0,
                    offset_func: TwelveTone::offset,
                    offset_lax_func: TwelveTone::offset_lax,
                    num_classes_func: TwelveTone::get_num_pitch_classes,
                }
            }

            concat_idents!(fn_name = $letter_class, _, FLAT {
                #[allow(non_snake_case)]
                pub fn fn_name() -> PitchClass {
                    let letter_class = stringify!($letter_class);
                    PitchClass {
                        letter_class: letter_class.to_string(),
                        letter_class_semitones: Self::get_semitones_for_letter_class(letter_class),
                        accidental: -1,
                        offset_func: TwelveTone::offset,
                        offset_lax_func: TwelveTone::offset_lax,
                        num_classes_func: TwelveTone::get_num_pitch_classes,
                    }
                }
            });

            concat_idents!(fn_name = $letter_class, _, SHARP {
                #[allow(non_snake_case)]
                pub fn fn_name() -> PitchClass {
                    let letter_class = stringify!($letter_class);
                    PitchClass {
                        letter_class: letter_class.to_string(),
                        letter_class_semitones: Self::get_semitones_for_letter_class(letter_class),
                        accidental: 1,
                        offset_func: TwelveTone::offset,
                        offset_lax_func: TwelveTone::offset_lax,
                        num_classes_func: TwelveTone::get_num_pitch_classes,
                    }
                }
            });

            concat_idents!(fn_name = $letter_class, _, DOUBLE, _, FLAT {
                #[allow(non_snake_case)]
                pub fn fn_name() -> PitchClass {
                    let letter_class = stringify!($letter_class);
                    PitchClass {
                        letter_class: letter_class.to_string(),
                        letter_class_semitones: Self::get_semitones_for_letter_class(letter_class),
                        accidental: -2,
                        offset_func: TwelveTone::offset,
                        offset_lax_func: TwelveTone::offset_lax,
                        num_classes_func: TwelveTone::get_num_pitch_classes,
                    }
                }
            });

            concat_idents!(fn_name = $letter_class, _, DOUBLE, _, SHARP {
                #[allow(non_snake_case)]
                pub fn fn_name() -> PitchClass {
                    let letter_class = stringify!($letter_class);
                    PitchClass {
                        letter_class: letter_class.to_string(),
                        letter_class_semitones: Self::get_semitones_for_letter_class(letter_class),
                        accidental: 2,
                        offset_func: TwelveTone::offset,
                        offset_lax_func: TwelveTone::offset_lax,
                        num_classes_func: TwelveTone::get_num_pitch_classes,
                    }
                }
            });
        }
    };
}

pitch_class!(A);
pitch_class!(B);
pitch_class!(C);
pitch_class!(D);
pitch_class!(E);
pitch_class!(F);
pitch_class!(G);
