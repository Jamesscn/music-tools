#[derive(Copy, Clone)]
pub struct Fraction {
    numerator: u8,
    denominator: u8
}

impl Fraction {
    pub fn new(numerator: u8, denominator: u8) -> Fraction {
        return Fraction {
            numerator,
            denominator
        }
    }

    pub fn get_numerator(&self) -> u8 {
        return self.numerator;
    }

    pub fn get_denominator(&self) -> u8 {
        return self.denominator;
    }

    pub fn get_as_float(&self) -> f32 {
        return self.numerator as f32 / self.denominator as f32;
    }

    pub fn get_simplified(&self) -> Fraction {
        let common_factor = gcd(self.numerator, self.denominator);
        return Fraction {
            numerator: self.numerator / common_factor,
            denominator: self.denominator / common_factor
        }
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        let left_simplified = self.get_simplified();
        let right_simplified = other.get_simplified();
        return left_simplified.numerator == right_simplified.numerator && left_simplified.denominator == right_simplified.denominator;
    }
}

impl Eq for Fraction {}

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
pub enum TriadQuality {
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

fn gcd(a: u8, b: u8) -> u8 {
    return if b == 0 { a } else { gcd(b, a % b) };
}