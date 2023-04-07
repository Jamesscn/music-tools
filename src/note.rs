use regex::Regex;
use crate::pitchclass::PitchClass;

/// A structure which is used to represent a note with a pitch class and an
/// octave or frequency.
#[derive(Copy, Clone, Debug)]
pub struct Note {
    pitch_class: PitchClass,
    octave: i8,
    base_frequency: f32
}

impl Note {
    /// Constructs a [`Note`] from a pitch class and an octave.
    /// 
    /// # Parameters
    /// 
    /// - `pitch_class`: A [`PitchClass`] representing the pitch class of the
    /// note to be constructed.
    /// - `octave`: An integer representing the octave of the note to be
    /// constructed.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use musictools::note::Note;
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let a = Note::from(PitchClasses::A_SHARP, 5);
    /// let b = Note::from(PitchClasses::B_FLAT, 4);
    /// let c = Note::from(PitchClasses::C, 3);
    /// ```
    pub fn from(pitch_class: PitchClass, octave: i8) -> Note {
        Note {
            pitch_class,
            octave,
            base_frequency: 440.0
        }
    }

    /// Constructs a [`Note`] from a string containing the pitch class and the
    /// octave of the note. If the string is invalid, [`None`] is returned.
    /// 
    /// # Parameters
    /// 
    /// - `string`: A string with the uppercase letter of the pitch class,
    /// which can be followed by a `#` or `♯` to indicate it is a sharp pitch
    /// class or a `b` or `♭` to indicate that it is a flat note, and which is
    /// then followed by a number representing the octave of the note.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use musictools::note::Note;
    /// 
    /// let a = Note::from_string("A#5").unwrap();
    /// let b = Note::from_string("Bb4").unwrap();
    /// let c = Note::from_string("C3").unwrap();
    /// ```
    pub fn from_string(string: &str) -> Option<Note> {
        let regex = Regex::new(r"^([A-G])(♮|x|b{1,2}|♭{1,2}|\#{1,2}|♯{1,2})?(\-?\d+)$").unwrap();
        if !regex.is_match(string) {
            return None;
        }
        let regex_capture_groups = regex.captures(string).unwrap();
        let pitch_class_letter = regex_capture_groups.get(1).map_or("", |x| x.as_str());
        let accidental = regex_capture_groups.get(2).map_or("", |x| x.as_str());
        let octave: i8 = regex_capture_groups.get(3).map_or(0, |x| x.as_str().parse::<i8>().unwrap());
        let pitch_class_option = PitchClass::from_name(format!("{pitch_class_letter}{accidental}").as_str());
        pitch_class_option.map(|pitch_class| Note {
                pitch_class,
                octave,
                base_frequency: 440.0
            })
    }

    /// Constructs a [`Note`] from a midi index between 0 and 127. If the value
    /// provided is outside of this range [`None`] is returned.
    /// 
    /// # Parameters
    /// 
    /// - `index`: The index of the midi note, which can be any number between
    /// 0 and 127 inclusive. 
    pub fn from_midi_index(index: u8) -> Option<Note> {
        if index > 127 {
            return None;
        }
        let pitch_class = PitchClass::from_value(index % 12).unwrap();
        let octave = (index / 12) as i8 - 1;
        Some(Note {
            pitch_class,
            octave,
            base_frequency: 440.0
        })
    }

    /// Changes the reference frequency of A4 to a specific value for this
    /// note, which will affect the frequency of the pitch class and octave
    /// when calculated. The default value for this frequency is equal to 440
    /// hertz.
    /// 
    /// # Parameters
    /// 
    /// - `base_frequency`: A floating point number which will represent the
    /// frequency in hertz of the reference note A4 when the frequency of the
    /// current note is calculated.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use musictools::note::Note;
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let mut note = Note::from(PitchClasses::C, 5);
    /// println!("{}", note.get_frequency());
    /// note.set_base_frequency(432.0);
    /// println!("{}", note.get_frequency());
    /// ```
    pub fn set_base_frequency(&mut self, base_frequency: f32) {
        self.base_frequency = base_frequency;
    }

    /// Obtains the reference frequency of A4 with respect to this note in
    /// hertz. The default value for this frequency is 440 hertz.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use musictools::note::Note;
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let mut note = Note::from(PitchClasses::C, 5);
    /// println!("{}", note.get_base_frequency());
    /// note.set_base_frequency(432.0);
    /// println!("{}", note.get_base_frequency());
    /// ```
    pub fn get_base_frequency(&self) -> f32 {
        self.base_frequency
    }

    /// Retuns the frequency in hertz of the current note. This frequency
    /// depends on the reference frequency for the note A4, which can be
    /// modified by the `set_base_frequency` function.
    pub fn get_frequency(&self) -> f32 {
        self.base_frequency as f32 * 2.0_f32.powf(self.octave as f32 + (self.pitch_class.get_value() as i8 - 9) as f32 / 12_f32 - 4.0)
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

    /// Returns a numerical value representing the position of the note with
    /// respect to C0. If a key is below C0, then this function will return a
    /// negative integer representing that note, or if it is above then the
    /// function will return a positive integer.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use musictools::note::Note;
    /// use musictools::pitchclass::PitchClasses;
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

    /// Returns an [`Option<u8>`] with an index representing the numerical
    /// position of the note on a keyboard with 88 keys starting at A0 and
    /// ending at C8, or [`None`] if the key is outside of this range.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use musictools::note::Note;
    /// use musictools::pitchclass::PitchClasses;
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

    /// Returns an [`Option<u8>`] with the value of the current note according
    /// to the MIDI standard, or [`None`] if the note is outside of the range
    /// playable by MIDI.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use musictools::note::Note;
    /// use musictools::pitchclass::PitchClasses;
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

impl PartialEq for Note {
    fn eq(&self, other: &Self) -> bool {
        self.get_keyboard_index() == other.get_keyboard_index() && self.base_frequency == other.base_frequency
    }
}