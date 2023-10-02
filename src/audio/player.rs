use super::common::{ArpeggioDirection, AudioPlayError, Playable, Synth};
use super::processor::{AudioProcessor, SynthRc};
use super::wavetable::WavetableOscillator;
use crate::common::AudioDuration;
use crate::midi::MIDI;
use crate::track::Event;
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rodio::{OutputStream, Sink, Source};
use std::cmp::min;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::time::Duration;

#[derive(Copy, Clone, Debug)]
pub enum BitsPerSample {
    EIGHT = 8,
    SIXTEEN = 16,
    TWENTYFOUR = 24,
}

#[derive(Clone, Debug)]
pub struct PlayableAudio {
    audio: Vec<f32>,
    index: usize,
}

impl PlayableAudio {
    pub fn new(audio: Vec<f32>) -> Self {
        Self { audio, index: 0 }
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }
}

impl Iterator for PlayableAudio {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.audio.get(self.index);
        self.index += 1;
        sample.map(|x| *x)
    }
}

impl Source for PlayableAudio {
    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        44100
    }

    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

pub struct AudioPlayer {
    tempo: f32,
    sink: Sink,
    _stream: OutputStream,
    processor: AudioProcessor,
    synth_ref: SynthRc,
    buffer: Vec<f32>,
}

impl AudioPlayer {
    pub fn new() -> Result<Self, AudioPlayError> {
        let stream_result = OutputStream::try_default();
        if stream_result.is_err() {
            return Err(AudioPlayError {
                message: "no sound card detected",
            });
        }
        let (_stream, stream_handle) = stream_result.unwrap();
        let sink_result = Sink::try_new(&stream_handle);
        if sink_result.is_err() {
            return Err(AudioPlayError {
                message: "sink could not be created",
            });
        }
        let mut processor = AudioProcessor::new();
        let oscillator = WavetableOscillator::default();
        let default_synth_ref = processor.register_synth(Box::new(oscillator));
        Ok(Self {
            tempo: 120.0,
            sink: sink_result.unwrap(),
            _stream,
            processor,
            synth_ref: default_synth_ref,
            buffer: Vec::new(),
        })
    }

    pub fn set_synth(&mut self, synth: impl Synth + 'static) {
        self.processor.unregister_synth(&self.synth_ref);
        let synth_ref = self.processor.register_synth(Box::new(synth));
        self.synth_ref = synth_ref;
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.processor.set_volume(volume.clamp(0.0, 1.0));
    }

    pub fn set_tempo(&mut self, tempo: f32) {
        self.tempo = tempo;
    }

    pub fn get_processor(&self) -> &AudioProcessor {
        &self.processor
    }

    pub fn push(&mut self, playable: &impl Playable, duration: &impl AudioDuration) {
        for frequency in playable.get_frequencies() {
            self.processor.start_frequency(frequency, &self.synth_ref);
        }
        let mut audio_vec = self.processor.render(duration.get_duration(self.tempo));
        self.processor.stop_all_frequencies();
        self.buffer.append(&mut audio_vec);
    }

    pub fn push_rest(&mut self, duration: &impl AudioDuration) {
        let mut audio_vec = self.processor.render(duration.get_duration(self.tempo));
        self.buffer.append(&mut audio_vec);
    }

    pub fn push_arpeggiate(
        &mut self,
        playable: &impl Playable,
        duration: &impl AudioDuration,
        direction: ArpeggioDirection,
        repetitions: usize,
    ) {
        let frequencies = playable.get_frequencies();
        let mut rng = rand::thread_rng();
        let distribution = Uniform::from(0..frequencies.len());
        let mut updown_ascending: bool = true;
        let mut current_index = match direction {
            ArpeggioDirection::Up => 0,
            ArpeggioDirection::Down => frequencies.len() - 1,
            ArpeggioDirection::UpDown => 0,
            ArpeggioDirection::Random => distribution.sample(&mut rng),
        };
        for _ in 0..repetitions {
            let curr_frequency = frequencies[current_index];
            self.push(&curr_frequency, duration);
            current_index = match direction {
                ArpeggioDirection::Up => (current_index + 1).rem_euclid(frequencies.len()),
                ArpeggioDirection::Down => {
                    (current_index as isize - 1).rem_euclid(frequencies.len() as isize) as usize
                }
                ArpeggioDirection::UpDown => match updown_ascending {
                    true => (current_index + 1).rem_euclid(frequencies.len()),
                    false => {
                        (current_index as isize - 1).rem_euclid(frequencies.len() as isize) as usize
                    }
                },
                ArpeggioDirection::Random => distribution.sample(&mut rng),
            };
            if !updown_ascending && current_index == 0 {
                updown_ascending = true;
            }
            if updown_ascending && current_index == frequencies.len() - 1 {
                updown_ascending = false;
            }
        }
    }

    pub fn push_midi(
        &mut self,
        midi: &MIDI,
        synth: impl Synth + Clone + 'static,
        custom_tempo: Option<f32>,
    ) {
        let mut tracks = midi.get_tracks();
        if tracks.is_empty() {
            return;
        }
        if let Some(tempo) = custom_tempo {
            tracks[0].set_tempo(tempo)
        }
        let tick_ms = tracks[0].get_tick_duration();
        let mut pending_event_tuples: Vec<(Event, u64, usize)> = Vec::new();
        for (track_index, track) in &mut tracks.iter_mut().enumerate() {
            let first_event_option = track.get_next_event();
            if let Some(first_event) = first_event_option {
                let event_tuple = (first_event, first_event.get_delta_ticks(), track_index);
                pending_event_tuples.push(event_tuple);
            }
        }
        let mut synth_ref_vec: Vec<SynthRc> = Vec::new();
        for _ in tracks.iter() {
            let oscillator = synth.clone();
            let synth_ref = self.processor.register_synth(Box::new(oscillator));
            synth_ref_vec.push(synth_ref);
        }
        loop {
            let mut next_event_tuples: Vec<(Event, u64, usize)> = Vec::new();
            let mut min_wait_ticks = u64::MAX;
            'track: for event_index in (0..pending_event_tuples.len()).rev() {
                let event_tuple = pending_event_tuples[event_index];
                let mut current_event = event_tuple.0;
                let mut wait_time = event_tuple.1;
                let track_index = event_tuple.2;
                let synth = &synth_ref_vec[track_index];
                while wait_time == 0 {
                    if current_event.is_active() {
                        self.processor
                            .start_frequency(current_event.get_note().get_frequency(), synth);
                    } else {
                        self.processor
                            .stop_frequency(current_event.get_note().get_frequency(), synth);
                    }
                    let next_event_option = &mut tracks[track_index].get_next_event();
                    if next_event_option.is_none() {
                        self.processor.unregister_synth(synth);
                        continue 'track;
                    }
                    let next_event = next_event_option.unwrap();
                    wait_time = next_event.get_delta_ticks();
                    current_event = next_event;
                }
                min_wait_ticks = min(min_wait_ticks, wait_time);
                next_event_tuples.insert(0, (current_event, wait_time, track_index));
            }
            if next_event_tuples.is_empty() {
                break;
            }
            let mut audio_vec = self.processor.render(Duration::from_millis(
                (tick_ms * (min_wait_ticks as f32)) as u64,
            ));
            self.processor.stop_all_frequencies();
            self.buffer.append(&mut audio_vec);
            for event in &mut next_event_tuples {
                *event = (event.0, event.1 - min_wait_ticks, event.2);
            }
            pending_event_tuples = next_event_tuples;
        }
        for mut track in tracks {
            track.reset_tracker();
        }
    }

    pub fn play(&self) {
        let audio = PlayableAudio::new(self.buffer.clone());
        self.sink.append(audio);
        self.sink.play();
        self.sink.sleep_until_end();
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn render(&self) -> Vec<f32> {
        self.buffer.clone()
    }

    pub fn export_wav(
        &self,
        path: &str,
        bits_per_sample: BitsPerSample,
    ) -> Result<(), Box<dyn Error>> {
        const CHANNELS: u16 = 1; //Mono audio
        let subchunk2_len: u32 =
            self.buffer.len() as u32 * CHANNELS as u32 * bits_per_sample as u32 / 8;
        let mut file = File::create(path)?;
        let mut file_buffer: Vec<u8> = Vec::new();
        file_buffer.write_u32::<BigEndian>(0x52494646)?; //"RIFF"
        file_buffer.write_u32::<LittleEndian>(36 + subchunk2_len)?; //Chunk size
        file_buffer.write_u32::<BigEndian>(0x57415645)?; //"WAVE"
        file_buffer.write_u32::<BigEndian>(0x666d7420)?; //"fmt "
        file_buffer.write_u32::<LittleEndian>(16)?; //PCM mode
        file_buffer.write_u16::<LittleEndian>(1)?; //No compression
        file_buffer.write_u16::<LittleEndian>(CHANNELS)?; //Mono audio
        file_buffer.write_u32::<LittleEndian>(self.processor.get_sample_rate())?; //Sample rate
        file_buffer.write_u32::<LittleEndian>(
            self.processor.get_sample_rate() * CHANNELS as u32 * bits_per_sample as u32 / 8,
        )?; //Byte rate
        file_buffer.write_u16::<LittleEndian>(CHANNELS * bits_per_sample as u16 / 8)?; //Block align
        file_buffer.write_u16::<LittleEndian>(bits_per_sample as u16)?; //Block align
        file_buffer.write_u32::<BigEndian>(0x64617461)?; //"data"
        file_buffer.write_u32::<LittleEndian>(subchunk2_len)?;
        for sample in self.buffer.iter() {
            match bits_per_sample {
                BitsPerSample::EIGHT => {
                    file_buffer.write_u8((127.5 * sample + 127.5) as u8)?;
                }
                BitsPerSample::SIXTEEN => {
                    file_buffer.write_i16::<LittleEndian>((32767.5 * sample - 0.5) as i16)?;
                }
                BitsPerSample::TWENTYFOUR => {
                    file_buffer.write_i24::<LittleEndian>((8388607.5 * sample - 0.5) as i32)?;
                }
            }
        }
        file.write_all(&file_buffer)?;
        Ok(())
    }
}
