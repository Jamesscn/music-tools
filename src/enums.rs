#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ScaleType {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
    Major,
    Minor,
    NaturalMinor,
    DescendingMelodicMinor,
    AscendingMelodicMinor,
    HarmonicMinor,
    PhrygianDominant,
    Whole,
    Chromatic 
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ChordQuality {
    Major,
    Minor,
    Sus2,
    Sus4,
    Augmented,
    Diminished
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Pentatonic {
    None,
    Major,
    Minor
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Seventh {
    None,
    Major,
    Minor
}