/// A structure which is used to hold the exact representation of a fraction.
/// Fractions are used in this library to precisely represent time signatures
/// and the durations of beats. These fractions are not simplified when they
/// are stored.
#[derive(Copy, Clone, Debug)]
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
    pub const fn new(numerator: u8, denominator: u8) -> Fraction {
        Fraction {
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
        self.numerator
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
        self.denominator
    }

    /// Returns the value of the fraction as a floating point number. This can
    /// panic if the denominator is zero.
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
        self.numerator as f32 / self.denominator as f32
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
        Fraction {
            numerator: self.numerator / common_factor,
            denominator: self.denominator / common_factor
        }
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        let left_simplified = self.get_simplified();
        let right_simplified = other.get_simplified();
        left_simplified.numerator == right_simplified.numerator && left_simplified.denominator == right_simplified.denominator
    }
}

/// The beat structure is the same as a fraction but used to keep track of the
/// duration of a rhythmic beat with respect to the time signature.
pub type Beat = Fraction;

impl Beat {
    /// The duration corresponding to a whole note.
    pub const WHOLE: Beat = Beat::new(1, 1);
    /// The duration corresponding to a half note.
    pub const HALF: Beat = Beat::new(1, 2);
    /// The duration corresponding to a quarter note.
    pub const QUARTER: Beat = Beat::new(1, 4);
    /// The duration corresponding to an eighth note.
    pub const EIGHTH: Beat = Beat::new(1, 8);
    /// The duration corresponding to a sixteenth note.
    pub const SIXTEENTH: Beat = Beat::new(1, 16);
    /// The duration corresponding to a thirty-second note.
    pub const THIRTYSECOND: Beat = Beat::new(1, 32);
    /// The duration corresponding to a dotted whole note.
    pub const WHOLE_DOTTED: Beat = Beat::new(3, 2);
    /// The duration corresponding to a dotted half note.
    pub const HALF_DOTTED: Beat = Beat::new(3, 4);
    /// The duration corresponding to a dotted quarter note.
    pub const QUARTER_DOTTED: Beat = Beat::new(3, 8);
    /// The duration corresponding to a dotted eighth note.
    pub const EIGHTH_DOTTED: Beat = Beat::new(3, 16);
    /// The duration corresponding to a dotted sixteenth note.
    pub const SIXTEENTH_DOTTED: Beat = Beat::new(3, 32);
    /// The duration corresponding to a dotted thirty-second note.
    pub const THIRTYSECOND_DOTTED: Beat = Beat::new(3, 64);
}

/// This enum contains representations for the different modes or types of
/// musical scales that can be distinguished or generated by the library.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
    /// A nonatonic blues scale, which is derived from the major scale with an
    /// added flat third and an added flat seventh of the key.
    NonatonicBlues,
    /// The major blues scale, which is a hexatonic scale derived from the
    /// major pentatonic scale with an added flat third of the key.
    MajorBlues,
    /// The minor blues scale, which is a hexatonic scale derived from the
    /// minor pentatonic scale with an added flat fifth of the key.
    MinorBlues,
    /// The whole tone scale, which is a hexatonic scale where each tone or
    /// pitch class is separated by a whole note or two semitones.
    Whole,
    /// The chromatic scale, which consists of all twelve pitch classes
    /// separated by a semitone.
    Chromatic
}

/// This enum contains representations for the different types of triads that
/// can be distinguished or generated by the library.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
    /// An augmented triad, which consists of the tonic, a major third and a
    /// minor sixth.
    Augmented,
    /// A diminished triad, which consists of the tonic, a minor third and a
    /// diminished fifth.
    Diminished
}

/// This enum is used to represent the type of a pentatonic.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PentatonicType {
    /// Corresponds to no pentatonic.
    None,
    /// Corresponds to a major pentatonic.
    Major,
    /// Corresponds to a minor pentatonic.
    Minor
}

/// Given a letter from A to G and an offset, this function returns the
/// letter at a given offset from the provided letter, or [`None`] if the
/// letter provided was invalid.
/// 
/// # Parameters
/// 
/// - `letter`: A [`char`] with the letter to offset.
/// - `offset`: A positive or negative integer to offset `letter` by.
/// 
/// # Examples
/// 
/// ```rust
/// use musictools::common::get_letter_at_offset;
/// 
/// let positive_offset = get_letter_at_offset('F', 2).unwrap();
/// let negative_offset = get_letter_at_offset('F', -2).unwrap();
/// println!("F + 2 = {positive_offset}, F - 2 = {negative_offset}");
/// ```
pub fn get_letter_at_offset(letter: char, offset: i8) -> Option<char> {
    const LETTERS: [char; 7] = ['A', 'B', 'C', 'D', 'E', 'F', 'G'];
    let letter_option = LETTERS.iter().position(|&x| x == letter);
    letter_option.map(|letter_index| LETTERS[(letter_index as i8 + (offset % 7)).rem_euclid(7) as usize])
}

fn gcd(a: u8, b: u8) -> u8 {
    if b == 0 { a } else { gcd(b, a % b) }
}