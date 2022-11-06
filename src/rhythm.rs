use crate::common::Fraction;

pub type Beat = Fraction;

impl Beat {
    pub fn whole() -> Beat {
        return Beat::new(1, 1);
    }

    pub fn half() -> Beat {
        return Beat::new(1, 2);
    }

    pub fn quarter() -> Beat {
        return Beat::new(1, 4);
    }

    pub fn eighth() -> Beat {
        return Beat::new(1, 8);
    }

    pub fn sixteenth() -> Beat {
        return Beat::new(1, 16);
    }

    pub fn thirtysecond() -> Beat {
        return Beat::new(1, 32);
    }

    pub fn whole_dotted() -> Beat {
        return Beat::new(3, 2);
    }

    pub fn half_dotted() -> Beat {
        return Beat::new(3, 4);
    }

    pub fn quarter_dotted() -> Beat {
        return Beat::new(3, 8);
    }

    pub fn eighth_dotted() -> Beat {
        return Beat::new(3, 16);
    }

    pub fn sixteenth_dotted() -> Beat {
        return Beat::new(3, 32);
    }

    pub fn thirtysecond_dotted() -> Beat {
        return Beat::new(3, 64);
    }
}

pub struct Rhythm {
    beats_per_minute: f32,
    time_signature: Fraction,
    beats: Vec<Beat>,
    current_beat: usize
}

impl Rhythm {
    pub fn new(beats_per_minute: f32, time_signature: Fraction) -> Rhythm {
        return Rhythm {
            beats_per_minute,
            time_signature,
            beats: Vec::new(),
            current_beat: 0
        }
    }

    pub fn from(beats_per_minute: f32, time_signature: Fraction, beats: Vec<Beat>) -> Rhythm {
        return Rhythm {
            beats_per_minute,
            time_signature,
            beats,
            current_beat: 0
        }
    }

    pub fn push(&mut self, beat: Beat) {
        self.beats.push(beat);
    }

    pub fn pop(&mut self) -> Option<Beat> {
        return self.beats.pop();
    }

    pub fn get_bpm(&self) -> f32 {
        return self.beats_per_minute;
    }

    pub fn set_bpm(&mut self, bpm: f32) {
        self.beats_per_minute = bpm;
    }

    pub fn get_time_signature(&self) -> Fraction {
        return self.time_signature;
    }

    pub fn set_time_signature(&mut self, time_signature: Fraction) {
        self.time_signature = time_signature;
    }

    pub fn get_seconds_per_whole_note(&self) -> f32 {
        let beats_per_second = self.beats_per_minute / 60.0;
        let beats_per_whole_note = self.time_signature.get_denominator();
        return beats_per_whole_note as f32 / beats_per_second;
    }

    fn get_beat_length(&self, beat: Beat) -> f32 {
        return self.get_seconds_per_whole_note() * beat.get_as_float();
    }

    pub fn get_beats(&self) -> &Vec<Beat> {
        return &self.beats;
    }

    pub fn reset(&mut self) {
        self.current_beat = 0;
    }

    pub fn get_next_beat(&mut self) -> (Beat, f32) {
        let beat = self.beats[self.current_beat];
        self.current_beat = (self.current_beat + 1) % self.beats.len();
        return (beat, self.get_beat_length(beat));
    }
}