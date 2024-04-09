use crate::common::{result_from_iterator, InputError, TriadQuality};
use crate::interval::Interval;
use crate::note::Note;
use crate::pitchclass::{PitchClass, TwelveTone};
use crate::scale::Scale;
use regex::Regex;
use std::fmt;
use std::hash::Hash;
use std::marker::PhantomData;

pub trait ChordTrait {
    fn add_semitone(&mut self, semitone: impl Into<isize>);
    fn remove_semitone(&mut self, semitone: impl Into<usize>);
    fn to_semitones(&self) -> Vec<usize>;
    /// Sets the inversion of the current chord which changes the order of the intervals in the
    /// chord.
    ///
    /// # Parameters
    ///
    /// - `inversion`: The inversion number to offset the intervals by. This number must be
    ///   positive, and if it exceeds the number of intervals it is automatically wrapped around.
    ///
    /// # Examples
    ///
    /// The following example constructs the first inversion of the major triad.
    ///
    /// ```rust
    /// use music_tools::chord::Chord;
    /// use music_tools::common::TriadQuality;
    ///
    /// let mut chord = Chord::from_triad(TriadQuality::Major, None, None);
    /// chord.set_inversion(1);
    /// ```
    ///
    /// The following example constructs the second inversion of the C minor triad.
    ///
    /// ```rust
    /// use music_tools::chord::Chord;
    /// use music_tools::common::TriadQuality;
    /// use music_tools::pitchclass::PitchClass;
    ///
    /// let mut chord = Chord::from_triad(TriadQuality::Minor, Some(PitchClass::C), None);
    /// chord.set_inversion(2);
    /// assert_eq!(
    ///     Vec::<PitchClass>::try_from(chord).unwrap(),
    ///     vec![PitchClass::G, PitchClass::C, PitchClass::EFlat]
    /// );
    /// ```
    fn set_inversion(&mut self, inversion: impl Into<usize>);
    /// Returns a positive integer representing the inversion of the current chord.
    fn get_inversion(&self) -> usize;
    fn to_intervals(&self) -> Result<Vec<Interval>, InputError>;
}

// Assumptions: Chords must always have a sorted list of semitones and must always contain the value
// zero, representing the tonic or base semitone.
// For example semitones = [1, 3, 5] is invalid, but semitones = [0, 1, 3, 5] is valid.

#[derive(Clone, Debug)]
pub struct GenericChord<PitchClassType: PitchClass> {
    pitch_class_type: PhantomData<PitchClassType>,
    semitones: Vec<usize>,
    inversion: usize,
}

#[derive(Clone, Debug)]
pub struct NoteChord<PitchClassType: PitchClass> {
    base_note: Note<PitchClassType>,
    semitones: Vec<usize>,
    inversion: usize,
}

// The following macro is defined to allow NoteChord and GenericChord to share fields while also
// allowing NoteChord to have a generic PitchClass type
macro_rules! add_common_chord_funcs {
    ($struct_name:ident $(, $generic_name: ident, $generic_type: tt)?) => {
        impl$(<$generic_name: $generic_type>)? ChordTrait for $struct_name$(<$generic_name>)? {
            fn add_semitone(&mut self, semitone: impl Into<isize>) {
                let semitone = semitone.into();
                self.add_semitone_common_impl(semitone);
                self.add_semitone_specific_impl(semitone);
            }

            fn remove_semitone(&mut self, semitone: impl Into<usize>) {
                let semitone = semitone.into();
                self.semitones.retain(|curr_semitone| *curr_semitone != semitone);
                if semitone == 0 {
                    if self.semitones.is_empty() {
                        self.semitones = vec![0]
                    } else {
                        let first_offset = self.semitones[0];
                        self.semitones.iter_mut().for_each(|curr_semitone| *curr_semitone -= first_offset);
                    }
                }
            }

            fn to_semitones(&self) -> Vec<usize> {
                let inversion_index = self.inversion % self.semitones.len();
                let mut inverted_semitones: Vec<usize> = self.semitones.iter().enumerate().map(|(index, value)| if index < inversion_index {value + PitchClassType::get_num_classes()} else {*value}).collect();
                inverted_semitones.sort();
                inverted_semitones
            }

            fn set_inversion(&mut self, inversion: impl Into<usize>) {
                self.inversion = inversion.into();
            }

            fn get_inversion(&self) -> usize {
                self.inversion
            }

            fn to_intervals(&self) -> Result<Vec<Interval>, InputError> {
                result_from_iterator(
                    self.to_semitones().windows(2),
                    |window: &[usize]| Interval::from_semitones(window[1] - window[0]),
                    |error| error,
                )
            }
        }

        impl$(<$generic_name: $generic_type>)? $struct_name$(<$generic_name>)? {
            fn add_semitone_common_impl(&mut self, semitone: isize) {
                if semitone < 0 {
                    self.semitones.iter_mut().for_each(|curr_semitone| *curr_semitone += (-semitone) as usize);
                    self.semitones.insert(0, 0);
                } else {
                    let mut insert_index = 0;
                    for (index, value) in self.semitones.iter().enumerate() {
                        if *value == semitone as usize {
                            return;
                        }
                        if *value > semitone as usize {
                            break;
                        }
                        insert_index = index + 1;
                    }
                    self.semitones.insert(insert_index, semitone as usize);
                }
            }
        }
    };
}

add_common_chord_funcs!(GenericChord, PitchClassType, PitchClass);
add_common_chord_funcs!(NoteChord, PitchClassType, PitchClass);

/// Empty structure used to instantiate chords.
pub struct Chord;

impl Chord {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<PitchClassType: PitchClass>() -> GenericChord<PitchClassType> {
        GenericChord::<PitchClassType> {
            pitch_class_type: PhantomData,
            semitones: vec![0],
            inversion: 0,
        }
    }

    pub fn from_semitones<PitchClassType: PitchClass>(
        semitones: &[impl Into<usize> + Clone],
    ) -> GenericChord<PitchClassType> {
        if semitones.is_empty() {
            return Self::new::<PitchClassType>();
        }
        let mut semitones: Vec<usize> = semitones
            .iter()
            .map(|semitone| semitone.clone().into())
            .collect();
        semitones.sort();
        semitones.dedup();
        let base_semitone = semitones[0];
        semitones
            .iter_mut()
            .for_each(|semitone| *semitone -= base_semitone);
        GenericChord::<PitchClassType> {
            pitch_class_type: PhantomData,
            semitones,
            inversion: 0,
        }
    }

    pub fn from_note<PitchClassType: PitchClass>(
        note: Note<PitchClassType>,
    ) -> NoteChord<PitchClassType> {
        NoteChord {
            base_note: note,
            semitones: vec![0],
            inversion: 0,
        }
    }

    pub fn from_notes<PitchClassType: PitchClass>(
        notes: &[Note<PitchClassType>],
    ) -> Result<NoteChord<PitchClassType>, InputError> {
        if notes.is_empty() {
            return Err(InputError {
                message: String::from("cannot create a chord from an empty set of notes"),
            });
        }
        let mut notes = notes.to_vec();
        notes.sort();
        notes.dedup();
        Ok(NoteChord {
            base_note: notes[0].clone(),
            semitones: notes
                .iter()
                .map(|note| (note.get_value() - notes[0].get_value()) as usize)
                .collect(),
            inversion: 0,
        })
    }

    pub fn from_scale<PitchClassType: PitchClass>(scale: Scale) -> GenericChord<PitchClassType> {
        GenericChord {
            pitch_class_type: PhantomData,
            semitones: scale.to_semitones(),
            inversion: 0,
        }
    }

    pub fn from_intervals<PitchClassType: PitchClass>(
        intervals: &[Interval],
    ) -> GenericChord<PitchClassType> {
        let mut cumulative_sum: usize = 0;
        let mut semitones: Vec<usize> = intervals
            .iter()
            .map(|interval| {
                cumulative_sum += interval.to_semitones();
                cumulative_sum
            })
            .collect();
        semitones.insert(0, 0);
        semitones.dedup();
        GenericChord {
            pitch_class_type: PhantomData,
            semitones,
            inversion: 0,
        }
    }

    pub fn from_triad(triad_quality: TriadQuality) -> GenericChord<TwelveTone> {
        GenericChord::<TwelveTone> {
            pitch_class_type: PhantomData,
            semitones: match triad_quality {
                TriadQuality::Major => vec![0, 4, 7],
                TriadQuality::Minor => vec![0, 3, 7],
                TriadQuality::Sus2 => vec![0, 2, 7],
                TriadQuality::Sus4 => vec![0, 5, 7],
                TriadQuality::Augmented => vec![0, 4, 8],
                TriadQuality::Diminished => vec![0, 3, 6],
            },
            inversion: 0,
        }
    }

    pub fn from_numeral(
        numeral: &str,
        base_note: Note<TwelveTone>,
    ) -> Result<NoteChord<TwelveTone>, InputError> {
        let numeral_array = ["I", "II", "III", "IV", "V", "VI", "VII"];
        let numeral_regex =
            Regex::new(r"^(b|♭|\#|♯)?(I|II|III|IV|V|VI|VII|i|ii|iii|iv|v|vi|vii)(°|\+)?(maj7|7)?$")
                .unwrap();
        if !numeral_regex.is_match(numeral) {
            return Err(InputError {
                message: String::from("string does not conform to expected numeral format"),
            });
        }
        let regex_capture_groups = numeral_regex.captures(numeral).unwrap();
        let accidental = regex_capture_groups.get(1).map_or("", |m| m.as_str());
        let letters = regex_capture_groups.get(2).map_or("", |m| m.as_str());
        let quality = regex_capture_groups.get(3).map_or("", |m| m.as_str());
        let seventh = regex_capture_groups.get(4).map_or("", |m| m.as_str());
        let numeral_value = numeral_array
            .iter()
            .position(|&x| x == letters.to_ascii_uppercase())
            .unwrap();
        let triad_quality: TriadQuality;
        if letters.chars().any(char::is_uppercase) {
            if quality == "+" {
                triad_quality = TriadQuality::Augmented;
            } else if quality == "°" {
                return Err(InputError {
                    message: String::from(concat!(
                        "numeral cannot be uppercase and contain ° symbol, it must either be ",
                        "augmented (uppercase with a +) or diminished (lowercase with a °)"
                    )),
                });
            } else {
                triad_quality = TriadQuality::Major;
            }
        } else if quality == "°" {
            triad_quality = TriadQuality::Diminished;
        } else if quality == "+" {
            return Err(InputError {
                message: String::from(concat!(
                    "numeral cannot be lowercase and contain + symbol, it must either be ",
                    "augmented (uppercase with a +) or diminished (lowercase with a °)"
                )),
            });
        } else {
            triad_quality = TriadQuality::Minor;
        }
        let increment: isize;
        if accidental == "b" || accidental == "♭" {
            increment =
                match numeral_value {
                    1 => 1,
                    2 => 3,
                    4 => 6,
                    5 => 8,
                    6 => 10,
                    _ => return Err(InputError {
                        message: String::from(
                            "only numerals ii, II, iii, III, v, V, vi, VI, vii and VII can be flat",
                        ),
                    }),
                };
        } else if accidental == "#" || accidental == "♯" {
            increment = match numeral_value {
                0 => 1,
                1 => 3,
                3 => 6,
                4 => 8,
                5 => 10,
                _ => {
                    return Err(InputError {
                        message: String::from(
                            "only numerals i, I, ii, II, iv, IV, v, V, vi and VI can be sharp",
                        ),
                    })
                }
            };
        } else {
            increment = match numeral_value {
                0 => 0,
                1 => 2,
                2 => 4,
                3 => 5,
                4 => 7,
                5 => 9,
                6 => 11,
                _ => unreachable!(),
            };
        }
        let chord_base_note = base_note.offset(increment);
        let mut chord = Chord::from_triad(triad_quality);
        if seventh == "maj7" {
            chord.add_semitone(Into::<usize>::into(Interval::MAJOR_SEVENTH()) as isize);
        } else if seventh == "7" {
            chord.add_semitone(Into::<usize>::into(Interval::MINOR_SEVENTH()) as isize);
        }
        Ok(chord.set_base_note(chord_base_note))
    }
}

impl<PitchClassType: PitchClass> GenericChord<PitchClassType> {
    pub fn set_base_note(self, base_note: Note<PitchClassType>) -> NoteChord<PitchClassType> {
        NoteChord {
            base_note,
            semitones: self.semitones,
            inversion: self.inversion,
        }
    }

    fn add_semitone_specific_impl(&mut self, _: isize) {}
}

impl<PitchClassType: PitchClass> NoteChord<PitchClassType> {
    pub fn make_generic(self) -> GenericChord<PitchClassType> {
        GenericChord::<PitchClassType> {
            pitch_class_type: PhantomData,
            semitones: self.semitones,
            inversion: self.inversion,
        }
    }

    pub fn add_note(&mut self, note: Note<PitchClassType>) {
        self.add_semitone((note.get_value() - self.base_note.get_value()) as isize);
    }

    pub fn remove_note(&mut self, note: Note<PitchClassType>) {
        let note_diff = (note.get_value() - self.base_note.get_value()) as isize;
        if note_diff >= 0 {
            self.remove_semitone(note_diff as usize);
        }
    }

    pub fn get_base_note(&self) -> Note<PitchClassType> {
        self.base_note.clone()
    }

    pub fn to_notes(&self) -> Vec<Note<PitchClassType>> {
        self.to_semitones()
            .iter()
            .map(|semitone| self.base_note.offset(*semitone as isize))
            .collect()
    }

    fn add_semitone_specific_impl(&mut self, semitone: isize) {
        if semitone < 0 {
            self.base_note = self.base_note.offset(semitone);
        }
    }
}

impl<PitchClassType: PitchClass> Default for GenericChord<PitchClassType> {
    fn default() -> Self {
        Chord::new()
    }
}

impl<PitchClassType: PitchClass> PartialEq for GenericChord<PitchClassType> {
    fn eq(&self, other: &Self) -> bool {
        self.to_semitones() == other.to_semitones()
    }
}

impl<PitchClassType: PitchClass> PartialEq for NoteChord<PitchClassType> {
    fn eq(&self, other: &Self) -> bool {
        self.base_note == other.base_note && self.to_semitones() == other.to_semitones()
    }
}

impl<PitchClassType: PitchClass> Hash for GenericChord<PitchClassType> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_semitones().hash(state);
    }
}

impl<PitchClassType: PitchClass> Hash for NoteChord<PitchClassType> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.base_note.hash(state);
        self.to_semitones().hash(state);
    }
}

impl<PitchClassType: PitchClass> fmt::Display for GenericChord<PitchClassType> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl<PitchClassType: PitchClass> fmt::Display for NoteChord<PitchClassType> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl From<TriadQuality> for GenericChord<TwelveTone> {
    fn from(value: TriadQuality) -> Self {
        Chord::from_triad(value)
    }
}

impl<PitchClassType: PitchClass> From<Note<PitchClassType>> for NoteChord<PitchClassType> {
    fn from(value: Note<PitchClassType>) -> Self {
        Chord::from_note(value)
    }
}

impl<PitchClassType: PitchClass> FromIterator<Note<PitchClassType>> for NoteChord<PitchClassType> {
    fn from_iter<T: IntoIterator<Item = Note<PitchClassType>>>(iter: T) -> Self {
        let notes: Vec<Note<PitchClassType>> = iter.into_iter().collect();
        Chord::from_notes(&notes).unwrap()
    }
}

impl<PitchClassType: PitchClass> From<Scale> for GenericChord<PitchClassType> {
    fn from(value: Scale) -> Self {
        Chord::from_scale(value)
    }
}

impl<PitchClassType: PitchClass> From<usize> for GenericChord<PitchClassType> {
    fn from(value: usize) -> Self {
        Chord::from_semitones(&[value])
    }
}

impl<PitchClassType: PitchClass> FromIterator<usize> for GenericChord<PitchClassType> {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let semitones: Vec<usize> = iter.into_iter().collect();
        Chord::from_semitones(&semitones)
    }
}

impl<PitchClassType: PitchClass> From<Interval> for GenericChord<PitchClassType> {
    fn from(value: Interval) -> Self {
        Chord::from_intervals(&[value])
    }
}

impl<PitchClassType: PitchClass> FromIterator<Interval> for GenericChord<PitchClassType> {
    fn from_iter<T: IntoIterator<Item = Interval>>(iter: T) -> Self {
        let intervals: Vec<Interval> = iter.into_iter().collect();
        Chord::from_intervals(&intervals)
    }
}

impl<PitchClassType: PitchClass> From<GenericChord<PitchClassType>> for String {
    fn from(value: GenericChord<PitchClassType>) -> Self {
        value.to_string()
    }
}

impl<PitchClassType: PitchClass> From<NoteChord<PitchClassType>> for String {
    fn from(value: NoteChord<PitchClassType>) -> Self {
        value.to_string()
    }
}

impl<PitchClassType: PitchClass> From<GenericChord<PitchClassType>> for Vec<usize> {
    fn from(value: GenericChord<PitchClassType>) -> Self {
        value.to_semitones()
    }
}

impl<PitchClassType: PitchClass> From<NoteChord<PitchClassType>> for Vec<usize> {
    fn from(value: NoteChord<PitchClassType>) -> Self {
        value.to_semitones()
    }
}

impl<PitchClassType: PitchClass> TryFrom<GenericChord<PitchClassType>> for Vec<Interval> {
    type Error = InputError;

    fn try_from(value: GenericChord<PitchClassType>) -> Result<Self, Self::Error> {
        value.to_intervals()
    }
}

impl<PitchClassType: PitchClass> TryFrom<NoteChord<PitchClassType>> for Vec<Interval> {
    type Error = InputError;

    fn try_from(value: NoteChord<PitchClassType>) -> Result<Self, Self::Error> {
        value.to_intervals()
    }
}

impl<PitchClassType: PitchClass> From<NoteChord<PitchClassType>> for Vec<Note<PitchClassType>> {
    fn from(value: NoteChord<PitchClassType>) -> Self {
        value.to_notes()
    }
}
