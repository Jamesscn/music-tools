use crate::common::Fraction;
use crate::note::Note;
use crate::track::Track;
use apres::MIDIEvent;
use apres::MIDI as Apres_MIDI;

/// A structure which holds a MIDI object that can be imported from or exported to a MIDI file,
/// containing a set of [`Track`] objects.
#[derive(Clone, Debug)]
pub struct MIDI {
    tracks: Vec<Track>,
}

impl MIDI {
    /// Creates an empty MIDI class with no tracks
    pub fn new() -> MIDI {
        MIDI { tracks: Vec::new() }
    }

    /// Imports a MIDI object from a MIDI file. The return value is an [`Option<MIDI>`] which can be
    /// [`None`] if the MIDI file provided does not exist or is invalid.
    ///
    /// # Parameters
    ///
    /// - `file_path`: A string of the path to the MIDI file to import.
    pub fn import_from_file(file_path: &str) -> Option<MIDI> {
        let midi_object = match Apres_MIDI::from_path(file_path) {
            Ok(apres_midi_object) => apres_midi_object,
            Err(_) => {
                println!("Could not read MIDI file from {file_path}!");
                return None;
            }
        };
        let ticks_per_quarter_note = midi_object.get_ppqn();
        let midi_tracks = midi_object.get_tracks();
        let mut tracks: Vec<Track> = Vec::new();
        let mut tempo: f32 = 0.0;
        let mut time_signature: Fraction = Fraction::new(4, 4);
        for midi_track_info in midi_tracks {
            let mut track = Track::new_with_ticks(tempo, time_signature, ticks_per_quarter_note);
            for (delta_ticks, event_id) in midi_track_info {
                let event = match midi_object.get_event(event_id) {
                    Some(event_object) => event_object,
                    None => continue,
                };
                match event {
                    MIDIEvent::NoteOn(_channel, note_index, velocity) => {
                        if velocity > 0 {
                            track.add_event(
                                Note::from_midi_index(note_index).unwrap(),
                                true,
                                delta_ticks as u64,
                            );
                        } else {
                            track.add_event(
                                Note::from_midi_index(note_index).unwrap(),
                                false,
                                delta_ticks as u64,
                            );
                        }
                    }
                    MIDIEvent::NoteOff(_channel, note_index, _velocity) => {
                        track.add_event(
                            Note::from_midi_index(note_index).unwrap(),
                            false,
                            delta_ticks as u64,
                        );
                    }
                    MIDIEvent::TimeSignature(
                        numerator,
                        denominator,
                        _clocks_per_metronome,
                        _thirtysecondths_per_quarter,
                    ) => {
                        time_signature = Fraction::new(numerator, u8::pow(2, denominator as u32));
                    }
                    MIDIEvent::SetTempo(us_per_quarter_note) => {
                        tempo = 60000000.0 / us_per_quarter_note as f32;
                    }
                    _ => {}
                }
            }
            if track.get_duration() > 0 {
                tracks.push(track);
            }
        }
        let mut timed_tracks: Vec<Track> = Vec::new();
        for mut track in tracks {
            track.set_time_signature(time_signature);
            track.set_tempo(tempo);
            timed_tracks.push(track);
        }
        Some(MIDI {
            tracks: timed_tracks,
        })
    }

    /// Exports a MIDI object to a MIDI file. The function returns true if the file was successfully
    /// exported or false if it was not or if the MIDI object has no tracks.
    ///
    /// # Parameters
    ///
    /// - `file_path`: A string of the path to save the MIDI file to.
    pub fn export_to_file(&self, file_path: &str) -> bool {
        let mut midi_object = Apres_MIDI::new();
        if self.tracks.is_empty() {
            return false;
        }
        let time_signature: Fraction = self.tracks[0].get_time_signature();
        let tempo: f32 = self.tracks[0].get_tempo();
        let ppqn: u16 = self.tracks[0].get_ticks_per_quarter_note();
        let us_per_quarter_note: u32 = (60000000.0 / tempo) as u32;
        let midi_num = time_signature.get_numerator();
        let midi_denom = f64::log2(time_signature.get_denominator() as f64) as u8;
        midi_object.set_ppqn(ppqn);
        midi_object.insert_event(0, 0, MIDIEvent::TimeSignature(midi_num, midi_denom, 24, 8));
        midi_object.insert_event(0, 0, MIDIEvent::SetTempo(us_per_quarter_note));
        let mut track_index = 1;
        for mut track in self.tracks.clone() {
            let mut current_tick = 0;
            while let Some(event) = track.get_next_event() {
                let note_option = event.get_note().get_midi_index();
                if note_option.is_none() {
                    continue;
                }
                let note_index = note_option.unwrap();
                let midi_event: MIDIEvent = if event.is_active() {
                    MIDIEvent::NoteOn(0, note_index, 100)
                } else {
                    MIDIEvent::NoteOff(0, note_index, 0)
                };
                current_tick += event.get_delta_ticks() as usize;
                midi_object.insert_event(track_index, current_tick, midi_event);
            }
            track_index += 1;
        }
        midi_object.save(file_path);
        true
    }

    /// Adds a [`Track`] to the MIDI object.
    ///
    /// # Parameters
    ///
    /// - `track`: The [`Track`] to add to the current MIDI object.
    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track);
    }

    /// Returns a vector of [`Track`] with the tracks of the MIDI object.
    pub fn get_tracks(&self) -> Vec<Track> {
        self.tracks.clone()
    }

    /// Returns the number of valid tracks in the MIDI object.
    pub fn get_num_tracks(&self) -> usize {
        self.tracks.len()
    }
}

impl Default for MIDI {
    fn default() -> Self {
        Self::new()
    }
}
