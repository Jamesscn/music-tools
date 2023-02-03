use std::cmp::Ordering;

#[derive(Copy, Clone)]
pub struct Interval {
    value: i8,
    full_name: Option<&'static str>,
    short_name: Option<&'static str>
}

impl Interval {
    pub fn from(value: i8) -> Interval {
        if value >= 0 && value < INTERVALS.len() as i8 {
            return INTERVALS[value as usize];
        }
        return Interval {
            value,
            full_name: None,
            short_name: None
        }
    }

    pub fn get_value(&self) -> i8 {
        return self.value;
    }

    pub fn get_name(&self) -> Option<&'static str> {
        return self.full_name;
    }

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

pub struct Intervals;

impl Intervals {
    pub const PERFECT_UNISON: Interval = Interval {
        value: 0,
        full_name: Some("Perfect Unison"),
        short_name: Some("P1")
    };
    pub const MINOR_SECOND: Interval = Interval {
        value: 1,
        full_name: Some("Minor Second"),
        short_name: Some("m2")
    };
    pub const MAJOR_SECOND: Interval = Interval {
        value: 2,
        full_name: Some("Major Second"),
        short_name: Some("M2")
    };
    pub const MINOR_THIRD: Interval = Interval {
        value: 3,
        full_name: Some("Minor Third"),
        short_name: Some("m3")
    };
    pub const MAJOR_THIRD: Interval = Interval {
        value: 4,
        full_name: Some("Major Third"),
        short_name: Some("M3")
    };
    pub const PERFECT_FOURTH: Interval = Interval {
        value: 5,
        full_name: Some("Perfect Fourth"),
        short_name: Some("P4")
    };
    pub const DIMINISHED_FIFTH: Interval = Interval {
        value: 6,
        full_name: Some("Diminished Fifth"),
        short_name: Some("d5")
    };
    pub const TRITONE: Interval = Interval {
        value: 6,
        full_name: Some("Tritone"),
        short_name: Some("TT")
    };
    pub const AUGMENTED_FOURTH: Interval = Interval {
        value: 6,
        full_name: Some("Augmented Fourth"),
        short_name: Some("A4")
    };
    pub const PERFECT_FIFTH: Interval = Interval {
        value: 7,
        full_name: Some("Perfect Fifth"),
        short_name: Some("P5")
    };
    pub const MINOR_SIXTH: Interval = Interval {
        value: 8,
        full_name: Some("Minor Sixth"),
        short_name: Some("m6")
    };
    pub const MAJOR_SIXTH: Interval = Interval {
        value: 9,
        full_name: Some("Major Sixth"),
        short_name: Some("M6")
    };
    pub const MINOR_SEVENTH: Interval = Interval {
        value: 10,
        full_name: Some("Minor Seventh"),
        short_name: Some("m7")
    };
    pub const MAJOR_SEVENTH: Interval = Interval {
        value: 11,
        full_name: Some("Major Seventh"),
        short_name: Some("M7")
    };
    pub const PERFECT_OCTAVE: Interval = Interval {
        value: 12,
        full_name: Some("Perfect Octave"),
        short_name: Some("P8")
    };
    pub const MINOR_NINTH: Interval = Interval {
        value: 13,
        full_name: Some("Minor Ninth"),
        short_name: Some("m9")
    };
    pub const MAJOR_NINTH: Interval = Interval {
        value: 14,
        full_name: Some("Major Ninth"),
        short_name: Some("M9")
    };
    pub const MINOR_TENTH: Interval = Interval {
        value: 15,
        full_name: Some("Minor Tenth"),
        short_name: Some("m10")
    };
    pub const MAJOR_TENTH: Interval = Interval {
        value: 16,
        full_name: Some("Major Tenth"),
        short_name: Some("M10")
    };
    pub const PERFECT_ELEVENTH: Interval = Interval {
        value: 17,
        full_name: Some("Perfect Eleventh"),
        short_name: Some("P11")
    };
    pub const DIMINISHED_TWELFTH: Interval = Interval {
        value: 18,
        full_name: Some("Diminished Twelfth"),
        short_name: Some("d12")
    };
    pub const AUGMENTED_ELEVENTH: Interval = Interval {
        value: 18,
        full_name: Some("Augmented Eleventh"),
        short_name: Some("A11")
    };
    pub const PERFECT_TWELFTH: Interval = Interval {
        value: 19,
        full_name: Some("Perfect Twelfth"),
        short_name: Some("P12")
    };
    pub const MINOR_THIRTEENTH: Interval = Interval {
        value: 20,
        full_name: Some("Minor Thirteenth"),
        short_name: Some("m13")
    };
    pub const MAJOR_THIRTEENTH: Interval = Interval {
        value: 21,
        full_name: Some("Major Thirteenth"),
        short_name: Some("M13")
    };
    pub const MINOR_FOURTEENTH: Interval = Interval {
        value: 22,
        full_name: Some("Minor Fourteenth"),
        short_name: Some("m14")
    };
    pub const MAJOR_FOURTEENTH: Interval = Interval {
        value: 23,
        full_name: Some("Major Fourteenth"),
        short_name: Some("M14")
    };
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