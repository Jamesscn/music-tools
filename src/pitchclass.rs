/// A structure used to define one of the pitch classes of the twelve tone
/// equal temperament system.
#[derive(Copy, Clone, Debug)]
pub struct PitchClass {
    value: u8,
    names: &'static [&'static str]
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
    /// let a = PitchClass::from_name("A");
    /// let b_flat = PitchClass::from_name("Bb");
    /// ```
    pub fn from_name(pitch_class_name: &str) -> Option<PitchClass> {
        for pitch_class_index in 0..12 {
            let pitch_class = PITCH_CLASSES[pitch_class_index];
            for current_name in pitch_class.names {
                if current_name == &pitch_class_name {
                    return Some(pitch_class);
                }
            }
        }
        return None;
    }

    /// Returns an [`Option`] with a [`PitchClass`] given its value from 0 to
    /// 11, where 0 represents C, 1 represents D flat and so on. If the index
    /// is greater than 11 then [`None`] is returned.
    /// 
    /// # Parameters
    /// 
    /// - `value`: An integer from 0 to 11 representing the [`PitchClass`]
    /// to return.
    /// - `prefer_flats`: A boolean which indicates whether the function
    /// should return the flat version of the pitch class or the sharp
    /// version depending on the note. For example, if this is set to true
    /// then the function would prefer to return A flat instead of G sharp.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use musictools::pitchclass::PitchClass;
    /// 
    /// let g_flat = PitchClass::from_value(6);
    /// ```
    pub fn from_value(value: u8) -> Option<PitchClass> {
        let index = value as usize;
        if index < 12 {
            return Some(PITCH_CLASSES[index]);
        }
        return None;
    }

    /// Returns the [`PitchClass`] that is a certain offset away from the
    /// current one.
    /// 
    /// # Parameters
    /// 
    /// - `offset`: The offset of the pitch class to return with respect to
    /// the current pitch class.
    /// - `prefer_flats`: A boolean which indicates whether the function
    /// should return the flat version of the pitch class or the sharp
    /// version depending on the note. For example, if this is set to true
    /// then the function would prefer to return A flat instead of G sharp.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let c = PitchClasses::A.get_offset(2);
    /// let f = PitchClasses::A.get_offset(-2);
    /// ```
    pub fn get_offset(&self, offset: i8) -> PitchClass {
        return PITCH_CLASSES[(self.value as i8 + (offset % 12)).rem_euclid(12) as usize];
    }

    /// Obtains a numeric value from 0 to 11 representing the pitch class,
    /// where 0 corresponds to the pitch class for C, 1 to C sharp and so on.
    pub fn get_value(&self) -> u8 {
        return self.value;
    }

    /// Returns a vector of equivalent names for this pitch class.
    pub fn get_names(&self) -> &'static [&'static str] {
        return self.names;
    }
}

impl PartialEq for PitchClass {
    fn eq(&self, other: &Self) -> bool {
        return self.value == other.value;
    }
}

/// A structure which can be used to obtain a reference to one of the twelve
/// equal temperament pitch classes.
pub struct PitchClasses;

impl PitchClasses {
    // Real pitch classes

    /// The pitch class for C.
    pub const C: PitchClass = PITCH_CLASSES[0];
    /// The pitch class for C sharp, which is equal to D flat.
    pub const C_SHARP: PitchClass = PITCH_CLASSES[1];
    /// The pitch class for D flat, which is equal to C sharp.
    pub const D_FLAT: PitchClass = PITCH_CLASSES[1];
    /// The pitch class for D.
    pub const D: PitchClass = PITCH_CLASSES[2];
    /// The pitch class for D sharp, which is equal to E flat.
    pub const D_SHARP: PitchClass = PITCH_CLASSES[3];
    /// The pitch class for E flat, which is equal to D sharp.
    pub const E_FLAT: PitchClass = PITCH_CLASSES[3];
    /// The pitch class for E.
    pub const E: PitchClass = PITCH_CLASSES[4];
    /// The pitch class for F.
    pub const F: PitchClass = PITCH_CLASSES[5];
    /// The pitch class for F sharp, which is equal to G flat.
    pub const F_SHARP: PitchClass = PITCH_CLASSES[6];
    /// The pitch class for G flat, which is equal to F sharp.
    pub const G_FLAT: PitchClass = PITCH_CLASSES[6];
    /// The pitch class for G.
    pub const G: PitchClass = PITCH_CLASSES[7];
    /// The pitch class for G sharp, which is equal to A flat.
    pub const G_SHARP: PitchClass = PITCH_CLASSES[8];
    /// The pitch class for A flat, which is equal to G sharp.
    pub const A_FLAT: PitchClass = PITCH_CLASSES[8];
    /// The pitch class for A.
    pub const A: PitchClass = PITCH_CLASSES[9];
    /// The pitch class for A sharp, which is equal to B flat.
    pub const A_SHARP: PitchClass = PITCH_CLASSES[10];
    /// The pitch class for B flat, which is equal to A sharp.
    pub const B_FLAT: PitchClass = PITCH_CLASSES[10];
    /// The pitch class for B.
    pub const B: PitchClass = PITCH_CLASSES[11];

    // Theoretical pitch classes

    /// The theoretical pitch class for B sharp, which is equal to C.
    pub const B_SHARP: PitchClass = PITCH_CLASSES[0];
    /// The theoretical pitch class for F flat, which is equal to E.
    pub const F_FLAT: PitchClass = PITCH_CLASSES[4];
    /// The theoretical pitch class for E sharp, which is equal to F.
    pub const E_SHARP: PitchClass = PITCH_CLASSES[5];
    /// The theoretical pitch class for C flat, which is equal to B.
    pub const C_FLAT: PitchClass = PITCH_CLASSES[11];
}

const PITCH_CLASSES: [PitchClass; 12] = [
    PitchClass {
        value: 0,
        names: &["C", "C♮", "B♯", "B#", "D♭♭", "Dbb"]
    },
    PitchClass {
        value: 1,
        names: &["C♯", "C#", "D♭", "Db"]
    },
    PitchClass {
        value: 2,
        names: &["D", "D♮", "C♯♯", "C##", "Cx", "E♭♭", "Ebb"]
    },
    PitchClass {
        value: 3,
        names: &["D♯", "D#", "E♭", "Eb"]
    },
    PitchClass {
        value: 4,
        names: &["E", "E♮", "F♭", "Fb", "D♯♯", "D##", "Dx"]
    },
    PitchClass {
        value: 5,
        names: &["F", "F♮", "E♯", "E#"]
    },
    PitchClass {
        value: 6,
        names: &["F♯", "F#", "G♭", "Gb"]
    },
    PitchClass {
        value: 7,
        names: &["G", "G♮", "F♯♯", "F##", "Fx", "A♭♭", "Abb"]
    },
    PitchClass {
        value: 8,
        names: &["G♯", "G#", "A♭", "Ab"]
    },
    PitchClass {
        value: 9,
        names: &["A", "A♮", "G♯♯", "G##", "Gx", "B♭♭", "Bbb"]
    },
    PitchClass {
        value: 10,
        names: &["A♯", "A#", "B♭", "Bb"]
    },
    PitchClass {
        value: 11,
        names: &["B", "B♮", "C♭", "Cb", "A♯♯", "A##", "Ax"]
    }
];