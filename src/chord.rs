use regex::Regex;
use crate::note::Note;
use crate::common::TriadQuality;
use crate::pitchclass::PitchClass;
use crate::interval::{Interval, Intervals};

/// A structure which holds a chord, which can be any group of pitch classes.
/// This class does not keep track of the octaves of the pitch classes it
/// holds, however it can store the inversion of the chord.
pub struct Chord {
    tonic: PitchClass,
    intervals: Vec<Interval>,
    inversion: usize
}

impl Chord {
    /// Creates a chord with only the tonic.
    /// 
    /// # Parameters
    /// 
    /// - `tonic`: A [`PitchClass`] representing the tonic or root pitch class
    /// of the chord to construct.
    pub fn new(tonic: PitchClass) -> Chord {
        return Chord {
            tonic,
            intervals: Vec::from([Intervals::PERFECT_UNISON]),
            inversion: 0
        }
    }

    /// Constructs a chord from a triad with a specific quality.
    /// 
    /// # Parameters
    /// 
    /// - `tonic`: A [`PitchClass`] representing the tonic or root pitch class
    /// of the triad to construct.
    /// - `triad_quality`: A [`TriadQuality`] which represents the type or
    /// quality of the triad to construct, such as a major or minor triad.
    /// 
    /// # Examples
    /// 
    /// The following example demonstrates the creation of a C major triad:
    /// 
    /// ```rust
    /// use musictools::chord::Chord;
    /// use musictools::common::TriadQuality;
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let chord = Chord::from_triad(PitchClasses::C, TriadQuality::Major);
    /// ```
    /// 
    /// The following example demonstrates the creation of a B flat sus2 triad:
    /// 
    /// ```rust
    /// use musictools::chord::Chord;
    /// use musictools::common::TriadQuality;
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let chord = Chord::from_triad(PitchClasses::B_FLAT, TriadQuality::Sus2);
    /// ```
    pub fn from_triad(tonic: PitchClass, triad_quality: TriadQuality) -> Chord {
        let intervals: Vec<Interval> = match triad_quality {
            TriadQuality::Major => vec![Intervals::PERFECT_UNISON, Intervals::MAJOR_THIRD, Intervals::PERFECT_FIFTH],
            TriadQuality::Minor => vec![Intervals::PERFECT_UNISON, Intervals::MINOR_THIRD, Intervals::PERFECT_FIFTH],
            TriadQuality::Sus2 => vec![Intervals::PERFECT_UNISON, Intervals::MAJOR_SECOND, Intervals::PERFECT_FIFTH],
            TriadQuality::Sus4 => vec![Intervals::PERFECT_UNISON, Intervals::PERFECT_FOURTH, Intervals::PERFECT_FIFTH],
            TriadQuality::Augmented => vec![Intervals::PERFECT_UNISON, Intervals::MAJOR_THIRD, Intervals::MINOR_SIXTH],
            TriadQuality::Diminished => vec![Intervals::PERFECT_UNISON, Intervals::MINOR_THIRD, Intervals::DIMINISHED_FIFTH]
        };
        return Chord {
            tonic,
            intervals,
            inversion: 0
        }
    }

    /// Constructs a chord from a string with a roman numeral that represents
    /// the offset of the chord from a tonic. The string may also contain
    /// information about the accidental, quality and a seventh note. This
    /// function returns an [`Option`] which can be [`None`] if the string is
    /// invalid.
    /// 
    /// # Parameters
    /// 
    /// - `tonic`: A [`PitchClass`] representing the tonic or root pitch class
    /// which will be offset by the numeral.
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
    /// 
    /// # Examples
    /// 
    /// The following example demonstrates the creation of two copies of a
    /// major tetrad seven scale degrees above C, which will be flat,
    /// augmented and will also contain a minor seventh.
    /// 
    /// ```rust
    /// use musictools::chord::Chord;
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let chord1 = Chord::from_numeral(PitchClasses::C, "bVII+7");
    /// let chord2 = Chord::from_numeral(PitchClasses::C, "♭VII+7");
    /// ```
    /// 
    /// The following example demonstrates the creation of two copies of a
    /// minor tetrad two scale degrees above G sharp, which will be sharp,
    /// diminished and will also contain a major seventh:
    /// 
    /// ```rust
    /// use musictools::chord::Chord;
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let chord = Chord::from_numeral(PitchClasses::G_SHARP, "#ii°maj7");
    /// let chord = Chord::from_numeral(PitchClasses::G_SHARP, "♯ii°maj7");
    /// ```
    /// 
    /// The following example demonstrates the creation of a minor triad
    /// three scale degrees above A:
    /// 
    /// ```rust
    /// use musictools::chord::Chord;
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let chord = Chord::from_numeral(PitchClasses::A, "iii");
    /// ```
    pub fn from_numeral(tonic: PitchClass, input_numeral: &str) -> Option<Chord> {
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
        let mut increment = match numeral_value {
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
        let chord_tonic = tonic.get_offset(increment, true);
        let mut chord = Chord::from_triad(chord_tonic, triad_quality);
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
    /// The following example demonstrates adding a minor seventh to a C major
    /// triad:
    /// 
    /// ```rust
    /// use musictools::chord::Chord;
    /// use musictools::common::TriadQuality;
    /// use musictools::pitchclass::PitchClasses;
    /// use musictools::interval::Intervals;
    /// 
    /// let mut chord = Chord::from_triad(PitchClasses::C, TriadQuality::Major);
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

    /// Returns a vector of [`Interval`] objects representing the pitch
    /// classes contained by the current chord, taking into account the
    /// inversion of the chord.
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

    /// Sets the inversion of the current chord, which changes the order of
    /// the pitch classes in the chord.
    /// 
    /// # Parameters
    /// 
    /// - `inversion`: The inversion number to offset the pitch classes by.
    /// This number must be positive, and if it exceeds the number of pitch
    /// classes it is automatically wrapped around.
    /// 
    /// # Examples
    /// 
    /// The following example constructs the first inversion of the C major
    /// chord, which is a C/E chord starting with the E pitch class. The pitch
    /// classes of the chord will be [E, G, C] instead of [C, E, G]:
    /// 
    /// ```rust
    /// use musictools::chord::Chord;
    /// use musictools::common::TriadQuality;
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let mut chord = Chord::from_triad(PitchClasses::C, TriadQuality::Major);
    /// chord.set_inversion(1);
    /// ```
    /// 
    /// The following example constructs the second inversion of the C major
    /// chord, which is a C/G chord starting with the G pitch class. The pitch
    /// classes of the chord will be [G, C, E] instead of [C, E, G]:
    /// 
    /// ```rust
    /// use musictools::chord::Chord;
    /// use musictools::common::TriadQuality;
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let mut chord = Chord::from_triad(PitchClasses::C, TriadQuality::Major);
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

    /// Returns a [`PitchClass`] representing the pitch class corresponding to
    /// the tonic or root pitch class of the current chord.
    pub fn get_tonic(&self) -> PitchClass {
        return self.tonic;
    }

    /// Returns a vector of [`PitchClass`] corresponding to the pitch classes
    /// in the current chord. Note that this representation of a chord is not
    /// optimal because it makes it impossible to distinguish the difference
    /// between an interval less than an octave and any interval larger than
    /// an octave.
    pub fn get_pitch_classes(&self) -> Vec<PitchClass> {
        let mut pitch_classes: Vec<PitchClass> = Vec::new();
        let intervals = self.get_intervals();
        for interval in intervals {
            let current_semitone = interval.get_value() % 12;
            let current_pitch_class = self.tonic.get_offset(current_semitone as i8, true);
            pitch_classes.push(current_pitch_class);
        }
        return pitch_classes;
    }

    /// Returns a vector of consecutive [`Note`] objects which contain the
    /// pitch classes and octaves of each note in the chord, given a starting
    /// octave.
    /// 
    /// # Parameters
    /// 
    /// - `starting_octave`: A positive integer representing the octave where
    /// the tonic will be placed. All subsequent notes will be placed either
    /// on this octave or on a higher one depending on the values of their
    /// intervals.
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
    /// let mut chord = Chord::from_triad(PitchClasses::G, TriadQuality::Major);
    /// let notes = chord.to_notes(4);
    /// ```
    pub fn to_notes(&self, starting_octave: u8) -> Vec<Note> {
        let mut notes: Vec<Note> = Vec::new();
        let intervals = self.get_intervals();
        for interval in intervals {
            let current_octave = starting_octave + (self.tonic.get_value() + interval.get_value()) / 12;
            let current_semitone = interval.get_value() % 12;
            let current_pitch_class = self.tonic.get_offset(current_semitone as i8, true);
            let current_note = Note::from(current_pitch_class, current_octave);
            notes.push(current_note);
        }
        return notes;
    }
}