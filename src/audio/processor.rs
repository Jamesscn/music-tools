use super::common::Synth;
use ordered_float::OrderedFloat;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub type SynthRef = Arc<Mutex<Box<dyn Synth + Sync + Send>>>;

/// A structure used to generate a single audio signal given multiple frequencies and synthesizers.
#[derive(Clone)]
pub struct AudioProcessor {
    frequencies: Vec<(SynthRef, HashSet<OrderedFloat<f32>>)>,
    current_sample: Option<f32>,
    sample_rate: u32,
    volume: f32,
}

impl AudioProcessor {
    /// Creates a new audio processor with the default settings.
    pub fn new() -> Self {
        Self {
            frequencies: Vec::new(),
            current_sample: None,
            sample_rate: 44100,
            volume: 1.0,
        }
    }

    /// Adjusts the volume of all the synthesizers registered.
    ///
    /// # Parameters
    ///
    /// - `volume`: An [`f32`] which represents the master volume of the audio processor, which must
    ///   be between 0.0 and 1.0. Volumes outside of this range are clamped.
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    /// Returns an [`f32`] representing the current sample output of the audio processor. This
    /// sample will remain the same until the advance_sample() function is called.
    pub fn get_current_sample(&mut self) -> f32 {
        if let Some(sample) = self.current_sample {
            sample
        } else {
            let mut sample = 0.0;
            let mut active_synths = 0;
            for (synth, _) in self.frequencies.iter_mut() {
                let synth_sample = synth.lock().unwrap().get_sample();
                sample += synth_sample;
                active_synths += 1;
            }
            sample = (sample * self.volume / (active_synths as f32).sqrt()).clamp(-1.0, 1.0);
            self.current_sample = Some(sample);
            sample
        }
    }

    /// Tells the audio processor to advance to the next sample.
    pub fn advance_sample(&mut self) {
        for (synth, _) in self.frequencies.iter_mut() {
            synth.lock().unwrap().advance_sample(self.sample_rate);
        }
        self.current_sample = None;
    }

    /// Sets the sample rate of the audio processor.
    ///
    /// # Parameters
    ///
    /// - `sample_rate`: A [`u32`] representing the sample rate to be set in hertz.
    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate;
    }

    /// Gets a [`u32`] that represents the sample rate of the audio processor in hertz.
    pub fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// Registers a synthesizer so that it can be used to generate an audio signal. The synthesizer
    /// is stored within the audio processor and a [`SynthRc`] is returned which can be used to
    /// reference the synthesizer and play frequencies through that synthesizer.
    ///
    /// # Parameters
    ///
    /// - `synth`: A [`Box<dyn Synth>`] which is a boxed synthesizer to store.
    pub fn register_synth(&mut self, synth: impl Synth + Sync + Send + 'static) -> SynthRef {
        self.frequencies
            .push((Arc::new(Mutex::new(Box::new(synth))), HashSet::new()));
        Arc::clone(&self.frequencies.last().unwrap().0)
    }

    /// Unregisters or drops a synthesizer stored in the processor given its [`SynthRc`] reference.
    ///
    /// # Parameters
    ///
    /// - `synth`: A reference to the [`SynthRc`] of the synthesizer to drop.
    pub fn unregister_synth(&mut self, synth: &SynthRef) {
        for (index, (stored_synth, _)) in self.frequencies.iter().enumerate() {
            if Arc::ptr_eq(stored_synth, synth) {
                self.frequencies.remove(index);
                return;
            }
        }
    }

    /// Unregisters or drops all synthesizers stored in the processor.
    pub fn unregister_all_synths(&mut self) {
        self.frequencies.clear();
    }

    /// Starts playing a specific frequency on one of the registered synthesizers.
    ///
    /// # Parameters
    ///
    /// - `frequency`: An [`f32`] representing the frequency in hertz that will be played.
    /// - `synth`: A reference to the [`SynthRc`] of the synthesizer that will play the frequency.
    pub fn start_frequency(&mut self, frequency: f32, synth: &SynthRef) {
        for (stored_synth, set) in self.frequencies.iter_mut() {
            if Arc::ptr_eq(stored_synth, synth) {
                if set.insert(OrderedFloat(frequency)) {
                    stored_synth.lock().unwrap().add_voice(frequency);
                }
                return;
            }
        }
    }

    /// Stops playing a specific frequency on one of the registered synthesizers.
    ///
    /// # Parameters
    ///
    /// - `frequency`: An [`f32`] representing the frequency in hertz that will stop being played.
    /// - `synth`: A reference to the [`SynthRc`] of the synthesizer that is playing the frequency.
    pub fn stop_frequency(&mut self, frequency: f32, synth: &SynthRef) {
        for (stored_synth, set) in self.frequencies.iter_mut() {
            if Arc::ptr_eq(stored_synth, synth) {
                if set.remove(&OrderedFloat(frequency)) {
                    stored_synth.lock().unwrap().remove_voice(frequency);
                }
                return;
            }
        }
    }

    /// Stops playing all frequencies across all the registered synthesizers.
    pub fn stop_all_frequencies(&mut self) {
        for (synth, set) in self.frequencies.iter_mut() {
            synth.lock().unwrap().clear_voices();
            set.clear();
        }
    }

    /// Renders out a [`Vec<f32>`] of sample outputs of the audio processor for a given duration.
    ///
    /// # Parameters
    ///
    /// - `duration`: An [`Duration`] representing the length of time that will be used to capture
    ///   and store the samples in the output table.
    pub fn render(&mut self, duration: Duration) -> Vec<f32> {
        let samples = (duration.as_secs_f64() * self.sample_rate as f64) as usize;
        let mut table: Vec<f32> = Vec::with_capacity(samples);
        for _ in 0..samples {
            table.push(self.get_current_sample());
            self.advance_sample();
        }
        table
    }
}

impl Default for AudioProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for AudioProcessor {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let sample = self.get_current_sample();
        self.advance_sample();
        Some(sample)
    }
}
