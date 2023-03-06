use apres::MIDIEvent;
use apres::MIDI as Apres_MIDI;
use crate::note::Note;
use crate::track::Track;
use crate::common::Fraction;

#[derive(Clone, Debug)]
pub struct MIDI {
    tracks: Vec<Track>
}

impl MIDI {
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
            let mut track = Track::new(tempo, time_signature, ticks_per_quarter_note);
            for (delta_ticks, event_id) in midi_track_info {
                let event = match midi_object.get_event(event_id) {
                    Some(event_object) => event_object,
                    None => continue
                };
                match event {
                    MIDIEvent::NoteOn(_channel, note_index, velocity) => {
                        if velocity > 0 {
                            track.add_event(Note::from_midi_index(note_index).unwrap(), true, delta_ticks as u64);
                        } else {
                            track.add_event(Note::from_midi_index(note_index).unwrap(), false, delta_ticks as u64);
                        }
                    },
                    MIDIEvent::NoteOff(_channel, note_index, _velocity) => {
                        track.add_event(Note::from_midi_index(note_index).unwrap(), false, delta_ticks as u64);
                    },
                    MIDIEvent::TimeSignature(numerator, denominator, _clocks_per_metronome, _thirtysecondths_per_quarter) => {
                        time_signature = Fraction::new(numerator, u8::pow(2, denominator as u32));
                    },
                    MIDIEvent::SetTempo(us_per_quarter_note) => {
                        tempo = 60000000 as f32 / us_per_quarter_note as f32;
                    },
                    _ => {}
                }
            }
            if track.get_length() > 0 {
                tracks.push(track);
            }
        }
        let mut timed_tracks: Vec<Track> = Vec::new();
        for mut track in tracks {
            track.set_time_signature(time_signature);
            track.set_tempo(tempo);
            timed_tracks.push(track);
        }
        return Some(MIDI {
            tracks: timed_tracks
        });
    }

    pub fn export_to_file(file_path: String) {
        todo!();
    }

    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track);
    }

    pub fn get_tracks(&self) -> Vec<Track> {
        return self.tracks.clone();
    }
}