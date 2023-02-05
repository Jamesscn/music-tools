use regex::Regex;
use crate::pitchclass::PitchClass;

#[derive(Debug)]
/// A structure which is used to represent a note with a pitch class and an
/// octave or frequency.
pub struct Note {
    pitch_class: PitchClass,
    octave: u8,
    keyboard_index: u32,
    base_frequency: f32
}

impl Note {
    /// Constructs a note from a string containing the pitch class and the
    /// octave of the note.
    /// 
    /// # Parameters
    /// 
    /// - `string`: A string with the uppercase letter of the pitch class,
    /// which can be followed by a # to indicate it is a sharp pitch class or
    /// a lowercase b to indicate that it is a flat note, and which is then
    /// followed by a number representing the octave of the note.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use musictools::note::Note;
    /// 
    /// let a = Note::from_string("A#5");
    /// let b = Note::from_string("Bb4");
    /// let c = Note::from_string("C3");
    /// ```
    pub fn from_string(string: &str) -> Option<Note> {
        let regex = Regex::new(r"^(A|A\#|Bb|B|C|C\#|Db|D|D\#|Eb|E|F|F\#|Gb|G|G\#|Ab)(\d)$").unwrap();
        if !regex.is_match(&string) {
            return None;
        }
        let regex_capture_groups = regex.captures(&string).unwrap();
        let octave: u8 = (&regex_capture_groups[2]).parse().unwrap();
        let pitch_class_string = String::from(&regex_capture_groups[1]);
        let pitch_class = PitchClass::from_name(pitch_class_string).unwrap();
        return Some(Note {
            pitch_class,
            octave,
            keyboard_index: octave as u32 * 12 + pitch_class.get_value() as u32,
            base_frequency: 440.0
        });
    }

    /// Constructs a note from a pitch class and an octave.
    /// 
    /// # Parameters
    /// 
    /// - `pitch_class`: A [`PitchClass`] representing the pitch class of the
    /// note to be constructed.
    /// - `octave`: A positive integer representing the octave of the note to
    /// be constructed.
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
    pub fn from(pitch_class: PitchClass, octave: u8) -> Note {
        return Note {
            pitch_class,
            octave,
            keyboard_index: octave as u32 * 12 + pitch_class.get_value() as u32,
            base_frequency: 440.0
        }
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
        return self.base_frequency;
    }

    /// Returns an index representing the position of the note on a keyboard.
    /// The lowest note allowed C0 would return the index 0, while the next
    /// note C#0 would return the index 1 and so on.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use musictools::note::Note;
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let lowest = Note::from(PitchClasses::C, 0);
    /// let next = Note::from(PitchClasses::C_SHARP, 0);
    /// println!("{}", lowest.get_keyboard_index());
    /// println!("{}", next.get_keyboard_index());
    /// ```
    pub fn get_keyboard_index(&self) -> u32 {
        return self.keyboard_index;
    }

    /// Retuns the frequency in hertz of the current note. This frequency
    /// depends on the reference frequency for the note A4, which can be
    /// modified by the `set_base_frequency` function.
    pub fn get_frequency(&self) -> f32 {
        return self.base_frequency as f32 * (2.0 as f32).powf(self.octave as f32 + (self.pitch_class.get_value() as i8 - 9) as f32 / 12 as f32 - 4.0);
    }

    /// Returns the octave of the current note.
    pub fn get_octave(&self) -> u8 {
        return self.octave;
    }

    /// Returns a [`PitchClass`] representing the pitch class of the note.
    pub fn get_pitch_class(&self) -> PitchClass {
        return self.pitch_class;
    }
}

impl PartialEq for Note {
    fn eq(&self, other: &Self) -> bool {
        return self.keyboard_index == other.keyboard_index && self.base_frequency == other.base_frequency;
    }
}