use crate::chord::Chord;
use crate::common::{IncompleteChordError, InputError};
use crate::pitchclass::PitchClass;
use regex::Regex;
use std::cmp::Ordering;
use std::str::FromStr;

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
    /// use music_tools::pitchclass::PitchClasses;
    ///
    /// let a = Note::new(PitchClasses::A_SHARP, 5);
    /// let b = Note::new(PitchClasses::B_FLAT, 4);
    /// let c = Note::new(PitchClasses::C, 3);
    /// ```
    pub fn new(pitch_class: PitchClass, octave: i8) -> Self {
        Self {
            pitch_class,
            octave,
            base_frequency: 440.0,
        }
    }

    /// Constructs a [`Note`] from a midi index between 0 and 127. The function returns a [`Result`]
    /// which can contain the note or an [`InputError`] if the input string was not valid.
    ///
    /// # Parameters
    ///
    /// - `index`: The index of the midi note, which can be any number between 0 and 127 inclusive.
    pub fn from_midi_index(index: u8) -> Result<Self, InputError> {
        if index > 127 {
            return Err(InputError {
                message: "the midi index must be an integer between 0 and 127",
            });
        }
        let pitch_class = PitchClass::try_from(index % 12).unwrap();
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
    pub fn at_offset(&self, offset: isize) -> Self {
        let pitch_class_val = self.pitch_class.get_value() as isize + offset;
        Self {
            pitch_class: PitchClass::try_from(pitch_class_val.rem_euclid(12) as u8).unwrap(),
            octave: self.octave + pitch_class_val.div_floor(12) as i8,
            base_frequency: self.base_frequency,
        }
    }

    /// Returns the next [`Note`] after the current one.
    pub fn next(&self) -> Self {
        let pitch_class_val = self.pitch_class.get_value() as i8 + 1;
        Self {
            pitch_class: PitchClass::try_from(pitch_class_val.rem_euclid(12) as u8).unwrap(),
            octave: self.octave + pitch_class_val.div_floor(12),
            base_frequency: self.base_frequency,
        }
    }

    /// Returns the previous [`Note`] before the current one.
    pub fn prev(&self) -> Self {
        let pitch_class_val = self.pitch_class.get_value() as i8 - 1;
        Self {
            pitch_class: PitchClass::try_from(pitch_class_val.rem_euclid(12) as u8).unwrap(),
            octave: self.octave + pitch_class_val.div_floor(12),
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
    /// use music_tools::pitchclass::PitchClasses;
    ///
    /// let mut note = Note::from(PitchClasses::C, 5);
    /// println!("{}", note.get_frequency());
    /// note.set_base_frequency(432.0);
    /// println!("{}", note.get_frequency());
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
    /// use music_tools::pitchclass::PitchClasses;
    ///
    /// let mut note = Note::from(PitchClasses::C, 5);
    /// println!("{}", note.get_base_frequency());
    /// note.set_base_frequency(432.0);
    /// println!("{}", note.get_base_frequency());
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

    /// Returns a [`Vec<String>`] with a set of names for the current note.
    pub fn get_names(&self) -> Vec<String> {
        let mut names: Vec<String> = Vec::new();
        for pitch_class_name in self.pitch_class.get_names() {
            let mut name = String::new();
            name.push_str(pitch_class_name);
            name.push_str(&self.octave.to_string());
            names.push(name);
        }
        names
    }

    /// Returns a numerical value representing the position of the note with respect to C0. If a key
    /// is below C0, then this function will return a negative integer representing that note, or if
    /// it is above then the function will return a positive integer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use music_tools::note::Note;
    /// use music_tools::pitchclass::PitchClasses;
    ///
    /// let c_minus_one = Note::from(PitchClasses::C, -1);
    /// let zero = Note::from(PitchClasses::C, 0);
    /// let middle_c = Note::from(PitchClasses::C, 4);
    /// println!("{}", c_minus_one.get_value());
    /// println!("{}", zero.get_value());
    /// println!("{}", middle_c.get_value());
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
    /// use music_tools::pitchclass::PitchClasses;
    ///
    /// let middle_c = Note::from(PitchClasses::C, 4);
    /// println!("{}", middle_c.get_keyboard_index().unwrap());
    /// ```
    pub fn get_keyboard_index(&self) -> Option<u8> {
        let keyboard_index = self.get_value() - 8;
        if !(1..=88).contains(&keyboard_index) {
            return None;
        }
        Some(keyboard_index as u8)
    }

    /// Returns an [`Option<u8>`] with the value of the current note according to the MIDI standard,
    /// or [`None`] if the note is outside of the range playable by MIDI.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use music_tools::note::Note;
    /// use music_tools::pitchclass::PitchClasses;
    ///
    /// let middle_c = Note::from(PitchClasses::C, 4);
    /// println!("{}", middle_c.get_keyboard_index().unwrap());
    /// ```
    pub fn get_midi_index(&self) -> Option<u8> {
        let midi_index = self.get_value() + 12;
        if !(0..=127).contains(&midi_index) {
            return None;
        }
        Some(midi_index as u8)
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

impl FromStr for Note {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"^([A-G])(♮|x|b{1,2}|♭{1,2}|\#{1,2}|♯{1,2})?(\-?\d+)$").unwrap();
        if !regex.is_match(s) {
            return Err(InputError {
                message: "string does not conform to expected note format",
            });
        }
        let regex_capture_groups = regex.captures(s).unwrap();
        let pitch_class_letter = regex_capture_groups.get(1).map_or("", |x| x.as_str());
        let accidental = regex_capture_groups.get(2).map_or("", |x| x.as_str());
        let octave: i8 = regex_capture_groups
            .get(3)
            .map_or(0, |x| x.as_str().parse::<i8>().unwrap());
        let pitch_class =
            PitchClass::from_str(format!("{pitch_class_letter}{accidental}").as_str())?;
        Ok(Self {
            pitch_class,
            octave,
            base_frequency: 440.0,
        })
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
                + ((value.get_tonic().unwrap().get_value() as u64 + interval.get_value()) / 12)
                    as i8;
            let current_semitone = interval.get_value() % 12;
            let current_pitch_class = value
                .get_tonic()
                .unwrap()
                .get_offset(current_semitone as i8);
            let current_note = Note::new(current_pitch_class, current_octave);
            notes.push(current_note);
        }
        Ok(notes)
    }
}
