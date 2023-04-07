use crate::note::Note;
use crate::chord::Chord;
use crate::common::{Fraction, Beat};

/// This structure is used to store a track with a sequence of events with the
/// same structure as a MIDI event, however holding [`Note`] structures
/// instead.
#[derive(Clone, Debug)]
pub struct Track {
    tempo: f32,
    time_signature: Fraction,
    ticks_per_quarter_note: u16,
    duration: u64,
    current_event: usize,
    events: Vec<Event>
}

impl Track {
    /// Creates an empty track with a given tempo and time signature, with a
    /// default value of 360 MIDI ticks per quarter note.
    pub fn new(tempo: f32, time_signature: Fraction) -> Track {
        Track {
            tempo,
            time_signature,
            ticks_per_quarter_note: 360,
            duration: 0,
            current_event: 0,
            events: Vec::new()
        }
    }

    /// Creates an empty track with a given tempo, time signature and MIDI
    /// ticks per quarter note.
    pub fn new_with_ticks(tempo: f32, time_signature: Fraction, ticks_per_quarter_note: u16) -> Track {
        Track {
            tempo,
            time_signature,
            ticks_per_quarter_note,
            duration: 0,
            current_event: 0,
            events: Vec::new()
        }
    }

    /// Adds a new [`Event`] to the current track, which can be used to turn
    /// a [`Note`] on or off after a certain amount of MIDI ticks.
    /// 
    /// # Parameters
    /// 
    /// - `note`: The [`Note`] to be activated or deactivated.
    /// - `active`: A boolean representing whether to activate or deactivate
    /// the note.
    /// - `delta_ticks`: The amount of MIDI ticks until the event should occur.
    pub fn add_event(&mut self, note: Note, active: bool, delta_ticks: u64) {
        self.events.push(Event {
            note,
            active,
            delta_ticks
        });
        self.duration += delta_ticks;
    }

    /// Adds a [`Note`] to the end of the current track which will be played
    /// for the given duration.
    /// 
    /// # Parameters
    /// 
    /// - `note`: The [`Note`] to be added.
    /// - `duration`: A [`Beat`] representing the duration to play the note
    /// for.
    pub fn add_note(&mut self, note: Note, duration: Beat) {
        let delta_ticks = self.beat_to_ticks(duration);
        self.add_event(note, true, 0);
        self.add_event(note, false, delta_ticks);
    }

    /// Adds a rest to the end of the current track.
    /// 
    /// # Parameters
    /// 
    /// - `duration`: The duration the rest will take.
    pub fn add_rest(&mut self, duration: Beat) {
        let delta_ticks = self.beat_to_ticks(duration);
        self.add_event(Note::from_midi_index(0).unwrap(), false, delta_ticks);
    }

    /// Adds a [`Chord`] to the end of the current track which will be played
    /// for the given duration. The function will return false if the chord
    /// could not be converted to a set of [`Note`] objects.
    /// 
    /// # Parameters
    /// 
    /// - `chord`: The [`Chord`] to be added.
    /// - `duration`: A [`Beat`] representing the duration to play the chord
    /// for.
    pub fn add_chord(&mut self, chord: Chord, duration: Beat) -> bool {
        let delta_ticks = self.beat_to_ticks(duration);
        let notes = match chord.to_notes() {
            Some(notes_vec) => notes_vec,
            None => return false
        };
        for note in notes.clone() {
            self.add_event(note, true, 0);
        }
        let mut first_note = true;
        for note in notes {
            if first_note {
                self.add_event(note, false, delta_ticks);
                first_note = false;
            } else {
                self.add_event(note, false, 0);
            }
        }
        true
    }

    /// Sets the tempo of the current track to a given value in beats per
    /// minute.
    /// 
    /// # Parameters
    /// 
    /// - `tempo`: The new tempo of the track.
    pub fn set_tempo(&mut self, tempo: f32) {
        self.tempo = tempo;
    }

    /// Sets the time signature of the current track to a given value.
    /// 
    /// # Parameters
    /// 
    /// - `time_signature`: A [`Fraction`] representing the new time signature
    /// of the track.
    pub fn set_time_signature(&mut self, time_signature: Fraction) {
        self.time_signature = time_signature;
    }

    /// Returns the tempo of the track in beats per minute.
    pub fn get_tempo(&self) -> f32 {
        self.tempo
    }

    /// Returns a [`Fraction`] representing the time signature of the track.
    pub fn get_time_signature(&self) -> Fraction {
        self.time_signature
    }

    /// Returns the total duration of the track in MIDI ticks.
    pub fn get_duration(&self) -> u64 {
        self.duration
    }

    /// Returns an [`Option<Event>`] which may contain the next MIDI event
    /// in the track or [`None`] if the end of the track has been reached.
    pub fn get_next_event(&mut self) -> Option<Event> {
        if self.current_event < self.events.len() {
            let event = self.events[self.current_event];
            self.current_event += 1;
            return Some(event);
        }
        None
    }

    /// Returns the amount of MIDI ticks in a quarter note.
    pub fn get_ticks_per_quarter_note(&self) -> u16 {
        self.ticks_per_quarter_note
    }

    /// Returns the duration of a single tick in milliseconds.
    pub fn get_tick_duration(&mut self) -> f32 {
        60000.0 / (self.tempo * self.ticks_per_quarter_note as f32)
    }

    fn beat_to_ticks(&self, beat: Beat) -> u64 {
        (4 * self.ticks_per_quarter_note as u64 * beat.get_numerator() as u64) / beat.get_denominator() as u64
    }
}

/// A struct representing a MIDI or track event.
#[derive(Copy, Clone, Debug)]
pub struct Event {
    note: Note,
    active: bool,
    delta_ticks: u64
}

impl Event {
    /// Returns the [`Note`] associated with the current event.
    pub fn get_note(&self) -> Note {
        self.note
    }

    /// Returns true if the event activates the current note, or false if it
    /// deactivates it.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Returns the amount of MIDI ticks between the last event and the current
    /// event.
    pub fn get_delta_ticks(&self) -> u64 {
        self.delta_ticks
    }
}