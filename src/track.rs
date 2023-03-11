use crate::note::Note;
use crate::chord::Chord;
use crate::common::{Fraction, Beat};

#[derive(Clone, Debug)]
/// This structure is used to store a track with a sequence of events with the
/// same structure as a MIDI event, however holding [`Note`] structures
/// instead.
pub struct Track {
    tempo: f32,
    time_signature: Fraction,
    ticks_per_quarter_note: u16,
    length: u64,
    current_event: usize,
    events: Vec<Event>
}

impl Track {
    pub fn new(tempo: f32, time_signature: Fraction, ticks_per_quarter_note: u16) -> Track {
        return Track {
            tempo,
            time_signature,
            ticks_per_quarter_note,
            length: 0,
            current_event: 0,
            events: Vec::new()
        };
    }

    pub fn add_event(&mut self, note: Note, active: bool, delta_ticks: u64) {
        self.events.push(Event {
            note,
            active,
            delta_ticks
        });
        self.length += delta_ticks;
    }

    pub fn add_note(&mut self, note: Note, duration: Beat) {
        let delta_ticks = self.beat_to_ticks(duration);
        self.add_event(note, true, 0);
        self.add_event(note, false, delta_ticks);
    }

    pub fn add_rest(&mut self, duration: Beat) {
        let delta_ticks = self.beat_to_ticks(duration);
        self.add_event(Note::from_midi_index(0).unwrap(), false, delta_ticks);
    }

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
        return true;
    }

    pub fn set_tempo(&mut self, tempo: f32) {
        self.tempo = tempo;
    }

    pub fn set_time_signature(&mut self, time_signature: Fraction) {
        self.time_signature = time_signature;
    }

    pub fn get_tempo(&self) -> f32 {
        return self.tempo;
    }

    pub fn get_time_signature(&self) -> Fraction {
        return self.time_signature;
    }

    pub fn get_length(&self) -> u64 {
        return self.length;
    }

    pub fn get_next_event(&mut self) -> Option<Event> {
        if self.current_event < self.events.len() {
            let event = self.events[self.current_event];
            self.current_event += 1;
            return Some(event);
        }
        return None;
    }

    pub fn get_ticks_per_quarter_note(&self) -> u16 {
        return self.ticks_per_quarter_note;
    }

    pub fn get_tick_duration(&mut self) -> f32 {
        return 60000.0 / (self.tempo * self.ticks_per_quarter_note as f32);
    }

    fn beat_to_ticks(&self, beat: Beat) -> u64 {
        return (4 * self.ticks_per_quarter_note as u64 * beat.get_numerator() as u64) / beat.get_denominator() as u64;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Event {
    note: Note,
    active: bool,
    delta_ticks: u64
}

impl Event {
    pub fn get_note(&self) -> Note {
        return self.note;
    }

    pub fn is_active(&self) -> bool {
        return self.active;
    }

    pub fn get_delta_ticks(&self) -> u64 {
        return self.delta_ticks;
    }
}