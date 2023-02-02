#[derive(Copy, Clone)]

/// A structure which is used to hold the exact representation of a fraction.
/// Fractions are used in this library to precisely represent time signatures
/// and the durations of beats. These fractions are not simplified when they
/// are stored.
pub struct Fraction {
    numerator: u8,
    denominator: u8
}

impl Fraction {
    /// Creates a new fraction with a given numerator and denominator.
    /// 
    /// # Parameters
    /// 
    /// - `numerator`: A positive integer representing the numerator of the
    /// fraction.
    /// - `denominator`: A positive integer representing the denominator of
    /// the fraction.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use musictools::common::Fraction;
    /// 
    /// let one_half = Fraction::new(1, 2);
    /// ```
    pub fn new(numerator: u8, denominator: u8) -> Fraction {
        return Fraction {
            numerator,
            denominator
        }
    }

    /// Returns the numerator or top half of the fraction.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use musictools::common::Fraction;
    /// 
    /// let five_sevenths = Fraction::new(5, 7);
    /// let five = five_sevenths.get_numerator();
    /// println!("{five}");
    /// ```
    pub fn get_numerator(&self) -> u8 {
        return self.numerator;
    }

    /// Returns the denominator or bottom half of the fraction.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use musictools::common::Fraction;
    /// 
    /// let five_sevenths = Fraction::new(5, 7);
    /// let seven = five_sevenths.get_denominator();
    /// println!("{seven}");
    /// ```
    pub fn get_denominator(&self) -> u8 {
        return self.denominator;
    }

    /// Returns the value of the fraction as a floating point number.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use musictools::common::Fraction;
    /// 
    /// let two_and_a_half = Fraction::new(5, 2);
    /// let float_value = two_and_a_half.get_as_float();
    /// println!("{float_value}");
    /// ```
    pub fn get_as_float(&self) -> f32 {
        return self.numerator as f32 / self.denominator as f32;
    }

    /// Returns a new fraction with a simplified numerator and denominator.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use musictools::common::Fraction;
    /// 
    /// let two_quarters = Fraction::new(2, 4);
    /// let one_half = two_quarters.get_simplified();
    /// ```
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

/// This enum contains representations for the different modes or types of
/// musical scales that can be distinguished or generated by the library.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ScaleType {
    /// The scale of the Ionian mode, which is the first mode and is the same
    /// as the major scale.
    Ionian,
    /// The scale of the Dorian mode, which is the second mode and is equal to
    /// natural minor scale with a major sixth instead of a minor sixth.
    Dorian,
    /// The scale of the Phrygian mode, which is the third mode and is equal
    /// to the natural minor scale with a minor second instead of a major
    /// second.
    Phrygian,
    /// The scale of the Lydian mode, which is the fourth mode and is equal to
    /// the major scale with an augmented fourth instead of a perfect fourth.
    Lydian,
    /// The scale of the Mixolydian mode, which is the fifth mode and is equal
    /// to the major scale with a minor seventh instead of a major seventh.
    Mixolydian,
    /// The scale of the Aeolian mode, which is the sixth mode and is the same
    /// as the natural minor scale.
    Aeolian,
    /// The scale of the Locrian mode, which is the seventh mode and is equal
    /// to the natural minor scale with a minor second instead of a major 
    /// second and a diminished fifth instead of a perfect fifth. 
    Locrian,
    /// The major scale, which is the same as the Ionian mode.
    Major,
    /// The modern minor scale, which differs from the natural minor scale and
    /// the Aeolian mode only in that it's fifth diatonic chord is major
    /// instead of minor.
    Minor,
    /// The natural minor scale, which is the same as the Aeolian mode.
    NaturalMinor,
    /// The descending melodic minor scale, which is the same as the natural
    /// minor scale but is intended to be used when playing the melodic minor
    /// scale in a descending manner.
    DescendingMelodicMinor,
    /// The ascending melodic minor scale, which is equal to the natural minor
    /// scale with a major sixth and major seventh, and is intended to be used
    /// when playing the melodic minor scale in an ascending manner.
    AscendingMelodicMinor,
    /// The harmonic minor scale, which is equal to the natural minor scale
    /// with a major seventh instead of a minor seventh.
    HarmonicMinor,
    /// The Phrygian Dominant scale, which is the equal to the Phrygian scale
    /// with a major third instead of a minor third.
    PhrygianDominant,
    /// The whole tone scale, which is a hexatonic scale where each tone or
    /// pitch class is separated by a whole note or two semitones.
    Whole,
    /// The chromatic scale, which consists of all twelve pitch classes
    /// separated by a semitone.
    Chromatic 
}

/// This enum contains representations for the different types of triads that
/// can be distinguished or generated by the library.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TriadQuality {
    /// A major triad, which consists of the tonic, a major third and a
    /// perfect fifth.
    Major,
    /// A minor triad, which consists of the tonic, a minor third and a
    /// perfect fifth.
    Minor,
    /// A suspended second triad, which consists of the tonic, a major second
    /// and a perfect fifth.
    Sus2,
    /// A suspended fourth triad, which consists of the tonic, a perfect
    /// fourth and a perfect fifth.
    Sus4,
    /// An augmented triad, which consists of the tonic, a major third and an
    /// augmented fifth.
    Augmented,
    /// A diminished triad, which consists of the tonic, a minor third and a
    /// diminished fifth.
    Diminished
}

/// This enum is used to represent the quality of a tone or a pentatonic.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Quality {
    /// Corresponds to no quality.
    None,
    /// Corresponds to a major quality.
    Major,
    /// Corresponds to a minor quality.
    Minor
}

fn gcd(a: u8, b: u8) -> u8 {
    return if b == 0 { a } else { gcd(b, a % b) };
}