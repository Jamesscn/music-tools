#[derive(Copy, Clone, Debug)]
/// A structure used to define one of the pitch classes of the twelve tone
/// equal temperament system.
pub struct PitchClass {
    value: u8,
    name: &'static str
}

impl PitchClass {
    /// Returns a [`PitchClass`] representing a pitch class given its name.
    /// 
    /// # Parameters
    /// 
    /// - `pitch_class_name`: A [`String`] representing the name of the pitch
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
    /// ```
    pub fn from_name(pitch_class_name: String) -> Option<PitchClass> {
        for pitch_class_index in 0..12 {
            let flat_pitch_class = PITCH_CLASSES_FLATS[pitch_class_index];
            let sharp_pitch_class = PITCH_CLASSES_SHARPS[pitch_class_index];
            if flat_pitch_class.get_name() == pitch_class_name {
                return Some(flat_pitch_class);
            }
            if sharp_pitch_class.get_name() == pitch_class_name {
                return Some(sharp_pitch_class);
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
    /// let g_flat = PitchClass::from_value(6, true);
    /// let f_sharp = PitchClass::from_value(6, false);
    /// ```
    pub fn from_value(value: u8, prefer_flats: bool) -> Option<PitchClass> {
        let index = value as usize;
        if index < 12 {
            if prefer_flats {
                return Some(PITCH_CLASSES_FLATS[index]);
            } else {
                return Some(PITCH_CLASSES_SHARPS[index]);
            }
        }
        return None
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
    /// let c = PitchClasses::A.get_offset(2, false);
    /// let f = PitchClasses::A.get_offset(-2, false);
    /// ```
    pub fn get_offset(&self, offset: i8, prefer_flats: bool) -> PitchClass {
        if prefer_flats {
            return PITCH_CLASSES_FLATS[((self.value as i8 + offset) % 12) as usize];
        } else {
            return PITCH_CLASSES_SHARPS[((self.value as i8 + offset) % 12) as usize];
        }
    }

    /// Obtains a numeric value from 0 to 11 representing the pitch class,
    /// where 0 corresponds to the pitch class for C, 1 to C# and so on.
    pub fn get_value(&self) -> u8 {
        return self.value;
    }

    /// Returns the name of this pitch class.
    pub fn get_name(&self) -> &'static str {
        return self.name;
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
    /// The pitch class for C.
    pub const C: PitchClass = PitchClass {
        value: 0,
        name: "C"
    };
    /// The pitch class for C sharp, which is equal to D flat in this library.
    pub const C_SHARP: PitchClass = PitchClass {
        value: 1,
        name: "C#"
    };
    /// The pitch class for D flat, which is equal to C sharp in this library.
    pub const D_FLAT: PitchClass = PitchClass {
        value: 1,
        name: "Db"
    };
    /// The pitch class for D.
    pub const D: PitchClass = PitchClass {
        value: 2,
        name: "D"
    };
    /// The pitch class for D sharp, which is equal to E flat in this library.
    pub const D_SHARP: PitchClass = PitchClass {
        value: 3,
        name: "D#"
    };
    /// The pitch class for E flat, which is equal to D sharp in this library.
    pub const E_FLAT: PitchClass = PitchClass {
        value: 3,
        name: "Eb"
    };
    /// The pitch class for E.
    pub const E: PitchClass = PitchClass {
        value: 4,
        name: "E"
    };
    /// The pitch class for F.
    pub const F: PitchClass = PitchClass {
        value: 5,
        name: "F"
    };
    /// The pitch class for F sharp, which is equal to G flat in this library.
    pub const F_SHARP: PitchClass = PitchClass {
        value: 6,
        name: "F#"
    };
    /// The pitch class for G flat, which is equal to F sharp in this library.
    pub const G_FLAT: PitchClass = PitchClass {
        value: 6,
        name: "Gb"
    };
    /// The pitch class for G.
    pub const G: PitchClass = PitchClass {
        value: 7,
        name: "G"
    };
    /// The pitch class for G sharp, which is equal to A flat in this library.
    pub const G_SHARP: PitchClass = PitchClass {
        value: 8,
        name: "G#"
    };
    /// The pitch class for A flat, which is equal to G sharp in this library.
    pub const A_FLAT: PitchClass = PitchClass {
        value: 8,
        name: "Ab"
    };
    /// The pitch class for A.
    pub const A: PitchClass = PitchClass {
        value: 9,
        name: "A"
    };
    /// The pitch class for A sharp, which is equal to B flat in this library.
    pub const A_SHARP: PitchClass = PitchClass {
        value: 10,
        name: "A#"
    };
    /// The pitch class for B flat, which is equal to A sharp in this library.
    pub const B_FLAT: PitchClass = PitchClass {
        value: 10,
        name: "Bb"
    };
    /// The pitch class for B.
    pub const B: PitchClass = PitchClass {
        value: 11,
        name: "B"
    };
}

const PITCH_CLASSES_FLATS: [PitchClass; 12] = [
    PitchClasses::C,
    PitchClasses::D_FLAT,
    PitchClasses::D,
    PitchClasses::E_FLAT,
    PitchClasses::E,
    PitchClasses::F,
    PitchClasses::G_FLAT,
    PitchClasses::G,
    PitchClasses::A_FLAT,
    PitchClasses::A,
    PitchClasses::B_FLAT,
    PitchClasses::B
];

const PITCH_CLASSES_SHARPS: [PitchClass; 12] = [
    PitchClasses::C,
    PitchClasses::C_SHARP,
    PitchClasses::D,
    PitchClasses::D_SHARP,
    PitchClasses::E,
    PitchClasses::F,
    PitchClasses::F_SHARP,
    PitchClasses::G,
    PitchClasses::G_SHARP,
    PitchClasses::A,
    PitchClasses::A_SHARP,
    PitchClasses::B
];