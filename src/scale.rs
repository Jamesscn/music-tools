use crate::note::Note;
use crate::chord::Chord;
use crate::interval::Interval;
use crate::pitchclass::PitchClass;
use crate::common::{ScaleType, PentatonicType};

/// A structure used to represent a scale of notes, or a major or minor
/// pentatonic variation of a scale.
#[derive(Clone, Debug)]
pub struct Scale {
    intervals: Vec<Interval>,
    scale: ScaleType,
    pentatonic: PentatonicType
}

impl Scale {
    /// Constructs a scale of notes given the type of scale, and optionally
    /// a pentatonic.
    /// 
    /// # Parameters
    /// 
    /// - `scale`: A [`ScaleType`] representing the type of scale to return.
    /// - `pentatonic`: A [`PentatonicType`] representing whether a major or
    /// minor pentatonic should be applied to the scale, or if no pentatonic
    /// should be applied. Pentatonics should only be provided if the type of
    /// scale corresponds to a heptatonic scale, otherwise the function will
    /// return None.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use musictools::scale::Scale;
    /// use musictools::common::{ScaleType, PentatonicType};
    /// 
    /// let locrian = Scale::from(ScaleType::Locrian, PentatonicType::None).unwrap();
    /// let some_pentatonic = Scale::from(ScaleType::Minor, PentatonicType::Major).unwrap();
    /// let chromatic_scale = Scale::from(ScaleType::Chromatic, PentatonicType::None).unwrap();
    /// ```
    pub fn from(scale: ScaleType, pentatonic: PentatonicType) -> Option<Scale> {
        let scale_intervals: Vec<u8> = match scale {
            ScaleType::Major | ScaleType::Ionian => vec![0, 2, 4, 5, 7, 9, 11, 12],
            ScaleType::Minor | ScaleType::Aeolian | ScaleType::NaturalMinor | ScaleType::DescendingMelodicMinor => vec![0, 2, 3, 5, 7, 8, 10, 12],
            ScaleType::Dorian => vec![0, 2, 3, 5, 7, 9, 10, 12],
            ScaleType::Phrygian => vec![0, 1, 3, 5, 7, 8, 10, 12],
            ScaleType::Lydian => vec![0, 2, 4, 6, 7, 9, 11, 12],
            ScaleType::Mixolydian => vec![0, 2, 4, 5, 7, 9, 10, 12],
            ScaleType::Locrian => vec![0, 1, 3, 5, 6, 8, 10, 12],
            ScaleType::HarmonicMinor => vec![0, 2, 3, 5, 7, 8, 11, 12],
            ScaleType::AscendingMelodicMinor => vec![0, 2, 3, 5, 7, 9, 11, 12],
            ScaleType::PhrygianDominant => vec![0, 1, 4, 5, 7, 8, 10, 12],
            ScaleType::NonatonicBlues => vec![0, 2, 3, 4, 5, 7, 9, 10, 11, 12],
            ScaleType::MajorBlues => vec![0, 2, 3, 4, 7, 9, 12],
            ScaleType::MinorBlues => vec![0, 3, 5, 6, 7, 10, 12],
            ScaleType::Whole => vec![0, 2, 4, 6, 8, 10, 12],
            ScaleType::Chromatic => vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
        };
        let mut intervals: Vec<Interval> = Vec::new();
        for interval_value in scale_intervals {
            intervals.push(Interval::from(interval_value));
        }
        if pentatonic != PentatonicType::None && intervals.len() != 8 {
            return None;
        }
        if pentatonic == PentatonicType::Major {
            intervals.remove(6);
            intervals.remove(3);
        } else if pentatonic == PentatonicType::Minor {
            intervals.remove(5);
            intervals.remove(1);
        }
        Some(Scale {
            intervals,
            scale,
            pentatonic
        })
    }

    /// Returns a vector of [`Interval`] representing the intervals of each
    /// of the notes in the scale with respect to the tonic.
    pub fn get_intervals(&self) -> Vec<Interval> {
        self.intervals.clone()
    }

    /// Returns a [`ScaleType`] representing the type of the current scale.
    pub fn get_scale_type(&self) -> ScaleType {
        self.scale
    }

    /// Returns a [`PentatonicType`] representing the type of the pentatonic
    /// used to construct the scale.
    pub fn get_pentatonic_type(&self) -> PentatonicType {
        self.pentatonic
    }

    /// Returns true if the scale is diatonic or heptatonic (has 7 notes), or
    /// false if otherwise.
    pub fn is_diatonic(&self) -> bool {
        if self.intervals.len() == 8 {
            return true;
        }
        false
    }

    /// Returns true if the scale is pentatonic (has 5 notes), or false if
    /// otherwise.
    pub fn is_pentatonic(&self) -> bool {
        if self.intervals.len() == 6 {
            return true;
        }
        false
    }

    /// Returns an [`Option<Vec<Chord>>`] with a vector of the seven diatonic
    /// chords of the current scale, given the pitch class of the tonic and
    /// optionally the octave of each of these chords, or [`None`] if the
    /// current scale is not diatonic.
    /// 
    /// # Parameters
    /// 
    /// - `tonic`: A [`PitchClass`] representing the pitch class of the tonic
    /// which will be offset by the numeral.
    /// - `octave`: An [`Option<i8>`] which can be an integer representing the
    /// octave of the first diatonic chord, or [`None`] if the chords should
    /// not have any octave.
    /// - `with_seventh`: A boolean which if set to true ensures that the
    /// chords that are returned contain the corresponding seventh intervals
    /// for the mode or scale, or if set to false ensures that the chords that
    /// are returns are only triads.
    /// 
    /// # Examples
    /// 
    /// The following example shows how one can obtain the diatonic chords with
    /// sevenths for the G locrian scale, starting at the fifth octave.
    /// 
    /// ```rust
    /// use musictools::scale::Scale;
    /// use musictools::pitchclass::PitchClasses;
    /// use musictools::common::{ScaleType, PentatonicType};
    /// 
    /// let locrian = Scale::from(ScaleType::Locrian, PentatonicType::None).unwrap();
    /// let g_locrian_chords = locrian.get_diatonic_chords(PitchClasses::G, Some(5), true).unwrap();
    /// let mut index = 1;
    /// for chord in g_locrian_chords {
    ///     let chord_notes = chord.to_notes().unwrap();
    ///     println!("Diatonic chord #{} of the G locrian scale contains the following notes:", index);
    ///     for note in chord_notes {
    ///         println!("{}{}", note.get_pitch_class().get_names()[0], note.get_octave());
    ///     }
    ///     index += 1;
    /// }
    /// ```
    pub fn get_diatonic_chords(&self, tonic: PitchClass, octave: Option<i8>, with_seventh: bool) -> Option<Vec<Chord>> {
        let chord_numerals: [&str; 7];
        if with_seventh {
            chord_numerals = match self.scale {
                ScaleType::Minor => ["i7", "ii°7", "bIIImaj7", "iv7", "Vmaj7", "bVImaj7", "bVII7"],
                ScaleType::Major | ScaleType::Ionian => ["Imaj7", "ii7", "iii7", "IVmaj7", "V7", "vi7", "vii°7"],
                ScaleType::Dorian => ["i7", "ii7", "bIIImaj7", "IV7", "v7", "vi°7", "bVIImaj7"],
                ScaleType::Phrygian => ["i7", "bIImaj7", "bIII7", "iv7", "v°7", "bVImaj7", "bvii7"],
                ScaleType::Lydian => ["Imaj7", "II7", "iii7", "#iv°7", "Vmaj7", "vi7", "vii7"],
                ScaleType::Mixolydian => ["I7", "ii7", "iii°7", "IVmaj7", "v7", "vi7", "bVIImaj7"],
                ScaleType::Aeolian | ScaleType::NaturalMinor => ["i7", "ii°7", "bIIImaj7", "iv7", "v7", "bVImaj7", "bVII7"],
                ScaleType::Locrian => ["i°7", "bIImaj7", "biii7", "iv7", "bVmaj7", "bVI7", "bvii7"],
                _ => return None
            };
        } else {
            chord_numerals = match self.scale {
                ScaleType::Minor | ScaleType::Aeolian | ScaleType::NaturalMinor => ["i", "ii°", "bIII", "iv", "V", "bVI", "bVII"],
                ScaleType::Major | ScaleType::Ionian => ["I", "ii", "iii", "IV", "V", "vi", "vii°"],
                ScaleType::Dorian => ["i", "ii", "bIII", "IV", "v", "vi°", "bVII"],
                ScaleType::Phrygian => ["i", "bII", "bIII", "iv", "v°", "bVI", "bvii"],
                ScaleType::Lydian => ["I", "II", "iii", "#iv°", "V", "vi", "vii"],
                ScaleType::Mixolydian => ["I", "ii", "iii°", "IV", "v", "vi", "bVII"],
                ScaleType::Locrian => ["i°", "bII", "biii", "iv", "bV", "bVI", "bvii"],
                _ => return None
            };
        }
        let chords = chord_numerals.iter().map(|x| Chord::from_numeral(x, tonic, octave).unwrap()).collect();
        Some(chords)
    }

    /// Converts the scale into a [`Chord`].
    /// 
    /// # Parameters
    /// 
    /// - `tonic`: An [`Option<PitchClass>`] which will serve as the pitch
    /// class of the tonic note if defined. If [`None`] is provided then the
    /// chord will not assign the intervals it holds to any pitch classes.
    /// - `octave`: An [`Option<i8>`] which will represent the octave the
    /// chord is based on if defined. If [`None`] is provided then the chord
    /// will not assign the intervals it holds to any octaves.
    pub fn to_chord(&self, tonic: Option<PitchClass>, octave: Option<i8>) -> Chord {
        let mut chord = Chord::new(tonic, octave);
        for index in 1..self.intervals.len() {
            chord.add_interval(self.intervals[index]);
        }
        chord
    }

    /// Converts the scale to a vector of [`Note`], given a pitch class as the
    /// tonic and the octave to place the notes of the chord over.
    /// 
    /// # Parameters
    /// 
    /// - `tonic`: A [`PitchClass`] representing the pitch class of the tonic
    /// of the set of notes.
    /// - `starting_octave`: An integer representing the octave to place the
    /// tonic on.
    pub fn to_notes(&self, tonic: PitchClass, starting_octave: i8) -> Vec<Note> {
        self.to_chord(Some(tonic), Some(starting_octave)).to_notes().unwrap()
    }

    /// Converts the scale to a vector of [`PitchClass`], given a pitch class
    /// as the tonic.
    /// 
    /// # Parameters
    /// 
    /// - `tonic`: A [`PitchClass`] representing the pitch class of the tonic
    /// of the other pitch classes.
    pub fn to_pitch_classes(&self, tonic: PitchClass) -> Vec<PitchClass> {
        self.to_chord(Some(tonic), None).to_pitch_classes().unwrap()
    }
}

impl PartialEq for Scale {
    fn eq(&self, other: &Self) -> bool {
        self.scale == other.scale && self.pentatonic == other.pentatonic
    }
}