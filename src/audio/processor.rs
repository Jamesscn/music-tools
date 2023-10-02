use super::common::Synth;
use ordered_float::OrderedFloat;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::time::Duration;

pub type SynthRc = Rc<RefCell<Box<dyn Synth>>>;

pub struct AudioProcessor {
    frequencies: Vec<(SynthRc, HashSet<OrderedFloat<f32>>)>,
    current_sample: Option<f32>,
    sample_rate: u32,
    volume: f32,
}

impl AudioProcessor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    pub fn get_current_sample(&mut self) -> f32 {
        if let Some(sample) = self.current_sample {
            sample
        } else {
            let mut sample = 0.0;
            let mut active_synths = 0;
            for (synth, _) in self.frequencies.iter_mut() {
                let synth_sample = synth.borrow_mut().get_sample();
                sample += synth_sample;
                active_synths += 1;
            }
            sample = (sample * self.volume / (active_synths as f32).sqrt()).clamp(-1.0, 1.0);
            self.current_sample = Some(sample);
            sample
        }
    }

    pub fn advance_sample(&mut self) {
        for (synth, _) in self.frequencies.iter_mut() {
            synth.borrow_mut().advance_sample(self.sample_rate);
        }
        self.current_sample = None;
    }

    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate;
    }

    pub fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn register_synth(&mut self, synth: Box<dyn Synth>) -> SynthRc {
        let reference = Rc::new(RefCell::new(synth));
        self.frequencies.push((reference, HashSet::new()));
        self.frequencies.last().unwrap().0.clone()
    }

    pub fn unregister_synth(&mut self, synth: &SynthRc) {
        for (index, (stored_synth, _)) in self.frequencies.iter().enumerate() {
            if Rc::ptr_eq(stored_synth, synth) {
                self.frequencies.remove(index);
                return;
            }
        }
    }

    pub fn unregister_all_synths(&mut self) {
        self.frequencies.clear();
    }

    pub fn start_frequency(&mut self, frequency: f32, synth: &SynthRc) {
        for (stored_synth, set) in self.frequencies.iter_mut() {
            if Rc::ptr_eq(stored_synth, synth) {
                stored_synth.borrow_mut().add_voice(frequency);
                set.insert(OrderedFloat(frequency));
                return;
            }
        }
    }

    pub fn stop_frequency(&mut self, frequency: f32, synth: &SynthRc) {
        for (stored_synth, set) in self.frequencies.iter_mut() {
            if Rc::ptr_eq(stored_synth, synth) {
                stored_synth.borrow_mut().remove_voice(frequency);
                set.insert(OrderedFloat(frequency));
                return;
            }
        }
    }

    pub fn stop_all_frequencies(&mut self) {
        for (synth, _) in self.frequencies.iter_mut() {
            synth.borrow_mut().clear_voices();
        }
    }

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
        Self {
            frequencies: Vec::new(),
            current_sample: None,
            sample_rate: 44100,
            volume: 1.0,
        }
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
