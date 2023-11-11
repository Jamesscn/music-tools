use crate::chord::Chord;
use crate::note::Note;
use crate::scale::Scale;
use std::cmp::Ordering;
use std::fmt;
use std::hash::Hash;

/// A structure which is used to represent the interval between two notes.
#[derive(Copy, Clone, Debug, Eq)]
pub struct Interval {
    semitones: u64,
    full_name: Option<&'static str>,
    short_name: Option<&'static str>,
}

impl Interval {
    /// Creates an interval given its size in semitones.
    ///
    /// # Parameters
    ///
    /// - `semitones`: The size of the interval in semitones.
    pub fn new(semitones: impl Into<u64>) -> Self {
        let numeric_value = semitones.into();
        let index = numeric_value as usize;
        if index < INTERVALS.len() {
            return INTERVALS[index];
        }
        Self {
            semitones: numeric_value,
            full_name: None,
            short_name: None,
        }
    }

    /// Returns the interval between two notes.
    ///
    /// # Parameters
    ///
    /// - `first`: A [`Note`] representing the first note.
    /// - `second`: A [`Note`] representing the second note.
    pub fn between_notes(first: Note, second: Note) -> Self {
        let first_value = first.get_value();
        let second_value = second.get_value();
        let difference: u16 = if first_value <= second_value {
            (second_value - first_value) as u16
        } else {
            (first_value - second_value) as u16
        };
        Self::new(difference)
    }

    /// Returns a positive integer representing the number of semitones held by the interval.
    pub fn get_semitones(&self) -> u64 {
        self.semitones
    }

    /// Returns the full name of the interval if it exists, such as Perfect Unison or Diminished
    /// Fifth.
    pub fn get_name(&self) -> Option<&'static str> {
        self.full_name
    }

    /// Returns an abbreviated name for the interval if it exists, such as P1 or m6.
    pub fn get_short_name(&self) -> Option<&'static str> {
        self.short_name
    }

    // Intervals

    /// The interval between two identical notes.
    pub const PERFECT_UNISON: Self = Self {
        semitones: 0,
        full_name: Some("Perfect Unison"),
        short_name: Some("P1"),
    };
    /// The interval between two notes separated a semitone.
    pub const MINOR_SECOND: Self = Self {
        semitones: 1,
        full_name: Some("Minor Second"),
        short_name: Some("m2"),
    };
    /// The interval between two notes separated by two semitones.
    pub const MAJOR_SECOND: Self = Self {
        semitones: 2,
        full_name: Some("Major Second"),
        short_name: Some("M2"),
    };
    /// The interval between two notes separated by three semitones.
    pub const MINOR_THIRD: Self = Self {
        semitones: 3,
        full_name: Some("Minor Third"),
        short_name: Some("m3"),
    };
    /// The interval between two notes separated by four semitones.
    pub const MAJOR_THIRD: Self = Self {
        semitones: 4,
        full_name: Some("Major Third"),
        short_name: Some("M3"),
    };
    /// The interval between two notes separated by five semitones.
    pub const PERFECT_FOURTH: Self = Self {
        semitones: 5,
        full_name: Some("Perfect Fourth"),
        short_name: Some("P4"),
    };
    /// The interval between two notes separated by six semitones, which is also equivalent to the
    /// tritone and the augmented fourth.
    pub const DIMINISHED_FIFTH: Self = Self {
        semitones: 6,
        full_name: Some("Diminished Fifth"),
        short_name: Some("d5"),
    };
    /// The interval between two notes separated by six semitones, which is also equivalent to the
    /// diminished fifth and the augmented fourth.
    pub const TRITONE: Self = Self {
        semitones: 6,
        full_name: Some("Tritone"),
        short_name: Some("TT"),
    };
    /// The interval between two notes separated by six semitones, which is also equivalent to the
    /// tritone and the diminished fifth.
    pub const AUGMENTED_FOURTH: Self = Self {
        semitones: 6,
        full_name: Some("Augmented Fourth"),
        short_name: Some("A4"),
    };
    /// The interval between two notes separated by seven semitones.
    pub const PERFECT_FIFTH: Self = Self {
        semitones: 7,
        full_name: Some("Perfect Fifth"),
        short_name: Some("P5"),
    };
    /// The interval between two notes separated by eight semitones.
    pub const AUGMENTED_FIFTH: Self = Self {
        semitones: 8,
        full_name: Some("Augmented Fifth"),
        short_name: Some("A5"),
    };
    /// The interval between two notes separated by eight semitones.
    pub const MINOR_SIXTH: Self = Self {
        semitones: 8,
        full_name: Some("Minor Sixth"),
        short_name: Some("m6"),
    };
    /// The interval between two notes separated by nine semitones.
    pub const MAJOR_SIXTH: Self = Self {
        semitones: 9,
        full_name: Some("Major Sixth"),
        short_name: Some("M6"),
    };
    /// The interval between two notes separated by ten semitones.
    pub const MINOR_SEVENTH: Self = Self {
        semitones: 10,
        full_name: Some("Minor Seventh"),
        short_name: Some("m7"),
    };
    /// The interval between two notes separated by eleven semitones.
    pub const MAJOR_SEVENTH: Self = Self {
        semitones: 11,
        full_name: Some("Major Seventh"),
        short_name: Some("M7"),
    };
    /// The interval between two notes separated by twelve semitones or an octave.
    pub const PERFECT_OCTAVE: Self = Self {
        semitones: 12,
        full_name: Some("Perfect Octave"),
        short_name: Some("P8"),
    };
    /// The interval between two notes separated by thirteen semitones.
    pub const MINOR_NINTH: Self = Self {
        semitones: 13,
        full_name: Some("Minor Ninth"),
        short_name: Some("m9"),
    };
    /// The interval between two notes separated by fourteen semitones.
    pub const MAJOR_NINTH: Self = Self {
        semitones: 14,
        full_name: Some("Major Ninth"),
        short_name: Some("M9"),
    };
    /// The interval between two notes separated by fifteen semitones.
    pub const MINOR_TENTH: Self = Self {
        semitones: 15,
        full_name: Some("Minor Tenth"),
        short_name: Some("m10"),
    };
    /// The interval between two notes separated by sixteen semitones.
    pub const MAJOR_TENTH: Self = Self {
        semitones: 16,
        full_name: Some("Major Tenth"),
        short_name: Some("M10"),
    };
    /// The interval between two notes separated by seventeen semitones.
    pub const PERFECT_ELEVENTH: Self = Self {
        semitones: 17,
        full_name: Some("Perfect Eleventh"),
        short_name: Some("P11"),
    };
    /// The interval between two notes separated by eighteen semitones, which is also equivalent to
    /// the augmented eleventh.
    pub const DIMINISHED_TWELFTH: Self = Self {
        semitones: 18,
        full_name: Some("Diminished Twelfth"),
        short_name: Some("d12"),
    };
    /// The interval between two notes separated by eighteen semitones, which is also equivalent to
    /// the diminished twelfth.
    pub const AUGMENTED_ELEVENTH: Self = Self {
        semitones: 18,
        full_name: Some("Augmented Eleventh"),
        short_name: Some("A11"),
    };
    /// The interval between two notes separated by nineteen semitones.
    pub const PERFECT_TWELFTH: Self = Self {
        semitones: 19,
        full_name: Some("Perfect Twelfth"),
        short_name: Some("P12"),
    };
    /// The interval between two notes separated by twenty semitones.
    pub const MINOR_THIRTEENTH: Self = Self {
        semitones: 20,
        full_name: Some("Minor Thirteenth"),
        short_name: Some("m13"),
    };
    /// The interval between two notes separated by twenty one semitones.
    pub const MAJOR_THIRTEENTH: Self = Self {
        semitones: 21,
        full_name: Some("Major Thirteenth"),
        short_name: Some("M13"),
    };
    /// The interval between two notes separated by twenty two semitones.
    pub const MINOR_FOURTEENTH: Self = Self {
        semitones: 22,
        full_name: Some("Minor Fourteenth"),
        short_name: Some("m14"),
    };
    /// The interval between two notes separated by twenty three semitones.
    pub const MAJOR_FOURTEENTH: Self = Self {
        semitones: 23,
        full_name: Some("Major Fourteenth"),
        short_name: Some("M14"),
    };
    /// The interval between two notes separated by twenty four semitones or two octaves.
    pub const PERFECT_FIFTEENTH: Self = Self {
        semitones: 24,
        full_name: Some("Perfect Fifteenth"),
        short_name: Some("P15"),
    };
}

impl Default for Interval {
    fn default() -> Self {
        Interval::PERFECT_OCTAVE
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.semitones == other.semitones
    }
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_semitones().cmp(&other.get_semitones())
    }
}

impl Hash for Interval {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_semitones().hash(state);
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.full_name {
            Some(name) => write!(f, "{}", name),
            None => write!(f, "Interval of {} semitones", self.semitones),
        }
    }
}

impl From<Chord> for Vec<Interval> {
    fn from(value: Chord) -> Self {
        value.get_intervals()
    }
}

impl From<Scale> for Vec<Interval> {
    fn from(value: Scale) -> Self {
        value.get_intervals()
    }
}

const INTERVALS: [Interval; 25] = [
    Interval::PERFECT_UNISON,
    Interval::MINOR_SECOND,
    Interval::MAJOR_SECOND,
    Interval::MINOR_THIRD,
    Interval::MAJOR_THIRD,
    Interval::PERFECT_FOURTH,
    Interval::TRITONE,
    Interval::PERFECT_FIFTH,
    Interval::MINOR_SIXTH,
    Interval::MAJOR_SIXTH,
    Interval::MINOR_SEVENTH,
    Interval::MAJOR_SEVENTH,
    Interval::PERFECT_OCTAVE,
    Interval::MINOR_NINTH,
    Interval::MAJOR_NINTH,
    Interval::MINOR_TENTH,
    Interval::MAJOR_TENTH,
    Interval::PERFECT_ELEVENTH,
    Interval::DIMINISHED_TWELFTH,
    Interval::PERFECT_TWELFTH,
    Interval::MINOR_THIRTEENTH,
    Interval::MAJOR_THIRTEENTH,
    Interval::MINOR_FOURTEENTH,
    Interval::MAJOR_FOURTEENTH,
    Interval::PERFECT_FIFTEENTH,
];
