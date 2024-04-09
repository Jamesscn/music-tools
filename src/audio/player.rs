use super::common::{ArpeggioDirection, AudioPlayError, Playable, Synth};
use super::processor::{AudioProcessor, SynthRef};
use super::wavetable::WavetableOscillator;
use crate::common::{AudioDuration, Beat, EqualTemperament, InputError, Tuning};
use crate::pitchclass::{PitchClass, TwelveTone};
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use rodio::{OutputStream, Sink, Source};
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Duration;

#[cfg(feature = "midi")]
use {
    crate::midi::common::MIDIEvent, crate::midi::parser::MIDI, crate::midi::track::TrackItem,
    crate::note::Note,
};

/// An enum representing the amount of bits per sample to use while exporting a WAV file.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BitsPerSample {
    /// Represents 8 bits per sample
    EIGHT = 8,
    /// Represents 16 bits per sample
    SIXTEEN = 16,
    /// Represents 24 bits per sample
    #[default]
    TWENTYFOUR = 24,
}

impl fmt::Display for BitsPerSample {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} bits per sample", *self as u32)
    }
}

impl TryFrom<usize> for BitsPerSample {
    type Error = InputError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            8 => Ok(Self::EIGHT),
            16 => Ok(Self::SIXTEEN),
            24 => Ok(Self::TWENTYFOUR),
            _ => Err(InputError {
                message: String::from("could not convert invalid usize to bits per sample"),
            }),
        }
    }
}

impl From<BitsPerSample> for usize {
    fn from(value: BitsPerSample) -> Self {
        value as Self
    }
}

#[derive(Clone, Debug)]
struct AudioBuffer {
    audio: Vec<f32>,
    index: usize,
}

impl AudioBuffer {
    pub fn new(audio: &[f32]) -> Self {
        Self {
            audio: Vec::from(audio),
            index: 0,
        }
    }
}

impl Iterator for AudioBuffer {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.audio.get(self.index);
        self.index += 1;
        sample.copied()
    }
}

impl Source for AudioBuffer {
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

impl PartialEq for AudioBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.audio == other.audio
    }
}

impl Eq for AudioBuffer {}

/// A structure which can be used to play audio through the speakers of the current machine or to
/// export audio into a WAV file.
pub struct AudioPlayer<PitchClassType: PitchClass = TwelveTone> {
    tempo: f32,
    speed: f32,
    base_frequency: f32,
    sink: Sink,
    _stream: OutputStream,
    processor: AudioProcessor,
    synth_ref: SynthRef,
    tuning: Box<dyn Tuning<PitchClassType>>,
    buffer: Vec<f32>,
}

impl<PitchClassType: PitchClass> AudioPlayer<PitchClassType> {
    /// Attempts to create a new audio player. A [`Result`] is returned which can be an error if
    /// there are no audio devices that can be captured.
    pub fn try_new() -> Result<Self, AudioPlayError> {
        let stream_result = OutputStream::try_default();
        if stream_result.is_err() {
            return Err(AudioPlayError {
                message: String::from("no sound card detected"),
            });
        }
        let (_stream, stream_handle) = stream_result.unwrap();
        let sink_result = Sink::try_new(&stream_handle);
        if sink_result.is_err() {
            return Err(AudioPlayError {
                message: String::from("sink could not be created"),
            });
        }
        let mut processor = AudioProcessor::new();
        let default_synth_ref = processor.register_synth(WavetableOscillator::default());
        Ok(Self {
            tempo: 120.0,
            speed: 1f32,
            base_frequency: 440f32,
            sink: sink_result.unwrap(),
            _stream,
            processor,
            synth_ref: default_synth_ref,
            tuning: Box::new(EqualTemperament::new()),
            buffer: Vec::new(),
        })
    }

    /// Sets the synthesizer that will be used to play the audio. If this function is never called a
    /// default synthesizer is used.
    ///
    /// # Parameters
    ///
    /// - `synth`: A synthesizer that implements the [`Synth`] trait.
    pub fn set_synth(&mut self, synth: impl Synth + Sync + Send + 'static) {
        self.processor.unregister_synth(&self.synth_ref);
        let synth_ref = self.processor.register_synth(synth);
        self.synth_ref = synth_ref;
    }

    /// Sets the volume of the audio player.
    ///
    /// # Parameters
    ///
    /// - `volume`: An [`f32`] that represents the volume of the audio player, which must be between
    ///   0.0 and 1.0. Volumes outside of this range are clamped.
    pub fn set_volume(&mut self, volume: f32) {
        self.processor.set_volume(volume.clamp(0.0, 1.0));
    }

    /// Sets the tempo of the audio player.
    ///
    /// # Parameters
    ///
    /// - `tempo`: A positive [`f32`] representing the new tempo of the audio player in beats per
    ///   minute. This does not affect the tempo of MIDI audio. If this value is less than or equal
    ///   to zero it is ignored.
    pub fn set_tempo(&mut self, tempo: f32) {
        if tempo > 0f32 {
            self.tempo = tempo;
        }
    }

    /// Sets the playback speed of the audio player, which is set to 1x by default.
    ///
    /// # Parameters:
    ///
    /// - `speed`: A positive [`f32`] representing the playback speed of the audio player. If there
    ///   is a tempo change in a MIDI track it is scaled with respect to this value. If this value
    ///   is less than or equal to zero it is ignored.
    pub fn set_speed(&mut self, speed: f32) {
        if speed > 0f32 {
            self.speed = speed;
        }
    }

    pub fn set_base_frequency(&mut self, base_frequency: f32) {
        if base_frequency > 0f32 {
            self.base_frequency = base_frequency;
        }
    }

    pub fn set_tuning(&mut self, tuning: impl Tuning<PitchClassType> + 'static) {
        self.tuning = Box::new(tuning);
    }

    /// Returns a reference to the [`AudioProcessor`] used by the audio player.
    pub fn get_processor(&self) -> &AudioProcessor {
        &self.processor
    }

    /// Pushes playable audio to the queue of audio to be played.
    ///
    /// # Parameters
    ///
    /// - `playable`: The audio to be played which must implement the [`Playable`] trait.
    /// - `duration`: A duration representing how long the audio will be played for. This duration
    ///   must implement the [`AudioDuration`] trait.
    pub fn push(
        &mut self,
        playable: &impl Playable<PitchClassType>,
        duration: &impl AudioDuration,
    ) {
        for frequency in playable.get_frequencies(self.tuning.as_ref(), self.base_frequency) {
            self.processor.start_frequency(frequency, &self.synth_ref);
        }
        let mut audio_vec = self.processor.render(duration.get_duration(self.tempo));
        self.processor.stop_all_frequencies();
        self.buffer.append(&mut audio_vec);
    }

    /// Pushes a rest note to the queue of audio to be played.
    ///
    /// # Parameters
    ///
    /// - `duration`: A duration representing how long the rest will last for. This duration must
    ///   implement the [`AudioDuration`] trait.
    pub fn push_rest(&mut self, duration: &impl AudioDuration) {
        let mut audio_vec = self.processor.render(duration.get_duration(self.tempo));
        self.buffer.append(&mut audio_vec);
    }

    /// Pushes an arpeggiation of playable audio to the queue of audio to be played.
    ///
    /// # Parameters
    ///
    /// - `playable`: The audio to be arpeggiated which must implement the [`Playable`] trait.
    /// - `duration`: A duration representing how long each individual repetition of the audio will
    ///   be played for. This duration must implement the [`AudioDuration`] trait.
    /// - `direction`: An [`ArpeggioDirection`] enum representing the direction that the audio will
    ///   be arpeggiated in.
    /// - `total_notes`: A [`usize`] representing the total amount of individual notes that will be
    ///   played while the arpeggio is happening.
    pub fn push_arpeggiate(
        &mut self,
        playable: &impl Playable<PitchClassType>,
        duration: &impl AudioDuration,
        direction: ArpeggioDirection,
        total_notes: usize,
    ) {
        let frequencies = playable.get_frequencies(self.tuning.as_ref(), self.base_frequency);
        let mut updown_ascending: bool = true;
        let mut current_index = match direction {
            ArpeggioDirection::Up => 0,
            ArpeggioDirection::Down => frequencies.len() - 1,
            ArpeggioDirection::UpDown => 0,
        };
        for _ in 0..total_notes {
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
            };
            if !updown_ascending && current_index == 0 {
                updown_ascending = true;
            }
            if updown_ascending && current_index == frequencies.len() - 1 {
                updown_ascending = false;
            }
        }
    }

    /// Pushes a series of playable audio items with a rhythm onto the queue of audio to be played.
    ///
    /// # Parameters
    ///
    /// - `playables`: The audio items to be played to the given rhythm, which must be an iterable
    ///   set of [`Playable`] trait objects.
    /// - `rhythm`: A [`Rhythm`] representing the rhythm that will be used to play the audio.
    /// - `total_notes`: A [`usize`] representing the total amount of playable items that will be
    ///   played. If this value is greater than either of the `playables` and `rhythm` iterables
    ///   then they are wrapped around from the beginning.
    pub fn push_rhythm(
        &mut self,
        playables: &[impl Playable<PitchClassType>],
        rhythm: Vec<Beat>,
        total_notes: usize,
    ) {
        if playables.is_empty() || rhythm.is_empty() {
            return;
        }
        for index in 0..total_notes {
            let curr_playable = &playables[index % playables.len()];
            let curr_beat = &rhythm[index % rhythm.len()];
            self.push(curr_playable, curr_beat);
        }
    }

    /// Starts playing all the audio in the queue through the current speaker. Pauses the current
    /// thread while playing.
    pub fn play(&self) {
        let audio = AudioBuffer::new(&self.buffer);
        self.sink.append(audio);
        self.sink.play();
        self.sink.sleep_until_end();
    }

    /// Clears all the audio that has been queued.
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Renders the audio that has been queued into a [`Vec<f32>`].
    pub fn render(&self) -> Vec<f32> {
        self.buffer.clone()
    }

    /// Exports the audio that has been queued to a WAV file.
    ///
    /// # Parameters
    ///
    /// - `path`: A string representing the path of the WAV file to generate.
    /// - `bits_per_sample`: A [`BitsPerSample`] enum representing the amount of bits per sample to
    ///   be stored in the WAV file.
    pub fn export_wav(
        &self,
        path: impl AsRef<Path>,
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

impl AudioPlayer<TwelveTone> {
    #[cfg(feature = "midi")]
    /// Pushes a MIDI item onto the queue of audio to be played.
    ///
    /// # Parameters
    ///
    /// - `midi`: A reference to the [`MIDI`] to be played.
    /// - `synths`: An array of synthesizers that will be used to play all the tracks of the MIDI
    ///   item, which must implement the [`Synth`] trait. If no synths are provided the default
    ///   synth is used. If there are less synths than tracks, then the synths are wrapped around to
    ///   fit multiple tracks.
    pub fn push_midi(
        &mut self,
        midi: &MIDI,
        synths: &[impl Synth + Sync + Send + Clone + 'static],
    ) -> Result<(), InputError> {
        use std::sync::Arc;

        if midi.is_empty() {
            return Err(InputError {
                message: String::from("midi object is empty"),
            });
        }
        let mut synth_ref_vec: Vec<SynthRef> = Vec::new();
        if synths.is_empty() {
            synth_ref_vec.push(Arc::clone(&self.synth_ref))
        } else {
            for index in 0..usize::min(midi.get_num_tracks(), synths.len()) {
                synth_ref_vec.push(self.processor.register_synth(synths[index].clone()));
            }
        }
        let looping_synth_count = synth_ref_vec.len();
        for index in looping_synth_count..midi.get_num_tracks() {
            synth_ref_vec.push(Arc::clone(&synth_ref_vec[index % looping_synth_count]));
        }
        let mut curr_tempo = 120;
        for (track_index, track_item) in midi.iter_track_items() {
            let synth = &synth_ref_vec[track_index];
            match track_item {
                TrackItem::Event(event) => match event {
                    MIDIEvent::NoteOn(note) => {
                        self.processor.start_frequency(
                            self.tuning.get_frequency(
                                self.base_frequency,
                                Note::from_string("A4").unwrap(),
                                note,
                            ),
                            synth,
                        );
                    }
                    MIDIEvent::NoteOff(note) => {
                        self.processor.stop_frequency(
                            self.tuning.get_frequency(
                                self.base_frequency,
                                Note::from_string("A4").unwrap(),
                                note,
                            ),
                            synth,
                        );
                    }
                    MIDIEvent::SetTempo(tempo) => curr_tempo = tempo,
                    MIDIEvent::SetTimeSignature(_) => {}
                },
                TrackItem::Rest(beat) => {
                    let mut audio_vec = self
                        .processor
                        .render(beat.get_duration(curr_tempo as f32 * self.speed));
                    self.buffer.append(&mut audio_vec);
                }
            }
        }
        Ok(())
    }
}
