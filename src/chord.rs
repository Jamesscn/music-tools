use crate::common::{IncompleteChordError, InputError, TriadQuality};
use crate::interval::{Interval, Intervals};
use crate::note::Note;
use crate::pitchclass::PitchClass;
use regex::Regex;

/// A structure which holds a chord, which is a group of consecutive intervals with a given
/// inversion. A chord can optionally have a tonic which will define the pitch classes of each of
/// the notes in the chord, and optionally also an octave which will define the octaves of these
/// pitch classes.
#[derive(Clone, Debug)]
pub struct Chord {
    intervals: Vec<Interval>,
    tonic: Option<&'static PitchClass>,
    octave: Option<i8>,
    inversion: usize,
}

impl Chord {
    /// Constructs a chord with a single interval, the perfect unison.
    ///
    /// # Parameters
    ///
    /// - `tonic`: An [`Option<PitchClass>`] which will serve as the pitch class of the tonic note
    ///   if defined. If [`None`] is provided then the chord will not assign the intervals it holds
    ///   to any pitch classes.
    /// - `octave`: An [`Option<i8>`] which will represent the octave the chord is based on if
    ///   defined. If [`None`] is provided then the chord will not assign the intervals it holds to
    ///   any octaves.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use music_tools::chord::Chord;
    /// use music_tools::pitchclass::PitchClasses;
    ///
    /// let unison_chord = Chord::new(None, None);
    /// let g_unison_chord = Chord::new(Some(PitchClasses::G), None);
    /// let g5_unison_chord = Chord::new(Some(PitchClasses::G), Some(5));
    /// ```
    pub fn new(tonic: Option<&'static PitchClass>, octave: Option<i8>) -> Chord {
        Chord {
            intervals: vec![Intervals::PERFECT_UNISON],
            tonic,
            octave,
            inversion: 0,
        }
    }

    pub fn from(item: impl ToChord) -> Chord {
        Chord {
            intervals: item.get_intervals(),
            tonic: item.get_tonic(),
            octave: item.get_octave(),
            inversion: item.get_inversion(),
        }
    }

    /// Constructs a chord from a triad with a specific quality.
    ///
    /// # Parameters
    ///
    /// - `triad_quality`: A [`TriadQuality`] which represents the type or quality of the triad to
    ///   construct, such as a major or minor triad.
    /// - `tonic`: An [`Option<PitchClass>`] which will serve as the pitch class of the tonic note
    ///   if defined. If [`None`] is provided then the chord will not assign the intervals it holds
    ///   to any pitch classes.
    /// - `octave`: An [`Option<i8>`] which will represent the octave the chord is based on if
    ///   defined. If [`None`] is provided then the chord will not assign the intervals it holds to
    ///   any octaves.
    ///
    /// # Examples
    ///
    /// The following example demonstrates the creation of a general major triad with no particular
    /// tonic pitch class.
    ///
    /// ```rust
    /// use music_tools::chord::Chord;
    /// use music_tools::common::TriadQuality;
    ///
    /// let chord = Chord::from_triad(TriadQuality::Major, None, None);
    /// ```
    ///
    /// The following example demonstrates the creation of a G sus2 triad:
    ///
    /// ```rust
    /// use music_tools::chord::Chord;
    /// use music_tools::common::TriadQuality;
    /// use music_tools::pitchclass::PitchClasses;
    ///
    /// let chord = Chord::from_triad(TriadQuality::Sus2, Some(PitchClasses::G), None);
    /// ```
    ///
    /// The following example demonstrates the creation of a C5 augmented triad:
    ///
    /// ```rust
    /// use music_tools::chord::Chord;
    /// use music_tools::common::TriadQuality;
    /// use music_tools::pitchclass::PitchClasses;
    ///
    /// let chord = Chord::from_triad(TriadQuality::Augmented, Some(PitchClasses::C), Some(5));
    /// ```
    pub fn from_triad(
        triad_quality: TriadQuality,
        tonic: Option<&'static PitchClass>,
        octave: Option<i8>,
    ) -> Chord {
        let intervals: Vec<Interval> = match triad_quality {
            TriadQuality::Major => vec![
                Intervals::PERFECT_UNISON,
                Intervals::MAJOR_THIRD,
                Intervals::PERFECT_FIFTH,
            ],
            TriadQuality::Minor => vec![
                Intervals::PERFECT_UNISON,
                Intervals::MINOR_THIRD,
                Intervals::PERFECT_FIFTH,
            ],
            TriadQuality::Sus2 => vec![
                Intervals::PERFECT_UNISON,
                Intervals::MAJOR_SECOND,
                Intervals::PERFECT_FIFTH,
            ],
            TriadQuality::Sus4 => vec![
                Intervals::PERFECT_UNISON,
                Intervals::PERFECT_FOURTH,
                Intervals::PERFECT_FIFTH,
            ],
            TriadQuality::Augmented => vec![
                Intervals::PERFECT_UNISON,
                Intervals::MAJOR_THIRD,
                Intervals::MINOR_SIXTH,
            ],
            TriadQuality::Diminished => vec![
                Intervals::PERFECT_UNISON,
                Intervals::MINOR_THIRD,
                Intervals::DIMINISHED_FIFTH,
            ],
        };
        Chord {
            intervals,
            tonic,
            octave,
            inversion: 0,
        }
    }

    /// Constructs a chord from a string with a roman numeral that represents the offset of the
    /// chord from a tonic, and a pitch class representing that tonic. One can also provide an
    /// [`Option<i8>`] representing the octave of the chord to be constructed. The string can also
    /// contain an accidental, the quality of the chord and a seventh note. The function returns a
    /// [`Result`] which can contain the new [`Chord`] or an [`InputError`] if the string could not
    /// be parsed correctly.
    ///
    /// # Parameters
    ///
    /// - `input_numeral`: A string that can contain the following items in the following order:
    ///     - (Optional) an accidental `b` or `♭` which will treat the chord as a flat chord, or `#`
    ///       or `♯` which will treat the chord as a sharp chord.
    ///     - (Required) A numeral I - VII or i - vii which will represent the scale degree to
    ///       offset the chord from the tonic. If the numeral is in uppercase then the chord will be
    ///       a major chord, and if it is in lowercase it will be a minor chord.
    ///     - (Optional) A quality `°` which will make the chord diminished or `+` which will make
    ///       the chord augmented.
    ///     - (Optional) A seventh `7` which will add a minor seventh on top of the chord, or `maj7`
    ///       which will add a major seventh on top of the chord.
    /// - `tonic`: A [`PitchClass`] representing the tonic or root note which will be offset by the
    ///   numeral.
    /// - `octave`: An [`Option<i8>`] representing the octave of the chord that will be returned. If
    ///   this is [`None`], the chord will have no particular octave.
    ///
    /// # Examples
    ///
    /// The following example demonstrates the creation of two copies of a major tetrad seven scale
    /// degrees above C4, which will be flat, augmented and will also contain a minor seventh.
    ///
    /// ```rust
    /// use music_tools::chord::Chord;
    /// use music_tools::pitchclass::PitchClasses;
    ///
    /// let chord1 = Chord::from_numeral("bVII+7", PitchClasses::C, Some(4)).unwrap();
    /// let chord2 = Chord::from_numeral("♭VII+7", PitchClasses::C, Some(4)).unwrap();
    /// ```
    ///
    /// The following example demonstrates the creation of two copies of a minor tetrad two scale
    /// degrees above G5 sharp, which will be sharp, diminished and will also contain a major
    /// seventh:
    ///
    /// ```rust
    /// use music_tools::chord::Chord;
    /// use music_tools::pitchclass::PitchClasses;
    ///
    /// let chord1 = Chord::from_numeral("#ii°maj7", PitchClasses::G_SHARP, Some(5)).unwrap();
    /// let chord2 = Chord::from_numeral("♯ii°maj7", PitchClasses::G_SHARP, Some(5)).unwrap();
    /// ```
    ///
    /// The following example demonstrates the creation of a minor triad three scale degrees above A
    /// with no octave.
    ///
    /// ```rust
    /// use music_tools::chord::Chord;
    /// use music_tools::pitchclass::PitchClasses;
    ///
    /// let chord = Chord::from_numeral("iii", PitchClasses::A, None).unwrap();
    /// ```
    pub fn from_numeral(
        input_numeral: &str,
        tonic: &'static PitchClass,
        octave: Option<i8>,
    ) -> Result<Chord, InputError> {
        let numeral_array = ["I", "II", "III", "IV", "V", "VI", "VII"];
        let numeral_regex =
            Regex::new(r"^(b|♭|\#|♯)?(I|II|III|IV|V|VI|VII|i|ii|iii|iv|v|vi|vii)(°|\+)?(maj7|7)?$")
                .unwrap();
        if !numeral_regex.is_match(input_numeral) {
            return Err(InputError {
                message: "string does not conform to expected numeral format",
            });
        }
        let regex_capture_groups = numeral_regex.captures(input_numeral).unwrap();
        let accidental = regex_capture_groups.get(1).map_or("", |m| m.as_str());
        let numeral = regex_capture_groups.get(2).map_or("", |m| m.as_str());
        let quality = regex_capture_groups.get(3).map_or("", |m| m.as_str());
        let seventh = regex_capture_groups.get(4).map_or("", |m| m.as_str());
        let numeral_value = numeral_array
            .iter()
            .position(|&x| x == numeral.to_ascii_uppercase())
            .unwrap();
        let triad_quality: TriadQuality;
        if numeral.chars().all(char::is_uppercase) {
            if quality == "+" {
                triad_quality = TriadQuality::Augmented;
            } else if quality == "°" {
                return Err(InputError {
                    message: concat!(
                        "numeral cannot be uppercase and contain ° symbol, it must either be ",
                        "augmented (uppercase with a +) or diminished (lowercase with a °)"
                    ),
                });
            } else {
                triad_quality = TriadQuality::Major;
            }
        } else if quality == "°" {
            triad_quality = TriadQuality::Diminished;
        } else if quality == "+" {
            return Err(InputError {
                message: concat!(
                    "numeral cannot be lowercase and contain + symbol, it must either be ",
                    "augmented (uppercase with a +) or diminished (lowercase with a °)"
                ),
            });
        } else {
            triad_quality = TriadQuality::Minor;
        }
        let increment: u8;
        if accidental == "b" || accidental == "♭" {
            increment =
                match numeral_value {
                    1 => 1,
                    2 => 3,
                    4 => 6,
                    5 => 8,
                    6 => 10,
                    _ => return Err(InputError {
                        message:
                            "only numerals ii, II, iii, III, v, V, vi, VI, vii and VII can be flat",
                    }),
                };
        } else if accidental == "#" || accidental == "♯" {
            increment = match numeral_value {
                0 => 1,
                1 => 3,
                3 => 6,
                4 => 8,
                5 => 10,
                _ => {
                    return Err(InputError {
                        message: "only numerals i, I, ii, II, iv, IV, v, V, vi and VI can be sharp",
                    })
                }
            };
        } else {
            increment = match numeral_value {
                0 => 0,
                1 => 2,
                2 => 4,
                3 => 5,
                4 => 7,
                5 => 9,
                6 => 11,
                _ => unreachable!(),
            };
        }
        let chord_tonic = tonic.get_offset(increment as i8);
        let chord_octave =
            octave.map(|octave_value| octave_value + ((tonic.get_value() + increment) / 12) as i8);
        let mut chord = Chord::from_triad(triad_quality, Some(chord_tonic), chord_octave);
        if seventh == "maj7" {
            chord.add_interval(Intervals::MAJOR_SEVENTH);
        } else if seventh == "7" {
            chord.add_interval(Intervals::MINOR_SEVENTH);
        }
        Ok(chord)
    }

    /// Adds an interval on top of the current chord.
    ///
    /// # Parameters
    ///
    /// - `interval`: An [`Interval`] representing the interval to add to the chord.
    ///
    /// # Examples
    ///
    /// The following example demonstrates adding a minor seventh to a major triad with no
    /// particular pitch classes or octaves assigned to the chord.
    ///
    /// ```rust
    /// use music_tools::chord::Chord;
    /// use music_tools::common::TriadQuality;
    /// use music_tools::interval::Intervals;
    ///
    /// let mut chord = Chord::from_triad(TriadQuality::Major, None, None);
    /// chord.add_interval(Intervals::MINOR_SEVENTH);
    /// ```
    pub fn add_interval(&mut self, interval: Interval) {
        let mut insert_index = 0;
        for (index, value) in self.intervals.iter().enumerate() {
            if value == &interval {
                return;
            }
            if value > &interval {
                break;
            }
            insert_index = index + 1;
        }
        self.intervals.insert(insert_index, interval);
    }

    /// Returns a vector of [`Interval`] objects representing the intervals of the current chord
    /// with the inversion of the chord applied.
    pub fn get_intervals(&self) -> Vec<Interval> {
        let mut values: Vec<u8> = Vec::new();
        let first_half_octave_offset = self.intervals[self.inversion].get_value() as i8 / 12;
        for index in self.inversion..self.intervals.len() {
            values.push(
                (self.intervals[index].get_value() as i8 - 12 * first_half_octave_offset) as u8,
            );
        }
        let second_half_octave_offset = values[values.len() - 1] as i8 / 12 + 1;
        for index in 0..self.inversion {
            values.push(
                (self.intervals[index].get_value() as i8 + 12 * second_half_octave_offset) as u8,
            );
        }
        values
            .iter()
            .map(|value| Interval::from_value(*value))
            .collect()
    }

    /// Sets the inversion of the current chord which changes the order of the intervals in the
    /// chord.
    ///
    /// # Parameters
    ///
    /// - `inversion`: The inversion number to offset the intervals by. This number must be
    ///   positive, and if it exceeds the number of intervals it is automatically wrapped around.
    ///
    /// # Examples
    ///
    /// The following example constructs the first inversion of the major triad.
    ///
    /// ```rust
    /// use music_tools::chord::Chord;
    /// use music_tools::common::TriadQuality;
    ///
    /// let mut chord = Chord::from_triad(TriadQuality::Major, None, None);
    /// chord.set_inversion(1);
    /// ```
    ///
    /// The following example constructs the second inversion of the C minor triad.
    ///
    /// ```rust
    /// use music_tools::chord::Chord;
    /// use music_tools::common::TriadQuality;
    /// use music_tools::pitchclass::PitchClasses;
    ///
    /// let mut chord = Chord::from_triad(TriadQuality::Minor, Some(PitchClasses::C), None);
    /// chord.set_inversion(2);
    /// ```
    pub fn set_inversion(&mut self, inversion: u8) {
        self.inversion = inversion as usize % self.intervals.len();
    }

    /// Returns a positive integer representing the inversion of the current chord.
    pub fn get_inversion(&self) -> u8 {
        self.inversion as u8
    }

    /// Sets the tonic of the current chord to the [`Option<PitchClass>`] passed to this function.
    /// If this is [`None`], it will unset the current pitch class of the chord.
    ///
    /// # Parameters
    ///
    /// - `tonic`: An [`Option<PitchClass>`] which will represent the new tonic of the current
    ///   chord.
    pub fn set_tonic(&mut self, tonic: Option<&'static PitchClass>) {
        self.tonic = tonic;
    }

    /// Returns an [`Option<PitchClass>`] which can be [`None`] if the chord has no tonic pitch
    /// class, or otherwise the pitch class of the tonic.
    pub fn get_tonic(&self) -> Option<&'static PitchClass> {
        self.tonic
    }

    /// Sets the octave of the tonic of the current chord to the [`Option<i8>`] passed to this
    /// function. If this is [`None`], it will unset the current octave of the chord.
    ///
    /// # Parameters
    ///
    /// - `octave`: An [`Option<i8>`] which will represent the new octave of the current chord.
    pub fn set_octave(&mut self, octave: Option<i8>) {
        self.octave = octave;
    }

    /// Returns an [`Option<i8>`] which can be [`None`] if the chord has no octave or an integer if
    /// it does have one.
    pub fn get_octave(&self) -> Option<i8> {
        self.octave
    }

    /// Returns a [`Result`] which can contain a [`Vec<Note>`] with consecutive [`Note`] objects for
    /// each of the pitch classes and octaves of each note in the chord, or an
    /// [`IncompleteChordError`] if either the tonic or the octave of the chord are [`None`].
    ///
    /// # Examples
    ///
    /// The following example will create a G major chord on the fourth octave whose notes will be
    /// G4, B4 and D5.
    ///
    /// ```rust
    /// use music_tools::chord::Chord;
    /// use music_tools::common::TriadQuality;
    /// use music_tools::pitchclass::PitchClasses;
    ///
    /// let mut chord = Chord::from_triad(TriadQuality::Major, Some(PitchClasses::G), Some(4));
    /// let notes = chord.to_notes().unwrap();
    /// ```
    pub fn to_notes(&self) -> Result<Vec<Note>, IncompleteChordError> {
        if self.tonic.is_none() || self.octave.is_none() {
            return Err(IncompleteChordError {
                needs_tonic: true,
                needs_octave: true,
                has_tonic: self.tonic.is_some(),
                has_octave: self.octave.is_some(),
            });
        }
        let mut notes: Vec<Note> = Vec::new();
        for interval in self.get_intervals() {
            let current_octave = self.octave.unwrap()
                + ((self.tonic.unwrap().get_value() + interval.get_value()) / 12) as i8;
            let current_semitone = interval.get_value() % 12;
            let current_pitch_class = self.tonic.unwrap().get_offset(current_semitone as i8);
            let current_note = Note::from(current_pitch_class, current_octave);
            notes.push(current_note);
        }
        Ok(notes)
    }

    /// Returns a [`Result`] which can contain a [`Vec<PitchClass>`] corresponding to the pitch
    /// classes of the notes in the current chord, or an [`IncompleteChordError`] if the tonic of
    /// the chord is [`None`]. Note that this representation of a chord is not optimal because it
    /// makes it impossible to tell the difference between an interval less than an octave and any
    /// interval larger than an octave.
    pub fn to_pitch_classes(&self) -> Result<Vec<&'static PitchClass>, IncompleteChordError> {
        if self.tonic.is_none() {
            return Err(IncompleteChordError {
                needs_tonic: true,
                needs_octave: false,
                has_tonic: self.tonic.is_some(),
                has_octave: self.octave.is_some(),
            });
        }
        let mut pitch_classes: Vec<&'static PitchClass> = Vec::new();
        let intervals = self.get_intervals();
        for interval in intervals {
            let current_semitone = interval.get_value() % 12;
            let current_pitch_class = self.tonic.unwrap().get_offset(current_semitone as i8);
            pitch_classes.push(current_pitch_class);
        }
        Ok(pitch_classes)
    }
}

pub trait ToChord {
    fn get_intervals(&self) -> Vec<Interval>;
    fn get_inversion(&self) -> usize;
    fn get_tonic(&self) -> Option<&'static PitchClass>;
    fn get_octave(&self) -> Option<i8>;
}

impl ToChord for Chord {
    fn get_intervals(&self) -> Vec<Interval> {
        self.intervals.clone()
    }

    fn get_inversion(&self) -> usize {
        self.inversion
    }

    fn get_tonic(&self) -> Option<&'static PitchClass> {
        self.tonic
    }

    fn get_octave(&self) -> Option<i8> {
        self.octave
    }
}
