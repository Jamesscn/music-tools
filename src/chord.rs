pub use regex::Regex;
pub use crate::common::{ScaleType, TriadQuality, Quality};
pub use crate::pitchclass::PitchClass;
pub use crate::scale::{Scale, get_scale};

/// A structure which holds a chord, which can be any group of pitch classes.
/// This class does not keep track of the octaves of the pitch classes it
/// holds, however it can store the inversion of the chord.
pub struct Chord {
    pitch_classes: Vec<&'static PitchClass>,
    inversion: u8
}

impl Chord {
    /// Creates an empty chord with no pitch classes.
    pub fn new() -> Chord {
        return Chord {
            pitch_classes: Vec::new(),
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
    /// use musictools::scale::Chord;
    /// use musictools::common::TriadQuality;
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let chord = Chord::from_triad(PitchClasses::C, TriadQuality::Major);
    /// ```
    /// 
    /// The following example demonstrates the creation of a B flat sus2 triad:
    /// 
    /// ```rust
    /// use musictools::scale::Chord;
    /// use musictools::common::TriadQuality;
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let chord = Chord::from_triad(PitchClasses::B_FLAT, TriadQuality::Sus2);
    /// ```
    pub fn from_triad(tonic: &'static PitchClass, triad_quality: TriadQuality) -> Chord {
        let major_scale_obj = get_scale(tonic, ScaleType::Major, Quality::None).unwrap();
        let minor_scale_obj = get_scale(tonic, ScaleType::Minor, Quality::None).unwrap();
        let whole_scale_obj = get_scale(tonic, ScaleType::Whole, Quality::None).unwrap();
        let locrian_scale_obj = get_scale(tonic, ScaleType::Locrian, Quality::None).unwrap();
        let major_scale = major_scale_obj.get_pitch_classes();
        let minor_scale = minor_scale_obj.get_pitch_classes();
        let whole_scale = whole_scale_obj.get_pitch_classes();
        let locrian_scale = locrian_scale_obj.get_pitch_classes();
        let pitch_classes: Vec<&'static PitchClass> = match triad_quality {
            TriadQuality::Major => vec![major_scale[0], major_scale[2], major_scale[4]],
            TriadQuality::Minor => vec![minor_scale[0], minor_scale[2], minor_scale[4]],
            TriadQuality::Sus2 => vec![major_scale[0], major_scale[1], major_scale[4]],
            TriadQuality::Sus4 => vec![major_scale[0], major_scale[3], major_scale[4]],
            TriadQuality::Augmented => vec![whole_scale[0], whole_scale[2], whole_scale[4]],
            TriadQuality::Diminished => vec![locrian_scale[0], locrian_scale[2], locrian_scale[4]]
        };
        return Chord {
            pitch_classes,
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
    ///     - An optional accidental `b` which will treat the chord as a flat
    ///     chord, or `#` which will treat the chord as a sharp chord.
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
    /// The following example demonstrates the creation of a major tetrad
    /// seven scale degrees above C, which will be flat, augmented and also
    /// contain a minor seventh:
    /// 
    /// ```rust
    /// use musictools::scale::Chord;
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let chord = Chord::from_numeral(PitchClasses::C, "bVII+7");
    /// ```
    /// 
    /// The following example demonstrates the creation of a minor tetrad two
    /// scale degrees above G sharp, which will be sharp, diminished and also
    /// contain a major seventh:
    /// 
    /// ```rust
    /// use musictools::scale::Chord;
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let chord = Chord::from_numeral(PitchClasses::G_SHARP, "#ii°maj7");
    /// ```
    /// 
    /// The following example demonstrates the creation of a minor triad
    /// three scale degrees above A:
    /// 
    /// ```rust
    /// use musictools::scale::Chord;
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let chord = Chord::from_numeral(PitchClasses::A, "iii");
    /// ```
    pub fn from_numeral(tonic: &'static PitchClass, input_numeral: &str) -> Option<Chord> {
        let numeral_array = ["I", "II", "III", "IV", "V", "VI", "VII"];
        let numeral_regex = Regex::new(r"^(b|\#)?(I|II|III|IV|V|VI|VII|i|ii|iii|iv|v|vi|vii)(°|\+)?(maj7|7)?$").unwrap();
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
        let chord_seventh: Quality;
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
        if seventh == "maj7" {
            chord_seventh = Quality::Major;
        } else if seventh == "7" {
            chord_seventh = Quality::Minor;
        } else {
            chord_seventh = Quality::None;
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
        if accidental == "b" {
            increment = match numeral_value {
                1 => 1,
                2 => 3,
                4 => 6,
                5 => 8,
                6 => 10,
                _ => return None
            };
        } else if accidental == "#" {
            increment = match numeral_value {
                0 => 1,
                1 => 3,
                3 => 6,
                4 => 8,
                5 => 10,
                _ => return None
            };
        }
        let chord_tonic = tonic.get_offset(increment);
        let mut chord = Chord::from_triad(chord_tonic, triad_quality);
        chord.add_seventh(chord_seventh);
        return Some(chord);
    }

    /// Adds a major or minor seventh on top of the current chord.
    /// 
    /// # Parameters
    /// 
    /// - `seventh`: A [`Quality`] representing the quality of the seventh to
    /// add. If it is major, then a major seventh is added. If it is minor, a
    /// minor seventh is added. If it is none, then nothing is added.
    /// 
    /// # Examples
    /// 
    /// The following example demonstrates adding a minor seventh to a C major
    /// triad:
    /// 
    /// ```rust
    /// use musictools::scale::Chord;
    /// use musictools::common::{TriadQuality, Quality};
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let mut chord = Chord::from_triad(PitchClasses::C, TriadQuality::Major);
    /// chord.add_seventh(Quality::Minor);
    /// ```
    pub fn add_seventh(&mut self, seventh: Quality) {
        if seventh == Quality::Major {
            let major_seventh = self.get_tonic().get_offset(11);
            self.pitch_classes.push(major_seventh);
        } else if seventh == Quality::Minor {
            let minor_seventh = self.get_tonic().get_offset(10);
            self.pitch_classes.push(minor_seventh);
        }
    }

    /// Adds a major or minor ninth on top of the current chord.
    /// 
    /// # Parameters
    /// 
    /// - `ninth`: A [`Quality`] representing the quality of the ninth to add.
    /// If it is major, then a major ninth is added. If it is minor, a minor
    /// ninth is added. If it is none, then nothing is added.
    /// 
    /// # Examples
    /// 
    /// The following example demonstrates adding a major ninth to a B minor
    /// triad:
    /// 
    /// ```rust
    /// use musictools::scale::Chord;
    /// use musictools::common::{TriadQuality, Quality};
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let mut chord = Chord::from_triad(PitchClasses::B, TriadQuality::Minor);
    /// chord.add_ninth(Quality::Major);
    /// ```
    pub fn add_ninth(&mut self, ninth: Quality) {
        if ninth == Quality::Major {
            let major_ninth = self.get_tonic().get_offset(14);
            self.pitch_classes.push(major_ninth);
        } else if ninth == Quality::Minor {
            let minor_ninth = self.get_tonic().get_offset(13);
            self.pitch_classes.push(minor_ninth);
        }
    }

    /// Adds a major or minor thirteenth on top of the current chord.
    /// 
    /// # Parameters
    /// 
    /// - `thirteenth`: A [`Quality`] representing the quality of the
    /// thirteenth to add. If it is major, then a major thirteenth is added.
    /// If it is minor, a minor thirteenth is added. If it is none, then
    /// nothing is added.
    /// 
    /// # Examples
    /// 
    /// The following example demonstrates adding a minor thirteenth to an A
    /// minor triad:
    /// 
    /// ```rust
    /// use musictools::scale::Chord;
    /// use musictools::common::{TriadQuality, Quality};
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let mut chord = Chord::from_triad(PitchClasses::A, TriadQuality::Minor);
    /// chord.add_thirteenth(Quality::Minor);
    /// ```
    pub fn add_thirteenth(&mut self, thirteenth: Quality) {
        if thirteenth == Quality::Major {
            let major_thirteenth = self.get_tonic().get_offset(21);
            self.pitch_classes.push(major_thirteenth);
        } else if thirteenth == Quality::Minor {
            let minor_thirteenth = self.get_tonic().get_offset(20);
            self.pitch_classes.push(minor_thirteenth);
        }
    }

    /// Adds a pitch class on top of the current chord.
    /// 
    /// # Parameters
    /// 
    /// - `pitch_class`: A [`PitchClass`] representing the pitch class to add
    /// to the chord.
    /// 
    /// # Examples
    /// 
    /// The following example demonstrates adding an A pitch class on top of a
    /// C major triad:
    /// 
    /// ```rust
    /// use musictools::scale::Chord;
    /// use musictools::common::TriadQuality;
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let mut chord = Chord::from_triad(PitchClasses::C, TriadQuality::Major);
    /// chord.add_pitch_class(PitchClasses::A);
    /// ```
    pub fn add_pitch_class(&mut self, pitch_class: &'static PitchClass) {
        self.pitch_classes.push(pitch_class);
    }

    /// Adds a pitch class on top of the current chord at an offset from the
    /// tonic.
    /// 
    /// # Parameters
    /// 
    /// - `offset`: A positive or negative integer representing the number of
    /// semitones of offset between the tonic and the new pitch class to add.
    /// 
    /// # Examples
    /// 
    /// The following example constructs a B minor triad and adds a pitch
    /// class which is 9 semitones above the tonic:
    /// 
    /// ```rust
    /// use musictools::scale::Chord;
    /// use musictools::common::TriadQuality;
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let mut chord = Chord::from_triad(PitchClasses::B, TriadQuality::Minor);
    /// chord.add_pitch_class_at_offset(9);
    /// ```
    /// 
    /// The following example constructs a G major triad and adds a pitch
    /// class which is 3 semitones below the tonic:
    /// 
    /// ```rust
    /// use musictools::scale::Chord;
    /// use musictools::common::TriadQuality;
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let mut chord = Chord::from_triad(PitchClasses::G, TriadQuality::Major);
    /// chord.add_pitch_class_at_offset(-3);
    /// ```
    pub fn add_pitch_class_at_offset(&mut self, offset: i8) {
        let pitch_class = self.get_tonic().get_offset(offset);
        self.pitch_classes.push(pitch_class);
    }

    /// Returns a vector of [`PitchClass`] objects representing the pitch
    /// classes contained by the current chord, taking into account the
    /// inversion of the chord.
    pub fn get_pitch_classes(&self) -> Vec<&'static PitchClass> {
        let mut pitch_classes = Vec::from(&self.pitch_classes[self.inversion as usize..]);
        let mut second_half = Vec::from(&self.pitch_classes[..self.inversion as usize]);
        pitch_classes.append(&mut second_half);
        return pitch_classes;
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
    /// use musictools::scale::Chord;
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
    /// use musictools::scale::Chord;
    /// use musictools::common::TriadQuality;
    /// use musictools::pitchclass::PitchClasses;
    /// 
    /// let mut chord = Chord::from_triad(PitchClasses::C, TriadQuality::Major);
    /// chord.set_inversion(2);
    /// ```
    pub fn set_inversion(&mut self, inversion: u8) {
        self.inversion = inversion % self.pitch_classes.len() as u8;
    }

    /// Returns a positive integer representing the inversion of the current
    /// chord.
    pub fn get_inversion(&self) -> u8 {
        return self.inversion;
    }

    /// Returns a [`PitchClass`] representing the pitch class corresponding to
    /// the tonic or root pitch class of the current chord.
    pub fn get_tonic(&self) -> &'static PitchClass {
        return self.pitch_classes[self.inversion as usize];
    }
}