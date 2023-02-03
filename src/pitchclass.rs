/// A structure used to define one of the pitch classes of the twelve tone
/// equal temperament system.
pub struct PitchClass {
    value: u8,
    names: [&'static str; 3]
}

impl PitchClass {
    /// Returns a [`PitchClass`] representing a pitch class given its name.
    /// 
    /// # Parameters
    /// 
    /// - `pitch_class_name`: A string representing the name of the pitch
    /// class to return. This string can contain flats, sharps, double flats
    /// or double sharps.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use musictools::pitchclass::PitchClass;
    /// 
    /// let a = PitchClass::from_name(String::from("A"));
    /// let b_flat = PitchClass::from_name(String::from("Bb"));
    /// let cx = PitchClass::from_name(String::from("Cx"));
    /// let dbb = PitchClass::from_name(String::from("Dbb"));
    /// ```
    pub fn from_name(pitch_class_name: String) -> Option<&'static PitchClass> {
        for pitch_class_index in 0..12 {
            let pitch_class = &PITCH_CLASSES[pitch_class_index];
            for current_name in pitch_class.names {
                if current_name == pitch_class_name {
                    return Some(&PITCH_CLASSES[pitch_class_index]);
                } 
            }
        }
        return None;
    }

    /// Given a letter from A to G and an offset, this function returns the
    /// letter at a given offset from the provided letter.
    /// 
    /// # Parameters
    /// 
    /// - `letter`: A [`char`] with the letter to offset.
    /// - `offset`: A positive or negative integer to offset `letter` by.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use musictools::pitchclass::PitchClass;
    /// 
    /// let positive_offset = PitchClass::get_letter_at_offset('F', 2).unwrap();
    /// let negative_offset = PitchClass::get_letter_at_offset('F', -2).unwrap();
    /// println!("F + 2 = {positive_offset}, F - 2 = {negative_offset}");
    /// ```
    pub fn get_letter_at_offset(letter: char, offset: i8) -> Option<char> {
        const LETTERS: [char; 7] = ['A', 'B', 'C', 'D', 'E', 'F', 'G'];
        let letter_option = LETTERS.iter().position(|&x| x == letter);
        return match letter_option {
            Some(letter_index) => Some(LETTERS[((letter_index as i8 + offset) % 7) as usize]),
            None => None
        }
    }

    /// Returns the [`PitchClass`] that is a certain offset away from the
    /// current one.
    /// 
    /// # Parameters
    /// 
    /// - `offset`: The offset of the pitch class to return with respect to
    /// the current pitch class.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let c = PitchClasses::A.get_offset(2);
    /// let f = PitchClasses::A.get_offset(-2);
    /// ```
    pub fn get_offset(&self, offset: i8) -> &'static PitchClass {
        return &PITCH_CLASSES[((self.value as i8 + offset) % 12) as usize];
    }

    /// Obtains a numeric value from 0 to 11 representing the pitch class,
    /// where 0 corresponds to the pitch class for C, 1 to C# and so on.
    pub fn get_value(&self) -> u8 {
        return self.value;
    }

    /// Returns the letter for this pitch class, preferring the sharp notation
    /// over the flat notation.
    pub fn get_name(&self) -> &'static str {
        return self.names[0];
    }

    /// Returns a list of all valid names for this pitch class.
    pub fn get_all_names(&self) -> Vec<&'static str> {
        let mut names: Vec<&'static str> = Vec::from(self.names);
        if names[2] == "Ab" {
            names.remove(2);
        }
        return names;
    }
}

const PITCH_CLASSES: [PitchClass; 12] = [
    PitchClass {
        value: 0,
        names: ["C", "B#", "Dbb"]
    },
    PitchClass {
        value: 1,
        names: ["C#", "Db", "Bx"]
    },
    PitchClass {
        value: 2,
        names: ["D", "Ebb", "Cx"]
    },
    PitchClass {
        value: 3,
        names: ["D#", "Eb", "Fbb"]
    },
    PitchClass {
        value: 4,
        names: ["E", "Fb", "Dx"]
    },
    PitchClass {
        value: 5,
        names: ["F", "E#", "Gbb"]
    },
    PitchClass {
        value: 6,
        names: ["F#", "Gb", "Ex"]
    },
    PitchClass {
        value: 7,
        names: ["G", "Abb", "Fx"]
    },
    PitchClass {
        value: 8,
        names: ["G#", "Ab", "Ab"]
    },
    PitchClass {
        value: 9,
        names: ["A", "Bbb", "Gx"]
    },
    PitchClass {
        value: 10,
        names: ["A#", "Bb", "Cbb"]
    },
    PitchClass {
        value: 11,
        names: ["B", "Cb", "Ax"]
    }
];

/// A structure which can be used to obtain a reference to one of the twelve
/// equal temperament pitch classes.
pub struct PitchClasses;

impl PitchClasses {
    /// The pitch class for C.
    pub const C: &PitchClass = &PITCH_CLASSES[0];
    /// The pitch class for C sharp, which is equal to D flat in this library.
    pub const C_SHARP: &PitchClass = &PITCH_CLASSES[1];
    /// The pitch class for D flat, which is equal to C sharp in this library.
    pub const D_FLAT: &PitchClass = &PITCH_CLASSES[1];
    /// The pitch class for D.
    pub const D: &PitchClass = &PITCH_CLASSES[2];
    /// The pitch class for D sharp, which is equal to E flat in this library.
    pub const D_SHARP: &PitchClass = &PITCH_CLASSES[3];
    /// The pitch class for E flat, which is equal to D sharp in this library.
    pub const E_FLAT: &PitchClass = &PITCH_CLASSES[3];
    /// The pitch class for E.
    pub const E: &PitchClass = &PITCH_CLASSES[4];
    /// The pitch class for F.
    pub const F: &PitchClass = &PITCH_CLASSES[5];
    /// The pitch class for F sharp, which is equal to G flat in this library.
    pub const F_SHARP: &PitchClass = &PITCH_CLASSES[6];
    /// The pitch class for G flat, which is equal to F sharp in this library.
    pub const G_FLAT: &PitchClass = &PITCH_CLASSES[6];
    /// The pitch class for G.
    pub const G: &PitchClass = &PITCH_CLASSES[7];
    /// The pitch class for G sharp, which is equal to A flat in this library.
    pub const G_SHARP: &PitchClass = &PITCH_CLASSES[8];
    /// The pitch class for A flat, which is equal to G sharp in this library.
    pub const A_FLAT: &PitchClass = &PITCH_CLASSES[8];
    /// The pitch class for A.
    pub const A: &PitchClass = &PITCH_CLASSES[9];
    /// The pitch class for A sharp, which is equal to B flat in this library.
    pub const A_SHARP: &PitchClass = &PITCH_CLASSES[10];
    /// The pitch class for B flat, which is equal to A sharp in this library.
    pub const B_FLAT: &PitchClass = &PITCH_CLASSES[10];
    /// The pitch class for B.
    pub const B: &PitchClass = &PITCH_CLASSES[11];
}