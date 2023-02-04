use crate::common::Fraction;

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

/// This structure is used to store a rhythmic pattern or sequence of notes,
/// along with the time signature and beats per minute of the rhythm. It can
/// also keep track of a position in the rhythm to allow playback.
pub struct Rhythm {
    beats_per_minute: f32,
    time_signature: Fraction,
    beats: Vec<Beat>,
    current_beat: usize
}

impl Rhythm {
    /// Creates an empty rhythmic track or pattern with a given time signature
    /// and beats per minute.
    /// 
    /// # Parameters
    /// 
    /// - `beats_per_minute`: A floating point number which represents the
    /// beats per minute of the rhythmic track.
    /// - `time_signature`: A [`Fraction`] representing the time signature
    /// of the rhythmic track.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use musictools::rhythm::Rhythm;
    /// use musictools::common::Fraction;
    /// 
    /// let time_signature = Fraction::new(4, 4);
    /// let empty_rhythm = Rhythm::new(120.0, time_signature);
    /// ```
    pub fn new(beats_per_minute: f32, time_signature: Fraction) -> Rhythm {
        return Rhythm {
            beats_per_minute,
            time_signature,
            beats: Vec::new(),
            current_beat: 0
        };
    }

    /// Creates an rhythmic track or pattern with a given time signature and
    /// beats per minute from a vector of beats.
    /// 
    /// # Parameters
    /// 
    /// - `beats_per_minute`: A floating point number which represents the
    /// beats per minute of the rhythmic track.
    /// - `time_signature`: A [`Fraction`] representing the time signature
    /// of the rhythmic track.
    /// - `beats`: A [`Vec<Beat>`] holding the sequence of beats to be stored.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use musictools::rhythm::{Rhythm, Beat};
    /// use musictools::common::Fraction;
    /// 
    /// let time_signature = Fraction::new(5, 4);
    /// let beats = Vec::from([Beat::QUARTER_DOTTED, Beat::QUARTER_DOTTED, Beat::QUARTER, Beat::QUARTER]);
    /// let rhythm = Rhythm::from(160.0, time_signature, beats);
    /// ```
    pub fn from(beats_per_minute: f32, time_signature: Fraction, beats: Vec<Beat>) -> Rhythm {
        return Rhythm {
            beats_per_minute,
            time_signature,
            beats,
            current_beat: 0
        };
    }

    /// Adds a beat to the current rhythmic sequence.
    /// 
    /// # Parameters
    /// 
    /// - `beat`: A [`Beat`] containing the duration of the beat to be added to
    /// the rhythm.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use musictools::rhythm::{Rhythm, Beat};
    /// use musictools::common::Fraction;
    /// 
    /// let time_signature = Fraction::new(3, 4);
    /// let mut rhythm = Rhythm::new(140.0, time_signature);
    /// rhythm.push(Beat::QUARTER);
    /// rhythm.push(Beat::EIGHTH);
    /// rhythm.push(Beat::QUARTER);
    /// rhythm.push(Beat::EIGHTH);
    /// ```
    pub fn push(&mut self, beat: Beat) {
        self.beats.push(beat);
    }

    /// Removes the last beat from the current rhythmic sequence.
    pub fn pop(&mut self) -> Option<Beat> {
        return self.beats.pop();
    }

    /// Inserts a beat at a specific index in the current rhythmic sequence.
    /// 
    /// # Parameters
    /// 
    /// - `index`: The index in the sequence where the beat will be inserted.
    /// - `beat`: A [`Beat`] containing the duration of the beat to be added to
    /// the rhythm.
    pub fn insert(&mut self, index: usize, beat: Beat) {
        self.beats.insert(index, beat);
    }

    /// Removes a beat from a specific index in the current rhythmic sequence.
    /// 
    /// # Parameters
    /// 
    /// - `index`: The index in the sequence of the beat will be removed.
    pub fn remove(&mut self, index: usize) {
        self.beats.remove(index);
    }

    /// Returns the beat at a given index.
    /// 
    /// # Parameters
    /// 
    /// - `index`: The index in the sequence of the beat that to be returned.
    pub fn at(&self, index: usize) -> Beat {
        return self.beats[index];
    }

    /// Returns the amount of beats in the rhythmic sequence.
    pub fn get_num_beats(&self) -> usize {
        return self.beats.len();
    }

    /// Returns the beats per minute of the rhythmic sequence.
    pub fn get_bpm(&self) -> f32 {
        return self.beats_per_minute;
    }

    /// Changes the beats per minute of the rhythm to a given value.
    /// 
    /// # Parameters
    /// 
    /// - `beats_per_minute`: The new value of beats per minute of the rhythm.
    pub fn set_bpm(&mut self, beats_per_minute: f32) {
        self.beats_per_minute = beats_per_minute;
    }

    /// Returns the time signature of the rhythm.
    pub fn get_time_signature(&self) -> Fraction {
        return self.time_signature;
    }

    /// Changes the time signature of the rhythm to a new value.
    /// 
    /// # Parameters
    /// 
    /// - `time_signature`: A [`Fraction`] representing the time signature
    /// of the rhythm.
    pub fn set_time_signature(&mut self, time_signature: Fraction) {
        self.time_signature = time_signature;
    }

    /// Returns the duration in seconds of a beat at a given its index in the
    /// rhythmic sequence.
    /// 
    /// # Parameters
    /// 
    /// - `index`: The index in the rhythm of the beat whose duration will be
    /// returned.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use musictools::rhythm::{Rhythm, Beat};
    /// use musictools::common::Fraction;
    /// 
    /// let time_signature = Fraction::new(3, 4);
    /// let mut rhythm = Rhythm::new(140.0, time_signature);
    /// rhythm.push(Beat::QUARTER);
    /// rhythm.push(Beat::EIGHTH);
    /// rhythm.push(Beat::QUARTER);
    /// rhythm.push(Beat::EIGHTH);
    /// println!("{}", rhythm.get_duration_at_index(0));
    /// println!("{}", rhythm.get_duration_at_index(1));
    /// ```
    pub fn get_duration_at_index(&self, index: usize) -> f32 {
        let beats_per_second = self.beats_per_minute / 60.0;
        let beats_per_whole_note = self.time_signature.get_denominator();
        let whole_note_duration = beats_per_whole_note as f32 / beats_per_second;
        return whole_note_duration * self.beats[index].get_as_float();
    }

    /// Tells the rhythm to advance its internal tracker to the next beat.
    pub fn next_position(&mut self) {
        self.current_beat = (self.current_beat + 1) % self.beats.len();
    }

    /// Tells the rhythm to reset its internal tracker to the beginning of the
    /// sequence.
    pub fn reset_position(&mut self) {
        self.current_beat = 0;
    }

    /// Returns the position of the sequence's internal tracker.
    pub fn get_position(&self) -> usize {
        return self.current_beat;
    }

    /// Returns the duration in seconds of the beat pointed to by the internal
    /// tracker.
    /// 
    /// /// # Examples
    /// 
    /// ```rust
    /// use std::time::Duration;
    /// use musictools::rhythm::{Rhythm, Beat};
    /// use musictools::common::Fraction;
    /// 
    /// let time_signature = Fraction::new(3, 4);
    /// let mut rhythm = Rhythm::from(140.0, time_signature, Vec::from(
    ///     [Beat::QUARTER, Beat::EIGHTH, Beat::QUARTER, Beat::EIGHTH]
    /// ));
    /// for _index in 0..4 {
    ///     let milliseconds = rhythm.get_duration_of_current_beat() * 1000.0;
    ///     println!("Waiting {milliseconds} ms...");
    ///     std::thread::sleep(Duration::from_millis(milliseconds as u64));
    ///     rhythm.next_position();
    /// }
    /// ```
    pub fn get_duration_of_current_beat(&mut self) -> f32 {
        return self.get_duration_at_index(self.current_beat);
    }

    /// Returns the vector of beats in the rhythm.
    pub fn get_beats(&self) -> &Vec<Beat> {
        return &self.beats;
    }
}