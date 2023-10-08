use crate::chord::Chord;
use crate::common::{Beat, Fraction, IncompleteChordError};
use crate::note::Note;
use std::fmt;

/// This structure is used to store a track with a sequence of events with the same structure as a
/// MIDI event, however holding [`Note`] structures instead.
#[derive(Clone, Debug, PartialEq)]
pub struct Track {
    tempo: f32,
    time_signature: Fraction,
    ticks_per_quarter_note: u16,
    duration: u64,
    current_delta_ticks: u64,
    current_event: usize,
    events: Vec<Event>,
}

impl Track {
    /// Creates an empty track with a given tempo and time signature, with a default value of 360
    /// MIDI ticks per quarter note.
    pub fn new(tempo: f32, time_signature: Fraction) -> Self {
        Self {
            tempo,
            time_signature,
            ticks_per_quarter_note: 360,
            duration: 0,
            current_delta_ticks: 0,
            current_event: 0,
            events: Vec::new(),
        }
    }

    /// Creates an empty track with a given tempo, time signature and MIDI ticks per quarter note.
    pub fn new_with_ticks(
        tempo: f32,
        time_signature: Fraction,
        ticks_per_quarter_note: u16,
    ) -> Self {
        Self {
            tempo,
            time_signature,
            ticks_per_quarter_note,
            duration: 0,
            current_delta_ticks: 0,
            current_event: 0,
            events: Vec::new(),
        }
    }

    /// Adds a new [`Event`] to the current track, which can be used to turn a [`Note`] on or off
    /// after a certain amount of MIDI ticks.
    ///
    /// # Parameters
    ///
    /// - `note`: The [`Note`] to be activated or deactivated.
    /// - `active`: A boolean representing whether to activate or deactivate the note.
    /// - `delta_ticks`: The amount of MIDI ticks until the event should occur.
    pub fn add_event(&mut self, note: Note, active: bool, delta_ticks: u64) {
        let total_delta_ticks = self.current_delta_ticks + delta_ticks;
        self.events.push(Event {
            note,
            active,
            delta_ticks: total_delta_ticks,
        });
        self.current_delta_ticks = 0;
        self.duration += delta_ticks;
    }

    /// Adds a [`Note`] to the end of the current track which will be played for the given duration.
    ///
    /// # Parameters
    ///
    /// - `note`: The [`Note`] to be added.
    /// - `duration`: A [`Beat`] representing the duration to play the note for.
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
        self.current_delta_ticks += delta_ticks;
        self.duration += delta_ticks;
    }

    /// Adds a [`Chord`] to the end of the current track which will be played for the given
    /// duration. The function will return a [`Result`] which can contain an
    /// [`IncompleteChordError`] if the chord did not have a tonic or an octave.
    ///
    /// # Parameters
    ///
    /// - `chord`: The [`Chord`] to be added.
    /// - `duration`: A [`Beat`] representing the duration to play the chord for.
    pub fn add_chord(&mut self, chord: Chord, duration: Beat) -> Result<(), IncompleteChordError> {
        let delta_ticks = self.beat_to_ticks(duration);
        let notes = Vec::<Note>::try_from(chord)?;
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
        Ok(())
    }

    /// Sets the tempo of the current track to a given value in beats per minute.
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
    /// - `time_signature`: A [`Fraction`] representing the new time signature of the track.
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

    /// Returns an [`Option<Event>`] which may contain the next MIDI event in the track or [`None`]
    /// if the end of the track has been reached.
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
    pub fn get_tick_duration(&self) -> f32 {
        60000.0 / (self.tempo * self.ticks_per_quarter_note as f32)
    }

    /// Resets the internal event tracker to the start of the track.
    pub fn reset_tracker(&mut self) {
        self.current_event = 0;
    }

    /// Returns the track as a vector of tuples with a [`Note`] and a [`u64`] representing how many
    /// MIDI ticks the note plays for. This function assumes that the track is monophonic. If
    /// multiple events intersect then this function only considers the highest note and discards
    /// all other notes.
    pub fn flatten(&self) -> Vec<(Note, u64)> {
        let mut flattened: Vec<(Note, u64)> = Vec::new();
        let mut last_note_option: Option<Note> = None;
        let rest_note = Note::from_midi_index(0).unwrap();
        for event in &self.events {
            let delta_ticks = event.get_delta_ticks();
            let last_note_candidate = if event.is_active() {
                Some(event.get_note())
            } else {
                None
            };
            if delta_ticks > 0 {
                let note: Note = match last_note_option {
                    Some(last_note) => last_note,
                    None => rest_note,
                };
                flattened.push((note, delta_ticks));
            } else if let Some(last_note) = last_note_option {
                if let Some(note_candidate) = last_note_candidate {
                    if note_candidate.get_frequency() < last_note.get_frequency() {
                        continue;
                    }
                }
            }
            last_note_option = last_note_candidate;
        }
        flattened
    }

    /// Returns a flattened copy of the track as a string that can be played by the GRUB bootloader.
    pub fn to_grub(&mut self) -> String {
        let grub_tempo = self.tempo as u64 * self.ticks_per_quarter_note as u64;
        let mut values: Vec<u64> = vec![grub_tempo];
        for (note, ticks) in self.flatten() {
            let frequency = if let Some(value) = note.get_midi_index() {
                if value == 0 {
                    0
                } else {
                    note.get_frequency() as u64
                }
            } else {
                note.get_frequency() as u64
            };
            values.push(frequency);
            values.push(ticks);
        }
        values
            .into_iter()
            .map(|int_val| int_val.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn beat_to_ticks(&self, beat: Beat) -> u64 {
        (4 * self.ticks_per_quarter_note as u64 * beat.get_numerator() as u64)
            / beat.get_denominator() as u64
    }
}

impl Default for Track {
    fn default() -> Self {
        Self {
            tempo: 120.0,
            time_signature: Fraction::new(4, 4),
            ticks_per_quarter_note: 360,
            duration: 0,
            current_delta_ticks: 0,
            current_event: 0,
            events: Vec::new(),
        }
    }
}

impl Eq for Track {}

/// A struct representing a MIDI or track event.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Event {
    note: Note,
    active: bool,
    delta_ticks: u64,
}

impl Event {
    /// Returns the [`Note`] associated with the current event.
    pub fn get_note(&self) -> Note {
        self.note
    }

    /// Returns true if the event activates the current note, or false if it deactivates it.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Returns the amount of MIDI ticks between the last event and the current event.
    pub fn get_delta_ticks(&self) -> u64 {
        self.delta_ticks
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = match self.active {
            true => "on",
            false => "off",
        };
        write!(
            f,
            "MIDI event: turn {} {} after {} ticks",
            self.note, state, self.delta_ticks
        )
    }
}
