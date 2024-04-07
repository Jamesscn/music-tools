use crate::chord::{Chord, NoteChord};
use crate::common::{result_from_iterator, InputError};
use crate::interval::Interval;
use crate::note::Note;
use crate::pitchclass::{PitchClass, TwelveTone};
use lazy_static::lazy_static;
use std::fmt;
use std::hash::Hash;

//TODO: add assumptions (0 and 12 always stored in semitones)

/// A structure used to represent a scale of notes, or a major or minor pentatonic variation of a
/// scale in the twelve tone system.
#[derive(Clone, Debug, Eq)]
pub struct Scale {
    semitones: Vec<usize>,
    diatonic_chords: Vec<String>,
    name: String,
}

impl Scale {
    pub fn new(
        semitones: &[impl Into<usize> + Clone],
        diatonic_chords: &[impl Into<String> + Clone],
        name: impl Into<String>,
    ) -> Scale {
        let mut semitones: Vec<usize> = semitones
            .iter()
            .map(|semitone| semitone.clone().into())
            .collect();
        semitones.sort();
        Scale {
            semitones,
            diatonic_chords: diatonic_chords
                .iter()
                .map(|chord| chord.clone().into())
                .collect(),
            name: name.into(),
        }
    }

    pub fn from_string(string: &str) -> Result<Self, InputError> {
        todo!();
    }

    /// Returns true if the scale is diatonic or heptatonic (has 7 notes), or false if otherwise.
    pub fn is_diatonic(&self) -> bool {
        self.semitones.len() == 8
    }

    /// Returns true if the scale is pentatonic (has 5 notes), or false if otherwise.
    pub fn is_pentatonic(&self) -> bool {
        self.semitones.len() == 6
    }

    /// Returns a [`Result`] which can contain a [`Vec<Chord>`] consisting of the seven diatonic
    /// chords of the current scale, given the pitch class of the tonic and optionally the octave of
    /// each of these chords, or an [`InputError`] if the current scale is not diatonic.
    ///
    /// # Parameters
    ///
    /// - `tonic`: A [`PitchClass`] representing the pitch class of the tonic which will be offset
    ///   by the numeral.
    /// - `octave`: An [`Option<i8>`] which can be an integer representing the octave of the first
    ///   diatonic chord, or [`None`] if the chords should not have any octave.
    /// - `with_seventh`: A boolean which if set to true ensures that the chords that are returned
    ///   contain the corresponding seventh intervals for the mode or scale, or if set to false
    ///   ensures that the chords that are returns are only triads.
    ///
    /// # Examples
    ///
    /// The following example shows how one can obtain the diatonic chords with sevenths for the G
    /// locrian scale, starting at the fifth octave.
    ///
    /// ```rust
    /// use music_tools::scale::Scale;
    /// use music_tools::chord::Chord;
    /// use music_tools::pitchclass::PitchClass;
    /// use music_tools::common::{ScaleType, PentatonicType, TriadQuality};
    ///
    /// let locrian = Scale::try_new(ScaleType::Locrian, PentatonicType::None).unwrap();
    /// let g_locrian_chords = locrian.get_diatonic_chords(PitchClass::G, Some(5), false).unwrap();
    /// assert_eq!(
    ///     Chord::from_triad(TriadQuality::Diminished, Some(PitchClass::G), Some(5)),
    ///     g_locrian_chords[0]
    /// );
    /// assert_eq!(
    ///     Chord::from_triad(TriadQuality::Major, Some(PitchClass::AFlat), Some(5)),
    ///     g_locrian_chords[1]
    /// );
    /// assert_eq!(
    ///     Chord::from_triad(TriadQuality::Minor, Some(PitchClass::BFlat), Some(5)),
    ///     g_locrian_chords[2]
    /// );
    /// assert_eq!(
    ///     Chord::from_triad(TriadQuality::Minor, Some(PitchClass::C), Some(6)),
    ///     g_locrian_chords[3]
    /// );
    /// assert_eq!(
    ///     Chord::from_triad(TriadQuality::Major, Some(PitchClass::DFlat), Some(6)),
    ///     g_locrian_chords[4]
    /// );
    /// assert_eq!(
    ///     Chord::from_triad(TriadQuality::Major, Some(PitchClass::EFlat), Some(6)),
    ///     g_locrian_chords[5]
    /// );
    /// assert_eq!(
    ///     Chord::from_triad(TriadQuality::Minor, Some(PitchClass::F), Some(6)),
    ///     g_locrian_chords[6]
    /// );
    /// ```
    pub fn get_diatonic_chords(
        &self,
        base_note: Note<TwelveTone>,
    ) -> Result<Vec<NoteChord<TwelveTone>>, InputError> {
        if self.diatonic_chords.is_empty() {
            return Err(InputError {
                message: String::from(
                    "attempted to obtain diatonic chords from a scale which does not have any",
                ),
            });
        }
        result_from_iterator(
            self.diatonic_chords.iter(),
            |numeral| Chord::from_numeral(numeral, base_note),
            |error| error,
        )
    }

    pub fn get_pentatonic_major(&self) -> Result<Self, InputError> {
        if !self.is_diatonic() {
            return Err(InputError {
                message: String::from("cannot obtain a pentatonic scale from a non diatonic scale"),
            });
        }
        let mut pentatonic_major_semitones = self.to_semitones();
        pentatonic_major_semitones.remove(6);
        pentatonic_major_semitones.remove(3);
        Ok(Scale {
            semitones: pentatonic_major_semitones,
            diatonic_chords: self.diatonic_chords.clone(),
            name: format!("{} pentatonic major", self.name),
        })
    }

    pub fn get_pentatonic_minor(&self) -> Result<Self, InputError> {
        if !self.is_diatonic() {
            return Err(InputError {
                message: String::from("cannot obtain a pentatonic scale from a non diatonic scale"),
            });
        }
        let mut pentatonic_minor_semitones = self.to_semitones();
        pentatonic_minor_semitones.remove(5);
        pentatonic_minor_semitones.remove(1);
        Ok(Scale {
            semitones: pentatonic_minor_semitones,
            diatonic_chords: self.diatonic_chords.clone(),
            name: format!("{} pentatonic minor", self.name),
        })
    }

    pub fn to_semitones(&self) -> Vec<usize> {
        self.semitones.clone()
    }

    pub fn to_intervals(&self) -> Result<Vec<Interval>, InputError> {
        result_from_iterator(
            self.semitones.windows(2),
            |window: &[usize]| Interval::from_semitones(window[1] - window[0]),
            |error| error,
        )
    }

    pub fn to_notes<PitchClassType: PitchClass>(
        &self,
        base_note: Note<PitchClassType>,
    ) -> Vec<Note<PitchClassType>> {
        self.semitones
            .iter()
            .map(|semitone| base_note.offset(*semitone as isize))
            .collect()
    }
}

impl Default for Scale {
    fn default() -> Self {
        MAJOR.clone()
    }
}

impl PartialEq for Scale {
    fn eq(&self, other: &Self) -> bool {
        self.semitones == other.semitones
    }
}

impl Hash for Scale {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.semitones.hash(state);
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} scale", self.name)
    }
}

impl TryFrom<&str> for Scale {
    type Error = InputError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_string(value)
    }
}

impl TryFrom<String> for Scale {
    type Error = InputError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_string(&value)
    }
}

impl TryFrom<Scale> for Vec<Interval> {
    type Error = InputError;

    fn try_from(value: Scale) -> Result<Self, Self::Error> {
        value.to_intervals()
    }
}

impl From<Scale> for String {
    fn from(value: Scale) -> Self {
        value.to_string()
    }
}

impl From<Scale> for Vec<usize> {
    fn from(value: Scale) -> Self {
        value.to_semitones()
    }
}

lazy_static! {
    /// The major scale, which is the same as the Ionian mode.
    pub static ref MAJOR: Scale = Scale {
        semitones: vec![0, 2, 4, 5, 7, 9, 11, 12],
        diatonic_chords: vec![
            "Imaj7".to_string(),
            "ii7".to_string(),
            "iii7".to_string(),
            "IVmaj7".to_string(),
            "V7".to_string(),
            "vi7".to_string(),
            "vii°7".to_string(),
        ],
        name: "Major".to_string(),
    };
    /// The scale of the Ionian mode, which is the first mode and is the same as the major scale.
    pub static ref IONIAN: Scale = Scale {
        semitones: vec![0, 2, 4, 5, 7, 9, 11, 12],
        diatonic_chords: vec![
            "Imaj7".to_string(),
            "ii7".to_string(),
            "iii7".to_string(),
            "IVmaj7".to_string(),
            "V7".to_string(),
            "vi7".to_string(),
            "vii°7".to_string(),
        ],
        name: "Ionian".to_string(),
    };
    /// The scale of the Dorian mode, which is the second mode and is equal to natural minor scale
    /// with a major sixth instead of a minor sixth.
    pub static ref DORIAN: Scale = Scale {
        semitones: vec![0, 2, 3, 5, 7, 9, 10, 12],
        diatonic_chords: vec![
            "i7".to_string(),
            "ii7".to_string(),
            "bIIImaj7".to_string(),
            "IV7".to_string(),
            "v7".to_string(),
            "vi°7".to_string(),
            "bVIImaj7".to_string(),
        ],
        name: "Dorian".to_string(),
    };
    /// The scale of the Phrygian mode, which is the third mode and is equal to the natural minor
    /// scale with a minor second instead of a major second.
    pub static ref PHRYGIAN: Scale = Scale {
        semitones: vec![0, 1, 3, 5, 7, 8, 10, 12],
        diatonic_chords: vec![
            "i7".to_string(),
            "bIImaj7".to_string(),
            "bIII7".to_string(),
            "iv7".to_string(),
            "v°7".to_string(),
            "bVImaj7".to_string(),
            "bvii7".to_string(),
        ],
        name: "Phrygian".to_string(),
    };
    /// The scale of the Lydian mode, which is the fourth mode and is equal to the major scale with
    /// an augmented fourth instead of a perfect fourth.
    pub static ref LYDIAN: Scale = Scale {
        semitones: vec![0, 2, 4, 6, 7, 9, 11, 12],
        diatonic_chords: vec![
            "Imaj7".to_string(),
            "II7".to_string(),
            "iii7".to_string(),
            "#iv°7".to_string(),
            "Vmaj7".to_string(),
            "vi7".to_string(),
            "vii7".to_string(),
        ],
        name: "Lydian".to_string(),
    };
    /// The scale of the Mixolydian mode, which is the fifth mode and is equal to the major scale
    /// with a minor seventh instead of a major seventh.
    pub static ref MIXOLYDIAN: Scale = Scale {
        semitones: vec![0, 2, 4, 5, 7, 9, 10, 12],
        diatonic_chords: vec![
            "I7".to_string(),
            "ii7".to_string(),
            "iii°7".to_string(),
            "IVmaj7".to_string(),
            "v7".to_string(),
            "vi7".to_string(),
            "bVIImaj7".to_string(),
        ],
        name: "Mixolydian".to_string(),
    };
    /// The modern minor scale, which differs from the natural minor scale and the Aeolian mode
    /// only in that it's fifth diatonic chord is major instead of minor.
    pub static ref MINOR: Scale = Scale {
        semitones: vec![0, 2, 3, 5, 7, 8, 10, 12],
        diatonic_chords: vec![
            "i7".to_string(),
            "ii°7".to_string(),
            "bIIImaj7".to_string(),
            "iv7".to_string(),
            "Vmaj7".to_string(),
            "bVImaj7".to_string(),
            "bVII7".to_string(),
        ],
        name: "Minor".to_string(),
    };
    /// The natural minor scale, which is the same as the Aeolian mode.
    pub static ref NATURAL_MINOR: Scale = Scale {
        semitones: vec![0, 2, 3, 5, 7, 8, 10, 12],
        diatonic_chords: vec![
            "i7".to_string(),
            "ii°7".to_string(),
            "bIIImaj7".to_string(),
            "iv7".to_string(),
            "v7".to_string(),
            "bVImaj7".to_string(),
            "bVII7".to_string(),
        ],
        name: "Natural minor".to_string(),
    };
    /// The descending melodic minor scale, which is the same as the natural minor scale but is
    /// intended to be used when playing the melodic minor scale in a descending manner.
    pub static ref DESCENDING_MELODIC_MINOR: Scale = Scale {
        semitones: vec![0, 2, 3, 5, 7, 8, 10, 12],
        diatonic_chords: vec![],
        name: "Descending melodic minor".to_string(),
    };
    /// The scale of the Aeolian mode, which is the sixth mode and is the same as the natural minor
    /// scale.
    pub static ref AEOLIAN: Scale = Scale {
        semitones: vec![0, 2, 3, 5, 7, 8, 10, 12],
        diatonic_chords: vec![
            "i7".to_string(),
            "ii°7".to_string(),
            "bIIImaj7".to_string(),
            "iv7".to_string(),
            "v7".to_string(),
            "bVImaj7".to_string(),
            "bVII7".to_string(),
        ],
        name: "Aeolian".to_string(),
    };
    /// The scale of the Locrian mode, which is the seventh mode and is equal to the natural minor
    /// scale with a minor second instead of a major second and a diminished fifth instead of a
    /// perfect fifth.
    pub static ref LOCRIAN: Scale = Scale {
        semitones: vec![0, 1, 3, 5, 6, 8, 10, 12],
        diatonic_chords: vec![
            "i°7".to_string(),
            "bIImaj7".to_string(),
            "biii7".to_string(),
            "iv7".to_string(),
            "bVmaj7".to_string(),
            "bVI7".to_string(),
            "bvii7".to_string(),
        ],
        name: "Locrian".to_string(),
    };
    /// The harmonic minor scale, which is equal to the natural minor scale with a major seventh
    /// instead of a minor seventh.
    pub static ref HARMONIC_MINOR: Scale = Scale {
        semitones: vec![0, 2, 3, 5, 7, 8, 11, 12],
        diatonic_chords: vec![],
        name: "Harmonic minor".to_string(),
    };
    /// The Aeolian ♯7 scale, which is the same as the harmonic minor scale.
    pub static ref AEOLIAN_SHARP_SEVEN: Scale = Scale {
        semitones: vec![0, 2, 3, 5, 7, 8, 11, 12],
        diatonic_chords: vec![],
        name: "Aeolian ♯7".to_string(),
    };
    /// The Locrian ♮6 scale, which is the second mode of the harmonic minor scale and the same as
    /// the Locrian scale with a natural sixth.
    pub static ref LOCRIAN_NATURAL_SIX: Scale = Scale {
        semitones: vec![0, 1, 3, 5, 6, 9, 10, 12],
        diatonic_chords: vec![],
        name: "Locrian ♮6".to_string(),
    };
    /// The Ionian ♯5 scale, which is the third mode of the harmonic minor scale and the same as
    /// the Ionian scale with a sharp fifth.
    pub static ref IONIAN_SHARP_FIVE: Scale = Scale {
        semitones: vec![0, 2, 4, 5, 8, 9, 11, 12],
        diatonic_chords: vec![],
        name: "Ionian ♯5".to_string(),
    };
    /// The Dorian ♯4 scale, which is the fourth mode of the harmonic minor scale and the same as
    /// the Dorian scale with a sharp fourth.
    pub static ref DORIAN_SHARP_FOUR: Scale = Scale {
        semitones: vec![0, 2, 3, 6, 7, 9, 10, 12],
        diatonic_chords: vec![],
        name: "Dorian ♯4".to_string(),
    };
    /// The Romanian minor scale, which is the same as the Dorian ♯4 scale.
    pub static ref ROMANIAN_MINOR: Scale = Scale {
        semitones: vec![0, 2, 3, 6, 7, 9, 10, 12],
        diatonic_chords: vec![],
        name: "Romanian minor".to_string(),
    };
    /// The Ukranian dorian scale, which is the same as the Dorian ♯4 scale.
    pub static ref UKRANIAN_DORIAN: Scale = Scale {
        semitones: vec![0, 2, 3, 6, 7, 9, 10, 12],
        diatonic_chords: vec![],
        name: "Ukranian dorian".to_string(),
    };
    /// The Phrygian dominant scale, which is the fifth mode of the harmonic minor scale and is the
    /// equal to the Phrygian scale with a major third instead of a minor third.
    pub static ref PHRYGIAN_DOMINANT: Scale = Scale {
        semitones: vec![0, 1, 4, 5, 7, 8, 10, 12],
        diatonic_chords: vec![],
        name: "Phrygian dominant".to_string(),
    };
    /// The Lydian ♯2 scale, which is the sixth mode of the harmonic minor scale and the same as
    /// the Lydian scale with a sharp second.
    pub static ref LYDIAN_SHARP_TWO: Scale = Scale {
        semitones: vec![0, 3, 4, 6, 7, 9, 11, 12],
        diatonic_chords: vec![],
        name: "Lydian ♯2".to_string(),
    };
    /// The altered diminished scale, which is the seventh mode of the harmonic minor scale and the
    /// same as the Locrian scale with a flat fourth and a double flat seventh.
    pub static ref ALTERED_DIMINISHED: Scale = Scale {
        semitones: vec![0, 1, 3, 4, 6, 8, 9, 12],
        diatonic_chords: vec![],
        name: "Altered diminished".to_string(),
    };
    /// The Super locrian ♭♭7 scale, which is the same as the altered diminished scale.
    pub static ref SUPER_LOCRIAN_DOUBLE_FLAT_SEVEN: Scale = Scale {
        semitones: vec![0, 1, 3, 4, 6, 8, 9, 12],
        diatonic_chords: vec![],
        name: "Super locrian ♭♭7".to_string(),
    };
    /// The ascending melodic minor scale, which is equal to the natural minor scale with a major
    /// sixth and major seventh, and is intended to be used when playing the melodic minor scale in
    /// an ascending manner. Also known as just the melodic minor scale.
    pub static ref ASCENDING_MELODIC_MINOR: Scale = Scale {
        semitones: vec![0, 2, 3, 5, 7, 9, 11, 12],
        diatonic_chords: vec![],
        name: "Ascending melodic minor".to_string(),
    };
    /// The melodic minor scale, which is the same as the ascending melodic minor scale.
    pub static ref MELODIC_MINOR: Scale = Scale {
        semitones: vec![0, 2, 3, 5, 7, 9, 11, 12],
        diatonic_chords: vec![],
        name: "Melodic minor".to_string(),
    };
    /// The jazz minor scale, which is the same as the ascending melodic minor scale.
    pub static ref JAZZ_MINOR: Scale = Scale {
        semitones: vec![0, 2, 3, 5, 7, 9, 11, 12],
        diatonic_chords: vec![],
        name: "Jazz minor".to_string(),
    };
    /// The Dorian ♭2 scale, which is the second mode of the melodic minor scale and the same as
    /// the Dorian scale but with a flat second.
    pub static ref DORIAN_FLAT_TWO: Scale = Scale {
        semitones: vec![0, 1, 3, 5, 7, 9, 10, 12],
        diatonic_chords: vec![],
        name: "Dorian ♭2".to_string(),
    };
    /// The Phrygian ♯6 scale, which is the same as the Dorian ♭2 scale.
    pub static ref PHRYGIAN_SHARP_SIX: Scale = Scale {
        semitones: vec![0, 1, 3, 5, 7, 9, 10, 12],
        diatonic_chords: vec![],
        name: "Phrygian ♯6".to_string(),
    };
    /// The Lyidan augmented scale, which is the third mode of the melodic minor scale and the
    /// same as the major scale with a raised fourth and fifth.
    pub static ref LYDIAN_AUGMENTED: Scale = Scale {
        semitones: vec![0, 2, 4, 6, 8, 9, 11, 12],
        diatonic_chords: vec![],
        name: "Lyidan augmented".to_string(),
    };
    /// The Lydian dominant scale, which is the fourth mode of the melodic minor scale and the same
    /// as the mixolydian scale with a sharp fourth.
    pub static ref LYDIAN_DOMINANT: Scale = Scale {
        semitones: vec![0, 2, 4, 6, 7, 9, 10, 12],
        diatonic_chords: vec![],
        name: "Lydian dominant".to_string(),
    };
    /// The overtone scale, which is the same as the Lydian dominant scale.
    pub static ref OVERTONE: Scale = Scale {
        semitones: vec![0, 2, 4, 6, 7, 9, 10, 12],
        diatonic_chords: vec![],
        name: "Overtone".to_string(),
    };
    /// The acoustic scale, which is the same as the Lydian dominant scale.
    pub static ref ACOUSTIC: Scale = Scale {
        semitones: vec![0, 2, 4, 6, 7, 9, 10, 12],
        diatonic_chords: vec![],
        name: "Acoustic".to_string(),
    };
    /// The Mixolydian ♯4 scale, which is the same as the Lydian dominant scale.
    pub static ref MIXOLYDIAN_SHARP_FOUR: Scale = Scale {
        semitones: vec![0, 2, 4, 6, 7, 9, 10, 12],
        diatonic_chords: vec![],
        name: "Mixolydian ♯4".to_string(),
    };
    /// The Mixolydian ♭6 scale, which is the fifth mode of the melodic minor scale and the same as
    /// the major scale with a flat sixth and seventh.
    pub static ref MIXOLYDIAN_FLAT_SIX: Scale = Scale {
        semitones: vec![0, 2, 4, 5, 7, 8, 10, 12],
        diatonic_chords: vec![],
        name: "Mixolydian ♭6".to_string(),
    };
    /// The Aeolian dominant scale, which is the same as the Mixolydian ♭6 scale.
    pub static ref AEOLIAN_DOMINANT: Scale = Scale {
        semitones: vec![0, 2, 4, 5, 7, 8, 10, 12],
        diatonic_chords: vec![],
        name: "Aeolian dominant".to_string(),
    };
    /// The descending melodic major scale, which is the same as the Mixolydian ♭6 scale.
    pub static ref DESCENDING_MELODIC_MAJOR: Scale = Scale {
        semitones: vec![0, 2, 4, 5, 7, 8, 10, 12],
        diatonic_chords: vec![],
        name: "Descending melodic minor".to_string(),
    };
    /// The hindu scale, which is the same as the Mixolydian ♭6 scale.
    pub static ref HINDU: Scale = Scale {
        semitones: vec![0, 2, 4, 5, 7, 8, 10, 12],
        diatonic_chords: vec![],
        name: "Hindu".to_string(),
    };
    /// The Locrian ♯2 scale, which is the sixth mode of the melodic minor scale and the same as
    /// the locrian scale with a natural second.
    pub static ref LOCRIAN_SHARP_TWO: Scale = Scale {
        semitones: vec![0, 2, 3, 5, 6, 8, 10, 12],
        diatonic_chords: vec![],
        name: "Locrian ♯2".to_string(),
    };
    /// The Aeolian ♭5 scale, which is the same as the Locrian ♯2 scale.
    pub static ref AEOLIAN_FLAT_FIVE: Scale = Scale {
        semitones: vec![0, 2, 3, 5, 6, 8, 10, 12],
        diatonic_chords: vec![],
        name: "Aeolian ♭5".to_string(),
    };
    /// The half diminished scale, which is the same as the Locrian ♯2 scale.
    pub static ref HALF_DIMINISHED: Scale = Scale {
        semitones: vec![0, 2, 3, 5, 6, 8, 10, 12],
        diatonic_chords: vec![],
        name: "Half diminished".to_string(),
    };
    /// The altered scale, which is the seventh mode of the melodic minor scale and the same as the
    /// major scale with all four altered extensions of the major mode.
    pub static ref ALTERED: Scale = Scale {
        semitones: vec![0, 1, 3, 4, 6, 8, 10, 12],
        diatonic_chords: vec![],
        name: "Altered".to_string(),
    };
    /// The altered dominant scale, which is the same as the altered scale.
    pub static ref ALTERED_DOMINANT: Scale = Scale {
        semitones: vec![0, 1, 3, 4, 6, 8, 10, 12],
        diatonic_chords: vec![],
        name: "Altered dominant".to_string(),
    };
    /// The super locrian scale, which is the same as the altered scale.
    pub static ref SUPER_LOCRIAN: Scale = Scale {
        semitones: vec![0, 1, 3, 4, 6, 8, 10, 12],
        diatonic_chords: vec![],
        name: "Super locrian".to_string(),
    };
    /// The diminished scale, which contains an alternating pattern of whole tones followed by
    /// semitones, starting with a whole tone.
    pub static ref DIMINISHED: Scale = Scale {
        semitones: vec![0, 2, 3, 5, 6, 8, 9, 11, 12],
        diatonic_chords: vec![],
        name: "Diminished".to_string(),
    };
    /// The dominant diminished scale, which contains an alternating pattern of semitones followed
    /// by whole tones, starting with a semitone.
    pub static ref DOMINANT_DIMINISHED: Scale = Scale {
        semitones: vec![0, 1, 3, 4, 6, 7, 9, 10, 12],
        diatonic_chords: vec![],
        name: "Dominant diminished".to_string(),
    };
    /// A nonatonic blues scale, which is derived from the major scale with an added flat third and
    /// an added flat seventh of the key.
    pub static ref NONATONIC_BLUES: Scale = Scale {
        semitones: vec![0, 2, 3, 4, 5, 7, 9, 10, 11, 12],
        diatonic_chords: vec![],
        name: "Nonatonic blues".to_string(),
    };
    /// The major blues scale, which is a hexatonic scale derived from the major pentatonic scale
    /// with an added flat third of the key.
    pub static ref MAJOR_BLUES: Scale = Scale {
        semitones: vec![0, 2, 3, 4, 7, 9, 12],
        diatonic_chords: vec![],
        name: "Major blues".to_string(),
    };
    /// The minor blues scale, which is a hexatonic scale derived from the minor pentatonic scale
    /// with an added flat fifth of the key.
    pub static ref MINOR_BLUES: Scale = Scale {
        semitones: vec![0, 3, 5, 6, 7, 10, 12],
        diatonic_chords: vec![],
        name: "Minor blues".to_string(),
    };
    /// The whole tone scale, which is a hexatonic scale where each tone or pitch class is
    /// separated by a whole note or two semitones.
    pub static ref WHOLE: Scale = Scale {
        semitones: vec![0, 2, 4, 6, 8, 10, 12],
        diatonic_chords: vec![],
        name: "Whole".to_string(),
    };
    /// The chromatic scale, which consists of all twelve pitch classes separated by a semitone.
    pub static ref CHROMATIC: Scale = Scale {
        semitones: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        diatonic_chords: vec![],
        name: "Chromatic".to_string(),
    };
}
