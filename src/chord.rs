use regex::Regex;
use crate::note::Note;
use crate::common::TriadQuality;
use crate::pitchclass::PitchClass;
use crate::interval::{Interval, Intervals};

#[derive(Clone, Debug)]
/// A structure which holds a chord, which is a group of consecutive intervals
/// with a given inversion. A chord can optionally have a tonic which will
/// define the pitch classes of each of the notes in the chord, and also an
/// octave which will define the octaves of these pitch classes, or neither of
/// these attributes.
pub struct Chord {
    intervals: Vec<Interval>,
    tonic: Option<PitchClass>,
    octave: Option<u8>,
    inversion: usize
}

impl Chord {
    /// Constructs a chord with a single interval, the perfect unison.
    /// 
    /// # Parameters
    /// 
    /// - `tonic`: An [`Option<PitchClass>`] which will serve as the pitch
    /// class of the tonic note if defined. If [`None`] is provided then the
    /// chord will not assign the intervals it holds to any pitch classes.
    /// - `octave`: An [`Option<u8>`] which will represent the octave the
    /// chord is based on if defined. If [`None`] is provided then the chord
    /// will not assign the intervals it holds to any octaves.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use musictools::chord::Chord;
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let unison_chord = Chord::new(None, None);
    /// let g_unison_chord = Chord::new(Some(PitchClasses::G), None);
    /// let g5_unison_chord = Chord::new(Some(PitchClasses::G), Some(5));
    /// ```
    pub fn new(tonic: Option<PitchClass>, octave: Option<u8>) -> Chord {
        return Chord {
            intervals: Vec::from([Intervals::PERFECT_UNISON]),
            tonic,
            octave,
            inversion: 0
        }
    }

    /// Constructs a chord from a triad with a specific quality.
    /// 
    /// # Parameters
    /// 
    /// - `triad_quality`: A [`TriadQuality`] which represents the type or
    /// quality of the triad to construct, such as a major or minor triad.
    /// - `tonic`: An [`Option<PitchClass>`] which will serve as the pitch
    /// class of the tonic note if defined. If [`None`] is provided then the
    /// chord will not assign the intervals it holds to any pitch classes.
    /// - `octave`: An [`Option<u8>`] which will represent the octave the
    /// chord is based on if defined. If [`None`] is provided then the chord
    /// will not assign the intervals it holds to any octaves.
    /// 
    /// # Examples
    /// 
    /// The following example demonstrates the creation of a general major
    /// triad with no particular tonic pitch class.
    /// 
    /// ```rust
    /// use musictools::chord::Chord;
    /// use musictools::common::TriadQuality;
    /// 
    /// let chord = Chord::from_triad(TriadQuality::Major, None, None);
    /// ```
    /// 
    /// The following example demonstrates the creation of a G sus2 triad:
    /// 
    /// ```rust
    /// use musictools::chord::Chord;
    /// use musictools::common::TriadQuality;
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let chord = Chord::from_triad(TriadQuality::Sus2, Some(PitchClasses::G), None);
    /// ```
    /// 
    /// The following example demonstrates the creation of a C5 augmented triad:
    /// 
    /// ```rust
    /// use musictools::chord::Chord;
    /// use musictools::common::TriadQuality;
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let chord = Chord::from_triad(TriadQuality::Augmented, Some(PitchClasses::C), Some(5));
    /// ```
    pub fn from_triad(triad_quality: TriadQuality, tonic: Option<PitchClass>, octave: Option<u8>) -> Chord {
        let intervals: Vec<Interval> = match triad_quality {
            TriadQuality::Major => vec![Intervals::PERFECT_UNISON, Intervals::MAJOR_THIRD, Intervals::PERFECT_FIFTH],
            TriadQuality::Minor => vec![Intervals::PERFECT_UNISON, Intervals::MINOR_THIRD, Intervals::PERFECT_FIFTH],
            TriadQuality::Sus2 => vec![Intervals::PERFECT_UNISON, Intervals::MAJOR_SECOND, Intervals::PERFECT_FIFTH],
            TriadQuality::Sus4 => vec![Intervals::PERFECT_UNISON, Intervals::PERFECT_FOURTH, Intervals::PERFECT_FIFTH],
            TriadQuality::Augmented => vec![Intervals::PERFECT_UNISON, Intervals::MAJOR_THIRD, Intervals::MINOR_SIXTH],
            TriadQuality::Diminished => vec![Intervals::PERFECT_UNISON, Intervals::MINOR_THIRD, Intervals::DIMINISHED_FIFTH]
        };
        return Chord {
            intervals,
            tonic,
            octave,
            inversion: 0
        }
    }

    /// Constructs a chord from a string with a roman numeral that represents
    /// the offset of the chord from a tonic, and a pitch class representing
    /// that tonic. One can also provide an [`Option<u8>`] representing the
    /// octave of the chord to be constructed. The string can also contain an
    /// accidental, the quality of the chord and a seventh note. This function
    /// returns an [`Option<Chord>`] which can be [`None`] if the input string
    /// was invalid.
    /// 
    /// # Parameters
    /// 
    /// - `input_numeral`: A string that can contain the following items in
    /// the following order:
    ///     - An optional accidental `b` or `♭` which will treat the chord as
    ///     a flat chord, or `#` or `♯` which will treat the chord as a sharp
    ///     chord.
    ///     - A numeral I - VII or i - vii which will represent the scale
    ///     degree to offset the chord from the tonic. If the numeral is in
    ///     uppercase then the chord will be a major chord, and if it is in
    ///     lowercase it will be a minor chord.
    ///     - A quality `°` which will make the chord diminished or `+` which
    ///     will make the chord augmented.
    ///     - A seventh `7` which will add a minor seventh on top of the chord,
    ///     or `maj7` which will add a major seventh on top of the chord.
    /// - `tonic`: A [`PitchClass`] representing the tonic or root note which
    /// will be offset by the numeral.
    /// - `octave`: An [`Option<u8>`] representing the octave of the chord that
    /// will be returned. If this is [`None`], the chord will have no
    /// particular octave.
    /// 
    /// # Examples
    /// 
    /// The following example demonstrates the creation of two copies of a
    /// major tetrad seven scale degrees above C4, which will be flat,
    /// augmented and will also contain a minor seventh.
    /// 
    /// ```rust
    /// use musictools::chord::Chord;
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let chord1 = Chord::from_numeral("bVII+7", PitchClasses::C, Some(4)).unwrap();
    /// let chord2 = Chord::from_numeral("♭VII+7", PitchClasses::C, Some(4)).unwrap();
    /// ```
    /// 
    /// The following example demonstrates the creation of two copies of a
    /// minor tetrad two scale degrees above G5 sharp, which will be sharp,
    /// diminished and will also contain a major seventh:
    /// 
    /// ```rust
    /// use musictools::chord::Chord;
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let chord1 = Chord::from_numeral("#ii°maj7", PitchClasses::G_SHARP, Some(5)).unwrap();
    /// let chord2 = Chord::from_numeral("♯ii°maj7", PitchClasses::G_SHARP, Some(5)).unwrap();
    /// ```
    /// 
    /// The following example demonstrates the creation of a minor triad
    /// three scale degrees above A with no octave.
    /// 
    /// ```rust
    /// use musictools::chord::Chord;
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let chord = Chord::from_numeral("iii", PitchClasses::A, None).unwrap();
    /// ```
    pub fn from_numeral(input_numeral: &str, tonic: PitchClass, octave: Option<u8>) -> Option<Chord> {
        let numeral_array = ["I", "II", "III", "IV", "V", "VI", "VII"];
        let numeral_regex = Regex::new(r"^(b|♭|\#|♯)?(I|II|III|IV|V|VI|VII|i|ii|iii|iv|v|vi|vii)(°|\+)?(maj7|7)?$").unwrap();
        if !numeral_regex.is_match(&input_numeral) {
            return None;
        }
        let regex_capture_groups = numeral_regex.captures(&input_numeral).unwrap();
        let accidental = regex_capture_groups.get(1).map_or("", |m| m.as_str());
        let numeral = regex_capture_groups.get(2).map_or("", |m| m.as_str());
        let quality = regex_capture_groups.get(3).map_or("", |m| m.as_str());
        let seventh = regex_capture_groups.get(4).map_or("", |m| m.as_str());
        let numeral_value = numeral_array.iter().position(|&x| x == numeral.to_ascii_uppercase()).unwrap();
        let triad_quality: TriadQuality;
        if numeral.chars().all(char::is_uppercase) {
            if quality == "+" {
                triad_quality = TriadQuality::Augmented;
            } else if quality == "°" {
                return None;
            } else {
                triad_quality = TriadQuality::Major;
            }
        } else {
            if quality == "°" {
                triad_quality = TriadQuality::Diminished;
            } else if quality == "+" {
                return None;
            } else {
                triad_quality = TriadQuality::Minor;
            }
        }
        let mut increment: u8 = match numeral_value {
            0 => 0,
            1 => 2,
            2 => 4,
            3 => 5,
            4 => 7,
            5 => 9,
            6 => 11,
            _ => return None
        };
        if accidental == "b" || accidental == "♭" {
            increment = match numeral_value {
                1 => 1,
                2 => 3,
                4 => 6,
                5 => 8,
                6 => 10,
                _ => return None
            };
        } else if accidental == "#" || accidental == "♯" {
            increment = match numeral_value {
                0 => 1,
                1 => 3,
                3 => 6,
                4 => 8,
                5 => 10,
                _ => return None
            };
        }
        let chord_tonic = tonic.get_offset(increment as i8);
        let chord_octave = match octave {
            Some(octave_value) => Some(octave_value + (tonic.get_value() + increment) / 12),
            None => None
        };
        let mut chord = Chord::from_triad(triad_quality, Some(chord_tonic), chord_octave);
        if seventh == "maj7" {
            chord.add_interval(Intervals::MAJOR_SEVENTH);
        } else if seventh == "7" {
            chord.add_interval(Intervals::MINOR_SEVENTH);
        }
        return Some(chord);
    }

    /// Adds an interval on top of the current chord.
    /// 
    /// # Parameters
    /// 
    /// - `interval`: An [`Interval`] representing the interval to add to the
    /// chord.
    /// 
    /// # Examples
    /// 
    /// The following example demonstrates adding a minor seventh to a major
    /// triad with no particular pitch classes or octaves.
    /// 
    /// ```rust
    /// use musictools::chord::Chord;
    /// use musictools::common::TriadQuality;
    /// use musictools::interval::Intervals;
    /// 
    /// let mut chord = Chord::from_triad(TriadQuality::Major, None, None);
    /// chord.add_interval(Intervals::MINOR_SEVENTH);
    /// ```
    pub fn add_interval(&mut self, interval: Interval) {
        let mut insert_index = 0;
        for (index, value) in self.intervals.iter().enumerate() {
            if value == &interval {
                return
            }
            if value > &interval {
                break
            }
            insert_index = index + 1;
        }
        self.intervals.insert(insert_index, interval);
    }

    /// Returns a vector of [`Interval`] objects representing the intervals
    /// of the current chord with the inversion of the chord applied.
    pub fn get_intervals(&self) -> Vec<Interval> {
        let mut values: Vec<i8> = Vec::new();
        let first_half_octave_offset = self.intervals[self.inversion as usize].get_value() as i8 / 12;
        for index in self.inversion..self.intervals.len() {
            values.push(self.intervals[index].get_value() as i8 - 12 * first_half_octave_offset);
        }
        let second_half_octave_offset = values[values.len() - 1] / 12 + 1;
        for index in 0..self.inversion {
            values.push(self.intervals[index].get_value() as i8 + 12 * second_half_octave_offset)
        }
        let mut intervals: Vec<Interval> = Vec::new();
        for value in values {
            intervals.push(Interval::from(value as u8));
        }
        return intervals;
    }

    /// Sets the inversion of the current chord which changes the order of
    /// the intervals in the chord.
    /// 
    /// # Parameters
    /// 
    /// - `inversion`: The inversion number to offset the intervals by. This
    /// number must be positive, and if it exceeds the number of intervals it
    /// is automatically wrapped around.
    /// 
    /// # Examples
    /// 
    /// The following example constructs the first inversion of the major
    /// triad.
    /// 
    /// ```rust
    /// use musictools::chord::Chord;
    /// use musictools::common::TriadQuality;
    /// 
    /// let mut chord = Chord::from_triad(TriadQuality::Major, None, None);
    /// chord.set_inversion(1);
    /// ```
    /// 
    /// The following example constructs the second inversion of the C minor
    /// triad.
    /// 
    /// ```rust
    /// use musictools::chord::Chord;
    /// use musictools::common::TriadQuality;
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let mut chord = Chord::from_triad(TriadQuality::Minor, Some(PitchClasses::C), None);
    /// chord.set_inversion(2);
    /// ```
    pub fn set_inversion(&mut self, inversion: u8) {
        self.inversion = inversion as usize % self.intervals.len();
    }

    /// Returns a positive integer representing the inversion of the current
    /// chord.
    pub fn get_inversion(&self) -> u8 {
        return self.inversion as u8;
    }

    /// Sets the tonic of the current chord to the [`Option<PitchClass>`]
    /// passed to this function. If this is [`None`], it will unset the current
    /// pitch class of the chord.
    /// 
    /// # Parameters
    /// 
    /// - `tonic`: An [`Option<PitchClass>`] which will represent the new tonic
    /// of the current chord.
    pub fn set_tonic(&mut self, tonic: Option<PitchClass>) {
        self.tonic = tonic;
    }

    /// Returns an [`Option<PitchClass>`] which can be [`None`] if the chord
    /// has no tonic pitch class, or otherwise the pitch class of the tonic.
    pub fn get_tonic(&self) -> Option<PitchClass> {
        return self.tonic;
    }

    /// Sets the octave of the tonic of the current chord to the [`Option<u8>`]
    /// passed to this function. If this is [`None`], it will unset the current
    /// octave of the chord.
    /// 
    /// # Parameters
    /// 
    /// - `octave`: An [`Option<u8>`] which will represent the new octave of
    /// the current chord.
    pub fn set_octave(&mut self, octave: Option<u8>) {
        self.octave = octave;
    }

    /// Returns an [`Option<u8>`] which can be [`None`] if the chord has no
    /// octave or a positive integer if it does have one.
    pub fn get_octave(&self) -> Option<u8> {
        return self.octave;
    }

    /// Returns an [`Option<Vec<Note>>`] which contains a vector of consecutive
    /// [`Note`] objects with the pitch classes and octaves of each note in the
    /// chord, or [`None`] if either the tonic or the octave of the chord are
    /// [`None`].
    /// 
    /// # Examples
    /// 
    /// The following example will create a G major chord on the fourth octave
    /// whose notes will be G4, B4 and D5.
    /// 
    /// ```rust
    /// use musictools::chord::Chord;
    /// use musictools::common::TriadQuality;
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let mut chord = Chord::from_triad(TriadQuality::Major, Some(PitchClasses::G), Some(4));
    /// let notes = chord.to_notes().unwrap();
    /// ```
    pub fn to_notes(&self) -> Option<Vec<Note>> {
        if self.tonic == None {
            return None;
        }
        if self.octave == None {
            return None;
        }
        let mut notes: Vec<Note> = Vec::new();
        let intervals = self.get_intervals();
        for interval in intervals {
            let current_octave = self.octave.unwrap() + (self.tonic.unwrap().get_value() + interval.get_value()) / 12;
            let current_semitone = interval.get_value() % 12;
            let current_pitch_class = self.tonic.unwrap().get_offset(current_semitone as i8);
            let current_note = Note::from(current_pitch_class, current_octave);
            notes.push(current_note);
        }
        return Some(notes);
    }

    /// Returns an [`Option<Vec<PitchClass>>`] which contains a vector of
    /// [`PitchClass`] corresponding to the pitch classes of the notes in the
    /// current chord, or [`None`] if the tonic of the chord is [`None`].
    /// Note that this representation of a chord is not optimal because
    /// it makes it impossible to distinguish the difference between an
    /// interval less than an octave and any interval larger than an octave.
    pub fn to_pitch_classes(&self) -> Option<Vec<PitchClass>> {
        if self.tonic == None {
            return None;
        }
        let mut pitch_classes: Vec<PitchClass> = Vec::new();
        let intervals = self.get_intervals();
        for interval in intervals {
            let current_semitone = interval.get_value() % 12;
            let current_pitch_class = self.tonic.unwrap().get_offset(current_semitone as i8);
            pitch_classes.push(current_pitch_class);
        }
        return Some(pitch_classes);
    }
}