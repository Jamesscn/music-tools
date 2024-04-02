use crate::common::InputError;
use crate::interval::Interval;
use crate::pitchclass::{PitchClass, TwelveTone};
use regex::Regex;
use std::cmp::Ordering;
use std::fmt;
use std::hash::Hash;

/// A structure which is used to represent a note with a pitch class and an octave.
#[derive(Copy, Clone, Debug)]
pub struct Note<PitchClassType: PitchClass = TwelveTone> {
    pitch_class: PitchClassType,
    octave: i8,
}

// Contains functions that assume the twelve tone pitch class system.
impl Note<TwelveTone> {
    /// Constructs a [`Note`] in twelve tone equal temperament tuning from a string containing the
    /// pitch class and the octave of the note. The function returns a [`Result`] which can contain
    /// the note or an [`InputError`] if the input string was invalid.
    ///
    /// # Parameters
    ///
    /// - `string`: A string with the uppercase letter of the pitch class, which can be followed by
    ///   one or two `#` or `♯` symbols to indicate it is a sharp or double sharp pitch class, or
    ///   one or two `b` or `♭` to indicate that it is a flat or double flat note. The string must
    ///   then followed by a number representing the octave of the note. The `♮`, `x` and `X`
    ///   symbols are also valid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use music_tools::note::Note;
    ///
    /// let a = Note::from_string("A#5").unwrap();
    /// let b = Note::from_string("Bb4").unwrap();
    /// let c = Note::from_string("C3").unwrap();
    /// assert_eq!(a.get_frequency(), 932.3277);
    /// assert_eq!(b.get_frequency(), 466.16385);
    /// assert_eq!(c.get_frequency(), 130.81277);
    /// ```
    pub fn from_string(string: &str) -> Result<Self, InputError> {
        let regex =
            Regex::new(r"^([A-Ga-g])(♮|x|X|b{1,2}|♭{1,2}|\#{1,2}|♯{1,2})?(\-?\d+)$").unwrap();
        if !regex.is_match(string) {
            return Err(InputError {
                message: String::from("string does not conform to expected note format"),
            });
        }
        let regex_capture_groups = regex.captures(string).unwrap();
        let pitch_class_letter = regex_capture_groups.get(1).map_or("", |x| x.as_str());
        let accidental = regex_capture_groups.get(2).map_or("", |x| x.as_str());
        let octave: i8 = regex_capture_groups
            .get(3)
            .map_or(0, |x| x.as_str().parse::<i8>().unwrap());
        let pitch_class =
            TwelveTone::from_string(format!("{pitch_class_letter}{accidental}").as_str())?;
        Ok(Self {
            pitch_class,
            octave,
        })
    }

    /// Constructs a [`Note`] in twelve tone equal temperament tuning from a midi index between 0
    /// and 127. The function returns a [`Result`] which can contain the note or an [`InputError`]
    /// if the input value was not valid.
    ///
    /// # Parameters
    ///
    /// - `index`: The index of the midi note, which can be any number between 0 and 127 inclusive.
    pub fn from_midi_index(index: u8) -> Result<Self, InputError> {
        if index > 127 {
            return Err(InputError {
                message: format!(
                    "cannot create note, the midi index {} must be an integer between 0 and 127",
                    index
                ),
            });
        }
        let pitch_class = TwelveTone::from_value(index % 12).unwrap();
        let octave = (index / 12) as i8 - 1;
        Ok(Self {
            pitch_class,
            octave,
        })
    }

    /// Returns a [`Result<u8>`] with an index representing the numerical position of the note on a
    /// keyboard with 88 keys starting at A0 and ending at C8, or an [`InputError`] if the key is
    /// outside of this range or a pitch class system with other than 12 pitch classes is used.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use music_tools::note::Note;
    /// use music_tools::pitchclass::*;
    ///
    /// let middle_c = Note::new(C, 4);
    /// assert_eq!(40, middle_c.get_keyboard_index().unwrap());
    /// ```
    pub fn get_keyboard_index(&self) -> Result<u8, InputError> {
        let keyboard_index = self.get_value() - 8;
        if !(1..=88).contains(&keyboard_index) {
            return Err(InputError {
                message: format!(
                    concat!(
                        "note {} does not have a keyboard index because it is out of range, ",
                        "expected value between 1 and 88, got {}"
                    ),
                    self, keyboard_index
                ),
            });
        }
        Ok(keyboard_index as u8)
    }

    /// Returns an [`Result<u8>`] with the value of the current note according to the MIDI standard,
    /// or an [`InputError`] if the note is outside of the range playable by MIDI or a pitch class
    /// system with other than 12 pitch classes is used.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use music_tools::note::Note;
    /// use music_tools::pitchclass::*;
    ///
    /// let middle_c = Note::new(C, 4);
    /// assert_eq!(60, middle_c.get_midi_index().unwrap());
    /// ```
    pub fn get_midi_index(&self) -> Result<u8, InputError> {
        let midi_index = self.get_value() + 12;
        if !(0..=127).contains(&midi_index) {
            return Err(InputError {
                message: format!(
                    concat!(
                        "note {} does not have a midi index because it is out of range, ",
                        "expected value between 0 and 127, got {}"
                    ),
                    self, midi_index
                ),
            });
        }
        Ok(midi_index as u8)
    }
}

// Contains functions that work for any pitch class system.
impl<PitchClassType: PitchClass> Note<PitchClassType> {
    /// Constructs a [`Note`] in equal temperament tuning from a pitch class and an octave.
    ///
    /// # Parameters
    ///
    /// - `pitch_class`: A [`PitchClass`] representing the pitch class of the note to be
    ///   constructed.
    /// - `octave`: An integer representing the octave of the note to be constructed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use music_tools::note::Note;
    /// use music_tools::pitchclass::*;
    ///
    /// let a = Note::new(A_SHARP, 5);
    /// let b = Note::new(B_FLAT, 4);
    /// let c = Note::new(C, 3);
    /// assert_eq!(a.get_frequency(), 932.3277);
    /// assert_eq!(b.get_frequency(), 466.16385);
    /// assert_eq!(c.get_frequency(), 130.81277);
    /// ```
    pub fn new(pitch_class: PitchClassType, octave: i8) -> Self {
        Self {
            pitch_class,
            octave,
        }
    }

    pub fn change_pitch_class<NewPitchClassType: PitchClass>(
        note: Self,
        pitch_class: NewPitchClassType,
    ) -> Note<NewPitchClassType> {
        Note {
            pitch_class,
            octave: note.octave,
        }
    }

    /// Returns a [`Note`] that is a certain offset away from the current note with the same base
    /// frequency as the current note.
    ///
    /// # Parameters
    ///
    /// - `offset`: A signed integer representing the offset of the new note to return from the
    /// current one.
    pub fn offset(&self, offset: isize) -> Self {
        Self {
            pitch_class: self.pitch_class.offset(offset),
            octave: self.octave
                + (self.pitch_class.get_value() as isize + offset).div_floor(
                    PitchClassType::get_num_classes()
                        .try_into()
                        .expect("could not convert num classes to isize"),
                ) as i8,
        }
    }

    /// Returns the next [`Note`] after the current one.
    pub fn next(&self) -> Self {
        self.offset(1)
    }

    /// Returns the previous [`Note`] before the current one.
    pub fn prev(&self) -> Self {
        self.offset(-1)
    }

    pub fn set_pitch_class(&mut self, pitch_class: PitchClassType) {
        self.pitch_class = pitch_class;
    }

    pub fn set_octave(&mut self, octave: i8) {
        self.octave = octave;
    }

    /// Returns the octave of the current note.
    pub fn get_octave(&self) -> i8 {
        self.octave
    }

    /// Returns a [`PitchClass`] representing the pitch class of the note.
    pub fn get_pitch_class(&self) -> &PitchClassType {
        &self.pitch_class
    }

    /// Returns a numerical value representing the position of the note with respect to C0 or the
    /// lowest pitch class in the pitch class system at octave 0. If a key is below C0, then this
    /// function will return a negative integer representing that note, or if it is above then the
    /// function will return a positive integer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use music_tools::note::Note;
    /// use music_tools::pitchclass::*;
    ///
    /// let c_minus_one = Note::new(C, -1);
    /// let zero = Note::new(C, 0);
    /// let middle_c = Note::new(C, 4);
    /// assert_eq!(-12, c_minus_one.get_value());
    /// assert_eq!(0, zero.get_value());
    /// assert_eq!(48, middle_c.get_value());
    /// ```
    pub fn get_value(&self) -> i32 {
        self.octave as i32 * PitchClassType::get_num_classes() as i32
            + self.pitch_class.get_value() as i32
    }

    pub fn get_interval_with<IntervalType: Interval>(
        &self,
        note: Note<PitchClassType>,
    ) -> Option<IntervalType> {
        let first_value = self.get_value();
        let second_value = note.get_value();
        let difference: usize = if first_value <= second_value {
            (second_value - first_value) as usize
        } else {
            (first_value - second_value) as usize
        };
        IntervalType::from_semitones(difference)
    }

    pub fn offset_interval(&self, interval: impl Interval) -> Self {
        self.offset(interval.get_semitones() as isize)
    }
}

impl Default for Note<TwelveTone> {
    fn default() -> Self {
        Self {
            pitch_class: TwelveTone::default(),
            octave: 4,
        }
    }
}

impl<PC: PitchClass> PartialEq for Note<PC> {
    fn eq(&self, other: &Self) -> bool {
        self.get_value() == other.get_value()
    }
}

impl<PC: PitchClass> Eq for Note<PC> {}

impl<PC: PitchClass> PartialOrd for Note<PC> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<PC: PitchClass> Ord for Note<PC> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_value().cmp(&other.get_value())
    }
}

impl<PC: PitchClass> Hash for Note<PC> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_value().hash(state);
    }
}

impl<PC: PitchClass> fmt::Display for Note<PC> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.pitch_class, self.octave)
    }
}

impl TryFrom<&str> for Note<TwelveTone> {
    type Error = InputError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_string(value)
    }
}

/*
impl<PC: PitchClass, T: Tuning> TryFrom<Chord> for Vec<GenericNote<PC, T>> {
    type Error = IncompleteChordError;

    fn try_from(value: Chord) -> Result<Self, Self::Error> {
        if value.get_tonic().is_none() || value.get_octave().is_none() {
            return Err(IncompleteChordError {
                needs_tonic: true,
                needs_octave: true,
                has_tonic: value.get_tonic().is_some(),
                has_octave: value.get_octave().is_some(),
            });
        }
        let mut notes: Vec<Note> = Vec::new();
        let start_note = Note::new(value.get_tonic().unwrap(), value.get_octave().unwrap());
        for interval in value.get_intervals() {
            let current_note = start_note.offset(interval.get_semitones() as isize);
            notes.push(current_note);
        }
        Ok(notes)
    }
}
 */
