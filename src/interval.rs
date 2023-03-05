use std::cmp::Ordering;

#[derive(Copy, Clone, Debug)]
/// A structure which is used to represent the interval between two notes.
pub struct Interval {
    value: u8,
    full_name: Option<&'static str>,
    short_name: Option<&'static str>
}

impl Interval {
    /// Constructs an interval given a positive integer representing the value
    /// of the interval or the distance between two notes.
    pub fn from(value: u8) -> Interval {
        let index = value as usize;
        if index < INTERVALS.len() {
            return INTERVALS[index];
        }
        return Interval {
            value,
            full_name: None,
            short_name: None
        }
    }

    /// Returns a positive integer representing the value of the interval.
    pub fn get_value(&self) -> u8 {
        return self.value;
    }

    /// Returns the full name of the interval if it exists, such as Perfect
    /// Unison or Diminished Fifth.
    pub fn get_name(&self) -> Option<&'static str> {
        return self.full_name;
    }

    /// Returns an abbreviated name for the interval if it exists, such as P1
    /// or m6.
    pub fn get_short_name(&self) -> Option<&'static str> {
        return self.short_name;
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        return self.value == other.value;
    }
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

/// A structure containing common intervals.
pub struct Intervals;

impl Intervals {
    /// The interval between two identical notes.
    pub const PERFECT_UNISON: Interval = Interval {
        value: 0,
        full_name: Some("Perfect Unison"),
        short_name: Some("P1")
    };
    /// The interval between two notes separated a semitone.
    pub const MINOR_SECOND: Interval = Interval {
        value: 1,
        full_name: Some("Minor Second"),
        short_name: Some("m2")
    };
    /// The interval between two notes separated by two semitones.
    pub const MAJOR_SECOND: Interval = Interval {
        value: 2,
        full_name: Some("Major Second"),
        short_name: Some("M2")
    };
    /// The interval between two notes separated by three semitones.
    pub const MINOR_THIRD: Interval = Interval {
        value: 3,
        full_name: Some("Minor Third"),
        short_name: Some("m3")
    };
    /// The interval between two notes separated by four semitones.
    pub const MAJOR_THIRD: Interval = Interval {
        value: 4,
        full_name: Some("Major Third"),
        short_name: Some("M3")
    };
    /// The interval between two notes separated by five semitones.
    pub const PERFECT_FOURTH: Interval = Interval {
        value: 5,
        full_name: Some("Perfect Fourth"),
        short_name: Some("P4")
    };
    /// The interval between two notes separated by six semitones, which is
    /// also equivalent to the tritone and the augmented fourth.
    pub const DIMINISHED_FIFTH: Interval = Interval {
        value: 6,
        full_name: Some("Diminished Fifth"),
        short_name: Some("d5")
    };
    /// The interval between two notes separated by six semitones, which is
    /// also equivalent to the diminished fifth and the augmented fourth.
    pub const TRITONE: Interval = Interval {
        value: 6,
        full_name: Some("Tritone"),
        short_name: Some("TT")
    };
    /// The interval between two notes separated by six semitones, which is
    /// also equivalent to the tritone and the diminished fifth.
    pub const AUGMENTED_FOURTH: Interval = Interval {
        value: 6,
        full_name: Some("Augmented Fourth"),
        short_name: Some("A4")
    };
    /// The interval between two notes separated by seven semitones.
    pub const PERFECT_FIFTH: Interval = Interval {
        value: 7,
        full_name: Some("Perfect Fifth"),
        short_name: Some("P5")
    };
    /// The interval between two notes separated by eight semitones.
    pub const MINOR_SIXTH: Interval = Interval {
        value: 8,
        full_name: Some("Minor Sixth"),
        short_name: Some("m6")
    };
    /// The interval between two notes separated by nine semitones.
    pub const MAJOR_SIXTH: Interval = Interval {
        value: 9,
        full_name: Some("Major Sixth"),
        short_name: Some("M6")
    };
    /// The interval between two notes separated by ten semitones.
    pub const MINOR_SEVENTH: Interval = Interval {
        value: 10,
        full_name: Some("Minor Seventh"),
        short_name: Some("m7")
    };
    /// The interval between two notes separated by eleven semitones.
    pub const MAJOR_SEVENTH: Interval = Interval {
        value: 11,
        full_name: Some("Major Seventh"),
        short_name: Some("M7")
    };
    /// The interval between two notes separated by twelve semitones or an
    /// octave.
    pub const PERFECT_OCTAVE: Interval = Interval {
        value: 12,
        full_name: Some("Perfect Octave"),
        short_name: Some("P8")
    };
    /// The interval between two notes separated by thirteen semitones.
    pub const MINOR_NINTH: Interval = Interval {
        value: 13,
        full_name: Some("Minor Ninth"),
        short_name: Some("m9")
    };
    /// The interval between two notes separated by fourteen semitones.
    pub const MAJOR_NINTH: Interval = Interval {
        value: 14,
        full_name: Some("Major Ninth"),
        short_name: Some("M9")
    };
    /// The interval between two notes separated by fifteen semitones.
    pub const MINOR_TENTH: Interval = Interval {
        value: 15,
        full_name: Some("Minor Tenth"),
        short_name: Some("m10")
    };
    /// The interval between two notes separated by sixteen semitones.
    pub const MAJOR_TENTH: Interval = Interval {
        value: 16,
        full_name: Some("Major Tenth"),
        short_name: Some("M10")
    };
    /// The interval between two notes separated by seventeen semitones.
    pub const PERFECT_ELEVENTH: Interval = Interval {
        value: 17,
        full_name: Some("Perfect Eleventh"),
        short_name: Some("P11")
    };
    /// The interval between two notes separated by eighteen semitones, which
    /// is also equivalent to the augmented eleventh.
    pub const DIMINISHED_TWELFTH: Interval = Interval {
        value: 18,
        full_name: Some("Diminished Twelfth"),
        short_name: Some("d12")
    };
    /// The interval between two notes separated by eighteen semitones, which
    /// is also equivalent to the diminished twelfth.
    pub const AUGMENTED_ELEVENTH: Interval = Interval {
        value: 18,
        full_name: Some("Augmented Eleventh"),
        short_name: Some("A11")
    };
    /// The interval between two notes separated by nineteen semitones.
    pub const PERFECT_TWELFTH: Interval = Interval {
        value: 19,
        full_name: Some("Perfect Twelfth"),
        short_name: Some("P12")
    };
    /// The interval between two notes separated by twenty semitones.
    pub const MINOR_THIRTEENTH: Interval = Interval {
        value: 20,
        full_name: Some("Minor Thirteenth"),
        short_name: Some("m13")
    };
    /// The interval between two notes separated by twenty one semitones.
    pub const MAJOR_THIRTEENTH: Interval = Interval {
        value: 21,
        full_name: Some("Major Thirteenth"),
        short_name: Some("M13")
    };
    /// The interval between two notes separated by twenty two semitones.
    pub const MINOR_FOURTEENTH: Interval = Interval {
        value: 22,
        full_name: Some("Minor Fourteenth"),
        short_name: Some("m14")
    };
    /// The interval between two notes separated by twenty three semitones.
    pub const MAJOR_FOURTEENTH: Interval = Interval {
        value: 23,
        full_name: Some("Major Fourteenth"),
        short_name: Some("M14")
    };
    /// The interval between two notes separated by twenty four semitones or
    /// two octaves.
    pub const PERFECT_FIFTEENTH: Interval = Interval {
        value: 24,
        full_name: Some("Perfect Fifteenth"),
        short_name: Some("P15")
    };
}

const INTERVALS: [Interval; 25] = [
    Intervals::PERFECT_UNISON,
    Intervals::MINOR_SECOND,
    Intervals::MAJOR_SECOND,
    Intervals::MINOR_THIRD,
    Intervals::MAJOR_THIRD,
    Intervals::PERFECT_FOURTH,
    Intervals::TRITONE,
    Intervals::PERFECT_FIFTH,
    Intervals::MINOR_SIXTH,
    Intervals::MAJOR_SIXTH,
    Intervals::MINOR_SEVENTH,
    Intervals::MAJOR_SEVENTH,
    Intervals::PERFECT_OCTAVE,
    Intervals::MINOR_NINTH,
    Intervals::MAJOR_NINTH,
    Intervals::MINOR_TENTH,
    Intervals::MAJOR_TENTH,
    Intervals::PERFECT_ELEVENTH,
    Intervals::DIMINISHED_TWELFTH,
    Intervals::PERFECT_TWELFTH,
    Intervals::MINOR_THIRTEENTH,
    Intervals::MAJOR_THIRTEENTH,
    Intervals::MINOR_FOURTEENTH,
    Intervals::MAJOR_FOURTEENTH,
    Intervals::PERFECT_FIFTEENTH
];