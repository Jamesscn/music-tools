use crate::chord::Chord;
use crate::common::{IncompleteChordError, InputError};
use crate::pitchclass::PitchClass;
use regex::Regex;
use std::cmp::Ordering;
use std::fmt;
use std::hash::Hash;

/// A structure which is used to represent a note with a pitch class and an octave or frequency.
#[derive(Copy, Clone, Debug)]
pub struct Note {
    pitch_class: PitchClass,
    octave: i8,
    base_frequency: f32,
}

impl Note {
    /// Constructs a [`Note`] from a pitch class and an octave.
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
    /// use music_tools::pitchclass::PitchClass;
    ///
    /// let a = Note::new(PitchClass::ASharp, 5);
    /// let b = Note::new(PitchClass::BFlat, 4);
    /// let c = Note::new(PitchClass::C, 3);
    /// assert_eq!(a.get_frequency(), 932.3277);
    /// assert_eq!(b.get_frequency(), 466.16385);
    /// assert_eq!(c.get_frequency(), 130.81277);
    /// ```
    pub fn new(pitch_class: PitchClass, octave: i8) -> Self {
        Self {
            pitch_class,
            octave,
            base_frequency: 440.0,
        }
    }

    /// Constructs a [`Note`] from a string containing the pitch class and the octave of the note.
    /// The function returns a [`Result`] which can contain the note or an [`InputError`] if the
    /// input string was not valid.
    ///
    /// # Parameters
    ///
    /// - `string`: A string with the uppercase letter of the pitch class, which can be followed by
    ///   a `#` or `♯` to indicate it is a sharp pitch class or a `b` or `♭` to indicate that it is
    ///   a flat note, and which is then followed by a number representing the octave of the note.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use music_tools::note::Note;
    ///
    /// let a = Note::from_string("A#5").unwrap();
    /// let b = Note::from_string("Bb4").unwrap();
    /// let c = Note::from_string("C3").unwrap();
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
            PitchClass::from_string(format!("{pitch_class_letter}{accidental}").as_str())?;
        Ok(Self {
            pitch_class,
            octave,
            base_frequency: 440.0,
        })
    }

    /// Constructs a [`Note`] from a midi index between 0 and 127. The function returns a [`Result`]
    /// which can contain the note or an [`InputError`] if the input value was not valid.
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
        let pitch_class = PitchClass::from_value(index % 12).unwrap();
        let octave = (index / 12) as i8 - 1;
        Ok(Self {
            pitch_class,
            octave,
            base_frequency: 440.0,
        })
    }

    /// Returns a [`Note`] that is a certain offset away from the current note with the same base
    /// frequency as the current note.
    ///
    /// # Parameters
    ///
    /// - `offset`: A signed integer representing the offset of the new note to return from the
    /// current one.
    pub fn offset(&self, offset: i8) -> Self {
        let offset_pitch_class = self.pitch_class.offset(offset);
        Self {
            pitch_class: offset_pitch_class,
            octave: self.octave + offset_pitch_class.get_value().div_floor(12) as i8,
            base_frequency: self.base_frequency,
        }
    }

    /// Returns the next [`Note`] after the current one.
    pub fn next(&self) -> Self {
        let next_pitch_class = self.pitch_class.next();
        Self {
            pitch_class: next_pitch_class,
            octave: self.octave + next_pitch_class.get_value().div_floor(12) as i8,
            base_frequency: self.base_frequency,
        }
    }

    /// Returns the previous [`Note`] before the current one.
    pub fn prev(&self) -> Self {
        let prev_pitch_class = self.pitch_class.prev();
        Self {
            pitch_class: prev_pitch_class,
            octave: self.octave + prev_pitch_class.get_value().div_floor(12) as i8,
            base_frequency: self.base_frequency,
        }
    }

    /// Changes the reference frequency of A4 to a specific value for this note, which will affect
    /// the frequency of the pitch class and octave when calculated. The default value for this
    /// frequency is equal to 440 hertz.
    ///
    /// # Parameters
    ///
    /// - `base_frequency`: A floating point number which will represent the frequency in hertz of
    ///   the reference note A4 when the frequency of the current note is calculated.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use music_tools::note::Note;
    /// use music_tools::pitchclass::PitchClass;
    ///
    /// let mut note = Note::new(PitchClass::A, 5);
    /// assert_eq!(note.get_frequency(), 880.0);
    /// note.set_base_frequency(432.0);
    /// assert_eq!(note.get_frequency(), 864.0);
    /// ```
    pub fn set_base_frequency(&mut self, base_frequency: f32) {
        self.base_frequency = base_frequency;
    }

    /// Obtains the reference frequency of A4 with respect to this note in hertz. The default value
    /// for this frequency is 440 hertz.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use music_tools::note::Note;
    /// use music_tools::pitchclass::PitchClass;
    ///
    /// let mut note = Note::new(PitchClass::C, 5);
    /// assert_eq!(440.0, note.get_base_frequency());
    /// note.set_base_frequency(432.0);
    /// assert_eq!(432.0, note.get_base_frequency());
    /// ```
    pub fn get_base_frequency(&self) -> f32 {
        self.base_frequency
    }

    /// Retuns the frequency in hertz of the current note. This frequency depends on the reference
    /// frequency for the note A4, which can be modified by the `set_base_frequency` function.
    pub fn get_frequency(&self) -> f32 {
        self.base_frequency
            * 2.0_f32.powf(
                self.octave as f32 + (self.pitch_class.get_value() as i8 - 9) as f32 / 12_f32 - 4.0,
            )
    }

    /// Returns the octave of the current note.
    pub fn get_octave(&self) -> i8 {
        self.octave
    }

    /// Returns a [`PitchClass`] representing the pitch class of the note.
    pub fn get_pitch_class(&self) -> PitchClass {
        self.pitch_class
    }

    /// Returns a numerical value representing the position of the note with respect to C0. If a key
    /// is below C0, then this function will return a negative integer representing that note, or if
    /// it is above then the function will return a positive integer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use music_tools::note::Note;
    /// use music_tools::pitchclass::PitchClass;
    ///
    /// let c_minus_one = Note::new(PitchClass::C, -1);
    /// let zero = Note::new(PitchClass::C, 0);
    /// let middle_c = Note::new(PitchClass::C, 4);
    /// assert_eq!(-12, c_minus_one.get_value());
    /// assert_eq!(0, zero.get_value());
    /// assert_eq!(48, middle_c.get_value());
    /// ```
    pub fn get_value(&self) -> i16 {
        self.octave as i16 * 12 + self.pitch_class.get_value() as i16
    }

    /// Returns an [`Option<u8>`] with an index representing the numerical position of the note on a
    /// keyboard with 88 keys starting at A0 and ending at C8, or [`None`] if the key is outside of
    /// this range.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use music_tools::note::Note;
    /// use music_tools::pitchclass::PitchClass;
    ///
    /// let middle_c = Note::new(PitchClass::C, 4);
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

    /// Returns an [`Option<u8>`] with the value of the current note according to the MIDI standard,
    /// or [`None`] if the note is outside of the range playable by MIDI.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use music_tools::note::Note;
    /// use music_tools::pitchclass::PitchClass;
    ///
    /// let middle_c = Note::new(PitchClass::C, 4);
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

impl Default for Note {
    fn default() -> Self {
        Self {
            pitch_class: PitchClass::default(),
            octave: 4,
            base_frequency: 440.0,
        }
    }
}

impl PartialEq for Note {
    fn eq(&self, other: &Self) -> bool {
        self.get_value() == other.get_value()
    }
}

impl Eq for Note {}

impl PartialOrd for Note {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Note {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_value().cmp(&other.get_value())
    }
}

impl Hash for Note {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_value().hash(state);
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.pitch_class, self.octave)
    }
}

impl TryFrom<Chord> for Vec<Note> {
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
        for interval in value.get_intervals() {
            let current_octave = value.get_octave().unwrap()
                + ((value.get_tonic().unwrap().get_value() as u64 + interval.get_semitones()) / 12)
                    as i8;
            let current_pitch_class = value
                .get_tonic()
                .unwrap()
                .offset(interval.get_semitones() as i8);
            let current_note = Note::new(current_pitch_class, current_octave);
            notes.push(current_note);
        }
        Ok(notes)
    }
}
