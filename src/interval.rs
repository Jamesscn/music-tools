use std::cmp::Ordering;
use std::fmt;
use std::hash::Hash;

pub trait Interval {
    fn get_value(&self) -> usize;
}

#[derive(Copy, Clone, Debug)]
pub struct TwelveToneInterval {
    semitones: usize,
    full_name: Option<&'static str>,
    short_name: Option<&'static str>,
}

impl TwelveToneInterval {
    /// Creates an interval given its size in semitones.
    ///
    /// # Parameters
    ///
    /// - `semitones`: The size of the interval in semitones.
    pub fn new(semitones: impl Into<usize>) -> Self {
        let semitones = semitones.into();
        if semitones < INTERVALS.len() {
            return INTERVALS[semitones];
        }
        Self {
            semitones,
            full_name: None,
            short_name: None,
        }
    }

    /// Returns a positive integer representing the number of semitones held by the interval.
    pub fn get_semitones(&self) -> usize {
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
}

impl PartialEq for TwelveToneInterval {
    fn eq(&self, other: &Self) -> bool {
        self.semitones == other.semitones
    }
}

impl Eq for TwelveToneInterval {}

impl PartialOrd for TwelveToneInterval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TwelveToneInterval {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_semitones().cmp(&other.get_semitones())
    }
}

impl Hash for TwelveToneInterval {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_semitones().hash(state);
    }
}

impl fmt::Display for TwelveToneInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.full_name {
            Some(name) => write!(f, "{}", name),
            None => write!(f, "Interval of {} semitones", self.semitones),
        }
    }
}

/*
//TODO
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
*/

/// The interval between two identical notes in the twelve tone system.
pub const PERFECT_UNISON: TwelveToneInterval = TwelveToneInterval {
    semitones: 0,
    full_name: Some("Perfect Unison"),
    short_name: Some("P1"),
};
/// The interval between two notes separated a semitone in the twelve tone system.
pub const MINOR_SECOND: TwelveToneInterval = TwelveToneInterval {
    semitones: 1,
    full_name: Some("Minor Second"),
    short_name: Some("m2"),
};
/// The interval between two notes separated by two semitones in the twelve tone system.
pub const MAJOR_SECOND: TwelveToneInterval = TwelveToneInterval {
    semitones: 2,
    full_name: Some("Major Second"),
    short_name: Some("M2"),
};
/// The interval between two notes separated by three semitones in the twelve tone system.
pub const MINOR_THIRD: TwelveToneInterval = TwelveToneInterval {
    semitones: 3,
    full_name: Some("Minor Third"),
    short_name: Some("m3"),
};
/// The interval between two notes separated by four semitones in the twelve tone system.
pub const MAJOR_THIRD: TwelveToneInterval = TwelveToneInterval {
    semitones: 4,
    full_name: Some("Major Third"),
    short_name: Some("M3"),
};
/// The interval between two notes separated by five semitones in the twelve tone system.
pub const PERFECT_FOURTH: TwelveToneInterval = TwelveToneInterval {
    semitones: 5,
    full_name: Some("Perfect Fourth"),
    short_name: Some("P4"),
};
/// The interval between two notes separated by six semitones, which is also equivalent to the
/// tritone and the augmented fourth in the twelve tone system.
pub const DIMINISHED_FIFTH: TwelveToneInterval = TwelveToneInterval {
    semitones: 6,
    full_name: Some("Diminished Fifth"),
    short_name: Some("d5"),
};
/// The interval between two notes separated by six semitones, which is also equivalent to the
/// diminished fifth and the augmented fourth in the twelve tone system.
pub const TRITONE: TwelveToneInterval = TwelveToneInterval {
    semitones: 6,
    full_name: Some("Tritone"),
    short_name: Some("TT"),
};
/// The interval between two notes separated by six semitones, which is also equivalent to the
/// tritone and the diminished fifth in the twelve tone system.
pub const AUGMENTED_FOURTH: TwelveToneInterval = TwelveToneInterval {
    semitones: 6,
    full_name: Some("Augmented Fourth"),
    short_name: Some("A4"),
};
/// The interval between two notes separated by seven semitones in the twelve tone system.
pub const PERFECT_FIFTH: TwelveToneInterval = TwelveToneInterval {
    semitones: 7,
    full_name: Some("Perfect Fifth"),
    short_name: Some("P5"),
};
/// The interval between two notes separated by eight semitones in the twelve tone system.
pub const MINOR_SIXTH: TwelveToneInterval = TwelveToneInterval {
    semitones: 8,
    full_name: Some("Minor Sixth"),
    short_name: Some("m6"),
};
/// The interval between two notes separated by nine semitones in the twelve tone system.
pub const MAJOR_SIXTH: TwelveToneInterval = TwelveToneInterval {
    semitones: 9,
    full_name: Some("Major Sixth"),
    short_name: Some("M6"),
};
/// The interval between two notes separated by ten semitones in the twelve tone system.
pub const MINOR_SEVENTH: TwelveToneInterval = TwelveToneInterval {
    semitones: 10,
    full_name: Some("Minor Seventh"),
    short_name: Some("m7"),
};
/// The interval between two notes separated by eleven semitones in the twelve tone system.
pub const MAJOR_SEVENTH: TwelveToneInterval = TwelveToneInterval {
    semitones: 11,
    full_name: Some("Major Seventh"),
    short_name: Some("M7"),
};
/// The interval between two notes separated by twelve semitones or an octave in the twelve tone
/// system.
pub const PERFECT_OCTAVE: TwelveToneInterval = TwelveToneInterval {
    semitones: 12,
    full_name: Some("Perfect Octave"),
    short_name: Some("P8"),
};
/// The interval between two notes separated by thirteen semitones in the twelve tone system.
pub const MINOR_NINTH: TwelveToneInterval = TwelveToneInterval {
    semitones: 13,
    full_name: Some("Minor Ninth"),
    short_name: Some("m9"),
};
/// The interval between two notes separated by fourteen semitones in the twelve tone system.
pub const MAJOR_NINTH: TwelveToneInterval = TwelveToneInterval {
    semitones: 14,
    full_name: Some("Major Ninth"),
    short_name: Some("M9"),
};
/// The interval between two notes separated by fifteen semitones in the twelve tone system.
pub const MINOR_TENTH: TwelveToneInterval = TwelveToneInterval {
    semitones: 15,
    full_name: Some("Minor Tenth"),
    short_name: Some("m10"),
};
/// The interval between two notes separated by sixteen semitones in the twelve tone system.
pub const MAJOR_TENTH: TwelveToneInterval = TwelveToneInterval {
    semitones: 16,
    full_name: Some("Major Tenth"),
    short_name: Some("M10"),
};
/// The interval between two notes separated by seventeen semitones in the twelve tone system.
pub const PERFECT_ELEVENTH: TwelveToneInterval = TwelveToneInterval {
    semitones: 17,
    full_name: Some("Perfect Eleventh"),
    short_name: Some("P11"),
};
/// The interval between two notes separated by eighteen semitones, which is also equivalent to
/// the augmented eleventh in the twelve tone system.
pub const DIMINISHED_TWELFTH: TwelveToneInterval = TwelveToneInterval {
    semitones: 18,
    full_name: Some("Diminished Twelfth"),
    short_name: Some("d12"),
};
/// The interval between two notes separated by eighteen semitones, which is also equivalent to
/// the diminished twelfth in the twelve tone system.
pub const AUGMENTED_ELEVENTH: TwelveToneInterval = TwelveToneInterval {
    semitones: 18,
    full_name: Some("Augmented Eleventh"),
    short_name: Some("A11"),
};
/// The interval between two notes separated by nineteen semitones in the twelve tone system.
pub const PERFECT_TWELFTH: TwelveToneInterval = TwelveToneInterval {
    semitones: 19,
    full_name: Some("Perfect Twelfth"),
    short_name: Some("P12"),
};
/// The interval between two notes separated by twenty semitones in the twelve tone system.
pub const MINOR_THIRTEENTH: TwelveToneInterval = TwelveToneInterval {
    semitones: 20,
    full_name: Some("Minor Thirteenth"),
    short_name: Some("m13"),
};
/// The interval between two notes separated by twenty one semitones in the twelve tone system.
pub const MAJOR_THIRTEENTH: TwelveToneInterval = TwelveToneInterval {
    semitones: 21,
    full_name: Some("Major Thirteenth"),
    short_name: Some("M13"),
};
/// The interval between two notes separated by twenty two semitones in the twelve tone system.
pub const MINOR_FOURTEENTH: TwelveToneInterval = TwelveToneInterval {
    semitones: 22,
    full_name: Some("Minor Fourteenth"),
    short_name: Some("m14"),
};
/// The interval between two notes separated by twenty three semitones in the twelve tone system.
pub const MAJOR_FOURTEENTH: TwelveToneInterval = TwelveToneInterval {
    semitones: 23,
    full_name: Some("Major Fourteenth"),
    short_name: Some("M14"),
};
/// The interval between two notes separated by twenty four semitones or two octaves in the twelve
/// tone system.
pub const PERFECT_FIFTEENTH: TwelveToneInterval = TwelveToneInterval {
    semitones: 24,
    full_name: Some("Perfect Fifteenth"),
    short_name: Some("P15"),
};

const INTERVALS: [TwelveToneInterval; 25] = [
    PERFECT_UNISON,
    MINOR_SECOND,
    MAJOR_SECOND,
    MINOR_THIRD,
    MAJOR_THIRD,
    PERFECT_FOURTH,
    TRITONE,
    PERFECT_FIFTH,
    MINOR_SIXTH,
    MAJOR_SIXTH,
    MINOR_SEVENTH,
    MAJOR_SEVENTH,
    PERFECT_OCTAVE,
    MINOR_NINTH,
    MAJOR_NINTH,
    MINOR_TENTH,
    MAJOR_TENTH,
    PERFECT_ELEVENTH,
    AUGMENTED_ELEVENTH,
    PERFECT_TWELFTH,
    MINOR_THIRTEENTH,
    MAJOR_THIRTEENTH,
    MINOR_FOURTEENTH,
    MAJOR_FOURTEENTH,
    PERFECT_FIFTEENTH,
];

impl Interval for TwelveToneInterval {
    fn get_value(&self) -> usize {
        self.semitones
    }
}
