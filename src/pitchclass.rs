use std::str::FromStr;

use crate::{
    chord::Chord,
    common::{IncompleteChordError, InputError},
};

/// A structure used to define one of the pitch classes of the twelve tone equal temperament system.
#[derive(Copy, Clone, Debug)]
pub struct PitchClass {
    reference: &'static StaticPitchClass,
}

impl PitchClass {
    /// Returns a [`Result`] which can contain a [`PitchClass`] given its value from 0 to 11, where
    /// 0 represents C, 1 represents D flat and so on. If the index is greater than 11 then an
    /// [`InputError`] is returned.
    ///
    /// # Parameters
    ///
    /// - `value`: An integer from 0 to 11 representing the [`PitchClass`] to return.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use music_tools::pitchclass::PitchClass;
    ///
    /// let g_flat = PitchClass::try_from(6).unwrap();
    /// ```
    pub fn try_from(value: impl Into<u64>) -> Result<PitchClass, InputError> {
        let numeric_value = value.into();
        let index = numeric_value as usize;
        if index < 12 {
            return Ok(PitchClass {
                reference: &PITCH_CLASSES[index],
            });
        }
        Err(InputError {
            message: "the value provided must be an integer between 0 and 11",
        })
    }
}

impl PitchClass {
    /// Returns the [`PitchClass`] that is a certain offset away from the current one.
    ///
    /// # Parameters
    ///
    /// - `offset`: The offset of the pitch class to return with respect to the current pitch class.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use music_tools::pitchclass::PitchClasses;
    ///
    /// let c = PitchClasses::A.get_offset(2);
    /// let f = PitchClasses::A.get_offset(-2);
    /// ```
    pub fn get_offset(&self, offset: i8) -> Self {
        Self {
            reference: &PITCH_CLASSES
                [(self.reference.value as i8 + (offset % 12)).rem_euclid(12) as usize],
        }
    }

    /// Obtains a numeric value from 0 to 11 representing the pitch class, where 0 corresponds to
    /// the pitch class for C, 1 to C sharp and so on.
    pub fn get_value(&self) -> u8 {
        self.reference.value
    }

    /// Returns a vector of equivalent names for this pitch class.
    pub fn get_names(&self) -> &'static [&'static str] {
        self.reference.names
    }
}

#[derive(Copy, Clone, Debug)]
struct StaticPitchClass {
    value: u8,
    names: &'static [&'static str],
}

trait TryFromStaticPitchClass {
    fn try_from(value: impl Into<u64>) -> Result<&'static Self, InputError>;
}

impl Default for PitchClass {
    fn default() -> Self {
        PitchClasses::C
    }
}

impl PartialEq for PitchClass {
    fn eq(&self, other: &Self) -> bool {
        self.reference.value == other.reference.value
    }
}

impl Eq for PitchClass {}

impl FromStr for PitchClass {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for pitch_class in &PITCH_CLASSES {
            for current_name in pitch_class.names {
                if *current_name == s {
                    return Ok(PitchClass {
                        reference: pitch_class,
                    });
                }
            }
        }
        Err(InputError {
            message: "string does not conform to expected pitch class format",
        })
    }
}

impl TryFrom<Chord> for Vec<PitchClass> {
    type Error = IncompleteChordError;

    fn try_from(value: Chord) -> Result<Self, Self::Error> {
        match value.get_tonic() {
            Some(tonic) => Ok(value
                .get_intervals()
                .iter()
                .map(|interval| tonic.get_offset((interval.get_value() % 12) as i8))
                .collect()),
            None => Err(IncompleteChordError {
                needs_tonic: true,
                needs_octave: false,
                has_tonic: value.get_tonic().is_some(),
                has_octave: value.get_octave().is_some(),
            }),
        }
    }
}

#[non_exhaustive]
/// A structure which can be used to obtain a reference to one of the twelve equal temperament pitch
/// classes.
pub struct PitchClasses;

impl PitchClasses {
    // Real pitch classes

    /// The pitch class for C.
    pub const C: PitchClass = PitchClass {
        reference: &PITCH_CLASSES[0],
    };
    /// The pitch class for C sharp, which is equal to D flat.
    pub const C_SHARP: PitchClass = PitchClass {
        reference: &PITCH_CLASSES[1],
    };
    /// The pitch class for D flat, which is equal to C sharp.
    pub const D_FLAT: PitchClass = PitchClass {
        reference: &PITCH_CLASSES[1],
    };
    /// The pitch class for D.
    pub const D: PitchClass = PitchClass {
        reference: &PITCH_CLASSES[2],
    };
    /// The pitch class for D sharp, which is equal to E flat.
    pub const D_SHARP: PitchClass = PitchClass {
        reference: &PITCH_CLASSES[3],
    };
    /// The pitch class for E flat, which is equal to D sharp.
    pub const E_FLAT: PitchClass = PitchClass {
        reference: &PITCH_CLASSES[3],
    };
    /// The pitch class for E.
    pub const E: PitchClass = PitchClass {
        reference: &PITCH_CLASSES[4],
    };
    /// The pitch class for F.
    pub const F: PitchClass = PitchClass {
        reference: &PITCH_CLASSES[5],
    };
    /// The pitch class for F sharp, which is equal to G flat.
    pub const F_SHARP: PitchClass = PitchClass {
        reference: &PITCH_CLASSES[6],
    };
    /// The pitch class for G flat, which is equal to F sharp.
    pub const G_FLAT: PitchClass = PitchClass {
        reference: &PITCH_CLASSES[6],
    };
    /// The pitch class for G.
    pub const G: PitchClass = PitchClass {
        reference: &PITCH_CLASSES[7],
    };
    /// The pitch class for G sharp, which is equal to A flat.
    pub const G_SHARP: PitchClass = PitchClass {
        reference: &PITCH_CLASSES[8],
    };
    /// The pitch class for A flat, which is equal to G sharp.
    pub const A_FLAT: PitchClass = PitchClass {
        reference: &PITCH_CLASSES[8],
    };
    /// The pitch class for A.
    pub const A: PitchClass = PitchClass {
        reference: &PITCH_CLASSES[9],
    };
    /// The pitch class for A sharp, which is equal to B flat.
    pub const A_SHARP: PitchClass = PitchClass {
        reference: &PITCH_CLASSES[10],
    };
    /// The pitch class for B flat, which is equal to A sharp.
    pub const B_FLAT: PitchClass = PitchClass {
        reference: &PITCH_CLASSES[10],
    };
    /// The pitch class for B.
    pub const B: PitchClass = PitchClass {
        reference: &PITCH_CLASSES[11],
    };

    // Theoretical pitch classes

    /// The theoretical pitch class for B sharp, which is equal to C.
    pub const B_SHARP: PitchClass = PitchClass {
        reference: &PITCH_CLASSES[0],
    };
    /// The theoretical pitch class for F flat, which is equal to E.
    pub const F_FLAT: PitchClass = PitchClass {
        reference: &PITCH_CLASSES[4],
    };
    /// The theoretical pitch class for E sharp, which is equal to F.
    pub const E_SHARP: PitchClass = PitchClass {
        reference: &PITCH_CLASSES[5],
    };
    /// The theoretical pitch class for C flat, which is equal to B.
    pub const C_FLAT: PitchClass = PitchClass {
        reference: &PITCH_CLASSES[11],
    };
}

const PITCH_CLASSES: [StaticPitchClass; 12] = [
    StaticPitchClass {
        value: 0,
        names: &["C", "C♮", "B♯", "B#", "D♭♭", "Dbb"],
    },
    StaticPitchClass {
        value: 1,
        names: &["C♯", "C#", "D♭", "Db"],
    },
    StaticPitchClass {
        value: 2,
        names: &["D", "D♮", "C♯♯", "C##", "Cx", "E♭♭", "Ebb"],
    },
    StaticPitchClass {
        value: 3,
        names: &["D♯", "D#", "E♭", "Eb"],
    },
    StaticPitchClass {
        value: 4,
        names: &["E", "E♮", "F♭", "Fb", "D♯♯", "D##", "Dx"],
    },
    StaticPitchClass {
        value: 5,
        names: &["F", "F♮", "E♯", "E#"],
    },
    StaticPitchClass {
        value: 6,
        names: &["F♯", "F#", "G♭", "Gb"],
    },
    StaticPitchClass {
        value: 7,
        names: &["G", "G♮", "F♯♯", "F##", "Fx", "A♭♭", "Abb"],
    },
    StaticPitchClass {
        value: 8,
        names: &["G♯", "G#", "A♭", "Ab"],
    },
    StaticPitchClass {
        value: 9,
        names: &["A", "A♮", "G♯♯", "G##", "Gx", "B♭♭", "Bbb"],
    },
    StaticPitchClass {
        value: 10,
        names: &["A♯", "A#", "B♭", "Bb"],
    },
    StaticPitchClass {
        value: 11,
        names: &["B", "B♮", "C♭", "Cb", "A♯♯", "A##", "Ax"],
    },
];
