use crate::common::{EqualTemperament, InputError};
use crate::interval::Interval;
use crate::note::Note;
use crate::pitchclass::TwelveTone;
use std::fmt;
use std::hash::Hash;

/// A structure used to represent a scale of notes, or a major or minor pentatonic variation of a
/// scale in the twelve tone system.
#[derive(Clone, Debug, Eq)]
pub struct Scale {
    intervals: &'static [usize],
    diatonic_chords: Option<[&'static str; 7]>,
    name: &'static str,
}

impl Scale {
    /// Returns true if the scale is diatonic or heptatonic (has 7 notes), or false if otherwise.
    pub fn is_diatonic(&self) -> bool {
        self.intervals.len() == 8
    }

    /// Returns true if the scale is pentatonic (has 5 notes), or false if otherwise.
    pub fn is_pentatonic(&self) -> bool {
        self.intervals.len() == 6
    }

    /*
    //TODO
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
        tonic: TwelveTone,
        octave: Option<i8>,
        with_seventh: bool,
    ) -> Result<Vec<Chord>, InputError> {
        let chords = self
            .diatonic_chords
            .ok_or(Err(InputError {
                message: String::from(
                    "cannot obtain the diatonic chords for a scale that is not diatonic",
                ),
            }))?
            .iter()
            .map(|numeral| Chord::from_numeral(numeral, tonic, octave).unwrap())
            .collect();
        Ok(chords)
    }
     */

    pub fn get_pentatonic_major<IntervalType: Interval>(
        &self,
    ) -> Result<Vec<IntervalType>, InputError> {
        if !self.is_diatonic() {
            return Err(InputError {
                message: String::from("cannot obtain a pentatonic scale from a non diatonic scale"),
            });
        }
        let mut pentatonic_major = self.get_intervals::<IntervalType>()?;
        pentatonic_major.remove(6);
        pentatonic_major.remove(3);
        Ok(pentatonic_major)
    }

    pub fn get_pentatonic_minor<IntervalType: Interval>(
        &self,
    ) -> Result<Vec<IntervalType>, InputError> {
        if !self.is_diatonic() {
            return Err(InputError {
                message: String::from("cannot obtain a pentatonic scale from a non diatonic scale"),
            });
        }
        let mut pentatonic_minor = self.get_intervals::<IntervalType>()?;
        pentatonic_minor.remove(5);
        pentatonic_minor.remove(1);
        Ok(pentatonic_minor)
    }

    /// Returns a vector with each of the intervals of the scale.
    pub fn get_intervals<IntervalType: Interval>(&self) -> Result<Vec<IntervalType>, InputError> {
        let interval_options: Vec<(usize, Option<IntervalType>)> = self
            .intervals
            .iter()
            .map(|semitones| (*semitones, IntervalType::from_semitones(*semitones)))
            .collect();
        if let Some((semitones, _)) = interval_options.iter().find(|(_, option)| option.is_none()) {
            return Err(InputError {
                message: format!("the from_semitones() function did not return an interval with a difference of {semitones} semitones"),
            });
        }
        Ok(interval_options
            .iter()
            .map(|(_, interval_option)| interval_option.clone().unwrap())
            .collect())
    }

    /*
    //TODO
    /// Converts the scale to a vector of [`Note`], given a pitch class as the tonic and the octave
    /// to place the notes of the chord over.
    ///
    /// # Parameters
    ///
    /// - `tonic`: A [`TwelveTone`] representing the pitch class of the tonic of the set of notes.
    /// - `starting_octave`: An integer representing the octave to place the tonic on.
    pub fn to_notes(
        &self,
        tonic: TwelveTone,
        starting_octave: i8,
    ) -> Vec<Note<TwelveTone, EqualTemperament>> {
        let mut chord = Chord::from(self.clone());
        chord.set_tonic(Some(tonic));
        chord.set_octave(Some(starting_octave));
        Vec::try_from(chord).unwrap()
    }

    /// Converts the scale to a vector of [`TwelveTone`], given a pitch class as the tonic.
    ///
    /// # Parameters
    ///
    /// - `tonic`: A [`TwelveTone`] representing the pitch class of the tonic of the other pitch
    ///   classes.
    pub fn to_pitch_classes(&self, tonic: TwelveTone) -> Vec<TwelveTone> {
        let mut chord = Chord::from(self.clone());
        chord.set_tonic(Some(tonic));
        Vec::try_from(chord).unwrap()
    }
     */
}

impl Default for Scale {
    fn default() -> Self {
        MAJOR
    }
}

impl PartialEq for Scale {
    fn eq(&self, other: &Self) -> bool {
        self.intervals == other.intervals
    }
}

impl Hash for Scale {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.intervals.hash(state);
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} scale", self.name)
    }
}

/// The major scale, which is the same as the Ionian mode.
pub const MAJOR: Scale = Scale {
    intervals: &[0, 2, 4, 5, 7, 9, 11, 12],
    diatonic_chords: Some(["Imaj7", "ii7", "iii7", "IVmaj7", "V7", "vi7", "vii°7"]),
    name: "Major",
};
/// The scale of the Ionian mode, which is the first mode and is the same as the major scale.
pub const IONIAN: Scale = Scale {
    intervals: &[0, 2, 4, 5, 7, 9, 11, 12],
    diatonic_chords: Some(["Imaj7", "ii7", "iii7", "IVmaj7", "V7", "vi7", "vii°7"]),
    name: "Ionian",
};
/// The scale of the Dorian mode, which is the second mode and is equal to natural minor scale
/// with a major sixth instead of a minor sixth.
pub const DORIAN: Scale = Scale {
    intervals: &[0, 2, 3, 5, 7, 9, 10, 12],
    diatonic_chords: Some(["i7", "ii7", "bIIImaj7", "IV7", "v7", "vi°7", "bVIImaj7"]),
    name: "Dorian",
};
/// The scale of the Phrygian mode, which is the third mode and is equal to the natural minor
/// scale with a minor second instead of a major second.
pub const PHRYGIAN: Scale = Scale {
    intervals: &[0, 1, 3, 5, 7, 8, 10, 12],
    diatonic_chords: Some(["i7", "bIImaj7", "bIII7", "iv7", "v°7", "bVImaj7", "bvii7"]),
    name: "Phrygian",
};
/// The scale of the Lydian mode, which is the fourth mode and is equal to the major scale with
/// an augmented fourth instead of a perfect fourth.
pub const LYDIAN: Scale = Scale {
    intervals: &[0, 2, 4, 6, 7, 9, 11, 12],
    diatonic_chords: Some(["Imaj7", "II7", "iii7", "#iv°7", "Vmaj7", "vi7", "vii7"]),
    name: "Lydian",
};
/// The scale of the Mixolydian mode, which is the fifth mode and is equal to the major scale
/// with a minor seventh instead of a major seventh.
pub const MIXOLYDIAN: Scale = Scale {
    intervals: &[0, 2, 4, 5, 7, 9, 10, 12],
    diatonic_chords: Some(["I7", "ii7", "iii°7", "IVmaj7", "v7", "vi7", "bVIImaj7"]),
    name: "Mixolydian",
};
/// The modern minor scale, which differs from the natural minor scale and the Aeolian mode
/// only in that it's fifth diatonic chord is major instead of minor.
pub const MINOR: Scale = Scale {
    intervals: &[0, 2, 3, 5, 7, 8, 10, 12],
    diatonic_chords: Some(["i7", "ii°7", "bIIImaj7", "iv7", "Vmaj7", "bVImaj7", "bVII7"]),
    name: "Minor",
};
/// The natural minor scale, which is the same as the Aeolian mode.
pub const NATURAL_MINOR: Scale = Scale {
    intervals: &[0, 2, 3, 5, 7, 8, 10, 12],
    diatonic_chords: Some(["i7", "ii°7", "bIIImaj7", "iv7", "v7", "bVImaj7", "bVII7"]),
    name: "Natural minor",
};
/// The descending melodic minor scale, which is the same as the natural minor scale but is
/// intended to be used when playing the melodic minor scale in a descending manner.
pub const DESCENDING_MELODIC_MINOR: Scale = Scale {
    intervals: &[0, 2, 3, 5, 7, 8, 10, 12],
    diatonic_chords: None,
    name: "Descending melodic minor",
};
/// The scale of the Aeolian mode, which is the sixth mode and is the same as the natural minor
/// scale.
pub const AEOLIAN: Scale = Scale {
    intervals: &[0, 2, 3, 5, 7, 8, 10, 12],
    diatonic_chords: Some(["i7", "ii°7", "bIIImaj7", "iv7", "v7", "bVImaj7", "bVII7"]),
    name: "Aeolian",
};
/// The scale of the Locrian mode, which is the seventh mode and is equal to the natural minor
/// scale with a minor second instead of a major second and a diminished fifth instead of a
/// perfect fifth.
pub const LOCRIAN: Scale = Scale {
    intervals: &[0, 1, 3, 5, 6, 8, 10, 12],
    diatonic_chords: Some(["i°7", "bIImaj7", "biii7", "iv7", "bVmaj7", "bVI7", "bvii7"]),
    name: "Locrian",
};
/// The harmonic minor scale, which is equal to the natural minor scale with a major seventh
/// instead of a minor seventh.
pub const HARMONIC_MINOR: Scale = Scale {
    intervals: &[0, 2, 3, 5, 7, 8, 11, 12],
    diatonic_chords: None,
    name: "Harmonic minor",
};
/// The Aeolian ♯7 scale, which is the same as the harmonic minor scale.
pub const AEOLIAN_SHARP_SEVEN: Scale = Scale {
    intervals: &[0, 2, 3, 5, 7, 8, 11, 12],
    diatonic_chords: None,
    name: "Aeolian ♯7",
};
/// The Locrian ♮6 scale, which is the second mode of the harmonic minor scale and the same as
/// the Locrian scale with a natural sixth.
pub const LOCRIAN_NATURAL_SIX: Scale = Scale {
    intervals: &[0, 1, 3, 5, 6, 9, 10, 12],
    diatonic_chords: None,
    name: "Locrian ♮6",
};
/// The Ionian ♯5 scale, which is the third mode of the harmonic minor scale and the same as
/// the Ionian scale with a sharp fifth.
pub const IONIAN_SHARP_FIVE: Scale = Scale {
    intervals: &[0, 2, 4, 5, 8, 9, 11, 12],
    diatonic_chords: None,
    name: "Ionian ♯5",
};
/// The Dorian ♯4 scale, which is the fourth mode of the harmonic minor scale and the same as
/// the Dorian scale with a sharp fourth.
pub const DORIAN_SHARP_FOUR: Scale = Scale {
    intervals: &[0, 2, 3, 6, 7, 9, 10, 12],
    diatonic_chords: None,
    name: "Dorian ♯4",
};
/// The Romanian minor scale, which is the same as the Dorian ♯4 scale.
pub const ROMANIAN_MINOR: Scale = Scale {
    intervals: &[0, 2, 3, 6, 7, 9, 10, 12],
    diatonic_chords: None,
    name: "Romanian minor",
};
/// The Ukranian dorian scale, which is the same as the Dorian ♯4 scale.
pub const UKRANIAN_DORIAN: Scale = Scale {
    intervals: &[0, 2, 3, 6, 7, 9, 10, 12],
    diatonic_chords: None,
    name: "Ukranian dorian",
};
/// The Phrygian dominant scale, which is the fifth mode of the harmonic minor scale and is the
/// equal to the Phrygian scale with a major third instead of a minor third.
pub const PHRYGIAN_DOMINANT: Scale = Scale {
    intervals: &[0, 1, 4, 5, 7, 8, 10, 12],
    diatonic_chords: None,
    name: "Phrygian dominant",
};
/// The Lydian ♯2 scale, which is the sixth mode of the harmonic minor scale and the same as
/// the Lydian scale with a sharp second.
pub const LYDIAN_SHARP_TWO: Scale = Scale {
    intervals: &[0, 3, 4, 6, 7, 9, 11, 12],
    diatonic_chords: None,
    name: "Lydian ♯2",
};
/// The altered diminished scale, which is the seventh mode of the harmonic minor scale and the
/// same as the Locrian scale with a flat fourth and a double flat seventh.
pub const ALTERED_DIMINISHED: Scale = Scale {
    intervals: &[0, 1, 3, 4, 6, 8, 9, 12],
    diatonic_chords: None,
    name: "Altered diminished",
};
/// The Super locrian ♭♭7 scale, which is the same as the altered diminished scale.
pub const SUPER_LOCRIAN_DOUBLE_FLAT_SEVEN: Scale = Scale {
    intervals: &[0, 1, 3, 4, 6, 8, 9, 12],
    diatonic_chords: None,
    name: "Super locrian ♭♭7",
};
/// The ascending melodic minor scale, which is equal to the natural minor scale with a major
/// sixth and major seventh, and is intended to be used when playing the melodic minor scale in
/// an ascending manner. Also known as just the melodic minor scale.
pub const ASCENDING_MELODIC_MINOR: Scale = Scale {
    intervals: &[0, 2, 3, 5, 7, 9, 11, 12],
    diatonic_chords: None,
    name: "Ascending melodic minor",
};
/// The melodic minor scale, which is the same as the ascending melodic minor scale.
pub const MELODIC_MINOR: Scale = Scale {
    intervals: &[0, 2, 3, 5, 7, 9, 11, 12],
    diatonic_chords: None,
    name: "Melodic minor",
};
/// The jazz minor scale, which is the same as the ascending melodic minor scale.
pub const JAZZ_MINOR: Scale = Scale {
    intervals: &[0, 2, 3, 5, 7, 9, 11, 12],
    diatonic_chords: None,
    name: "Jazz minor",
};
/// The Dorian ♭2 scale, which is the second mode of the melodic minor scale and the same as
/// the Dorian scale but with a flat second.
pub const DORIAN_FLAT_TWO: Scale = Scale {
    intervals: &[0, 1, 3, 5, 7, 9, 10, 12],
    diatonic_chords: None,
    name: "Dorian ♭2",
};
/// The Phrygian ♯6 scale, which is the same as the Dorian ♭2 scale.
pub const PHRYGIAN_SHARP_SIX: Scale = Scale {
    intervals: &[0, 1, 3, 5, 7, 9, 10, 12],
    diatonic_chords: None,
    name: "Phrygian ♯6",
};
/// The Lyidan augmented scale, which is the third mode of the melodic minor scale and the
/// same as the major scale with a raised fourth and fifth.
pub const LYDIAN_AUGMENTED: Scale = Scale {
    intervals: &[0, 2, 4, 6, 8, 9, 11, 12],
    diatonic_chords: None,
    name: "Lyidan augmented",
};
/// The Lydian dominant scale, which is the fourth mode of the melodic minor scale and the same
/// as the mixolydian scale with a sharp fourth.
pub const LYDIAN_DOMINANT: Scale = Scale {
    intervals: &[0, 2, 4, 6, 7, 9, 10, 12],
    diatonic_chords: None,
    name: "Lydian dominant",
};
/// The overtone scale, which is the same as the Lydian dominant scale.
pub const OVERTONE: Scale = Scale {
    intervals: &[0, 2, 4, 6, 7, 9, 10, 12],
    diatonic_chords: None,
    name: "Overtone",
};
/// The acoustic scale, which is the same as the Lydian dominant scale.
pub const ACOUSTIC: Scale = Scale {
    intervals: &[0, 2, 4, 6, 7, 9, 10, 12],
    diatonic_chords: None,
    name: "Acoustic",
};
/// The Mixolydian ♯4 scale, which is the same as the Lydian dominant scale.
pub const MIXOLYDIAN_SHARP_FOUR: Scale = Scale {
    intervals: &[0, 2, 4, 6, 7, 9, 10, 12],
    diatonic_chords: None,
    name: "Mixolydian ♯4",
};
/// The Mixolydian ♭6 scale, which is the fifth mode of the melodic minor scale and the same as
/// the major scale with a flat sixth and seventh.
pub const MIXOLYDIAN_FLAT_SIX: Scale = Scale {
    intervals: &[0, 2, 4, 5, 7, 8, 10, 12],
    diatonic_chords: None,
    name: "Mixolydian ♭6",
};
/// The Aeolian dominant scale, which is the same as the Mixolydian ♭6 scale.
pub const AEOLIAN_DOMINANT: Scale = Scale {
    intervals: &[0, 2, 4, 5, 7, 8, 10, 12],
    diatonic_chords: None,
    name: "Aeolian dominant",
};
/// The descending melodic major scale, which is the same as the Mixolydian ♭6 scale.
pub const DESCENDING_MELODIC_MAJOR: Scale = Scale {
    intervals: &[0, 2, 4, 5, 7, 8, 10, 12],
    diatonic_chords: None,
    name: "Descending melodic minor",
};
/// The hindu scale, which is the same as the Mixolydian ♭6 scale.
pub const HINDU: Scale = Scale {
    intervals: &[0, 2, 4, 5, 7, 8, 10, 12],
    diatonic_chords: None,
    name: "Hindu",
};
/// The Locrian ♯2 scale, which is the sixth mode of the melodic minor scale and the same as
/// the locrian scale with a natural second.
pub const LOCRIAN_SHARP_TWO: Scale = Scale {
    intervals: &[0, 2, 3, 5, 6, 8, 10, 12],
    diatonic_chords: None,
    name: "Locrian ♯2",
};
/// The Aeolian ♭5 scale, which is the same as the Locrian ♯2 scale.
pub const AEOLIAN_FLAT_FIVE: Scale = Scale {
    intervals: &[0, 2, 3, 5, 6, 8, 10, 12],
    diatonic_chords: None,
    name: "Aeolian ♭5",
};
/// The half diminished scale, which is the same as the Locrian ♯2 scale.
pub const HALF_DIMINISHED: Scale = Scale {
    intervals: &[0, 2, 3, 5, 6, 8, 10, 12],
    diatonic_chords: None,
    name: "Half diminished",
};
/// The altered scale, which is the seventh mode of the melodic minor scale and the same as the
/// major scale with all four altered extensions of the major mode.
pub const ALTERED: Scale = Scale {
    intervals: &[0, 1, 3, 4, 6, 8, 10, 12],
    diatonic_chords: None,
    name: "Altered",
};
/// The altered dominant scale, which is the same as the altered scale.
pub const ALTERED_DOMINANT: Scale = Scale {
    intervals: &[0, 1, 3, 4, 6, 8, 10, 12],
    diatonic_chords: None,
    name: "Altered dominant",
};
/// The super locrian scale, which is the same as the altered scale.
pub const SUPER_LOCRIAN: Scale = Scale {
    intervals: &[0, 1, 3, 4, 6, 8, 10, 12],
    diatonic_chords: None,
    name: "Super locrian",
};
/// The diminished scale, which contains an alternating pattern of whole tones followed by
/// semitones, starting with a whole tone.
pub const DIMINISHED: Scale = Scale {
    intervals: &[0, 2, 3, 5, 6, 8, 9, 11, 12],
    diatonic_chords: None,
    name: "Diminished",
};
/// The dominant diminished scale, which contains an alternating pattern of semitones followed
/// by whole tones, starting with a semitone.
pub const DOMINANT_DIMINISHED: Scale = Scale {
    intervals: &[0, 1, 3, 4, 6, 7, 9, 10, 12],
    diatonic_chords: None,
    name: "Dominant diminished",
};
/// A nonatonic blues scale, which is derived from the major scale with an added flat third and
/// an added flat seventh of the key.
pub const NONATONIC_BLUES: Scale = Scale {
    intervals: &[0, 2, 3, 4, 5, 7, 9, 10, 11, 12],
    diatonic_chords: None,
    name: "Nonatonic blues",
};
/// The major blues scale, which is a hexatonic scale derived from the major pentatonic scale
/// with an added flat third of the key.
pub const MAJOR_BLUES: Scale = Scale {
    intervals: &[0, 2, 3, 4, 7, 9, 12],
    diatonic_chords: None,
    name: "Major blues",
};
/// The minor blues scale, which is a hexatonic scale derived from the minor pentatonic scale
/// with an added flat fifth of the key.
pub const MINOR_BLUES: Scale = Scale {
    intervals: &[0, 3, 5, 6, 7, 10, 12],
    diatonic_chords: None,
    name: "Minor blues",
};
/// The whole tone scale, which is a hexatonic scale where each tone or pitch class is
/// separated by a whole note or two semitones.
pub const WHOLE: Scale = Scale {
    intervals: &[0, 2, 4, 6, 8, 10, 12],
    diatonic_chords: None,
    name: "Whole",
};
/// The chromatic scale, which consists of all twelve pitch classes separated by a semitone.
pub const CHROMATIC: Scale = Scale {
    intervals: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
    diatonic_chords: None,
    name: "Chromatic",
};
