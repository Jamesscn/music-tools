use super::common::{Synth, Waveforms};

/**
 * A structure used to play a specific wavetable at a specific frequency.
 */
#[derive(Copy, Clone, Debug)]
struct WavetableVoice {
    frequency: f32,
    table_index: f32,
}

impl WavetableVoice {
    pub fn new(frequency: f32) -> Self {
        Self {
            frequency,
            table_index: 0.0,
        }
    }

    pub fn add_delta_time(&mut self, table_size: usize, sample_rate: u32) {
        let table_delta = self.frequency * table_size as f32 / sample_rate as f32;
        self.table_index += table_delta;
        self.table_index %= table_size as f32;
    }

    pub fn get_frequency(&self) -> f32 {
        self.frequency
    }

    pub fn get_table_index(&self) -> f32 {
        self.table_index
    }
}

impl PartialEq for WavetableVoice {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}

impl Eq for WavetableVoice {}

/// A structure which holds a wavetable oscillator.
///
/// A wavetable oscillator is used to store the shape of a wave in a table or an array which can
/// later be played at a specific frequency. There are several advantages to storing a wave this
/// way, most notably:
///
/// - Efficiency: It is more efficient to use a lookup table to store certain shapes of waves such
///   as sine waves than to call the sin() function.
/// - Timbre: It is easy to change the shape of the wave to something more complex such as a square,
///   sawtooth or triangle wave.
///
/// This implementation of a wavetable oscillator also allows you to play multiple frequencies of
/// the wave at the same time.
///
/// # Examples
///
/// ```rust
/// use music_tools::audio::player::AudioPlayer;
/// use music_tools::audio::common::Waveforms;
/// use music_tools::audio::wavetable::WavetableOscillator;
/// use music_tools::common::Beat;
/// use music_tools::note::Note;
///
/// let mut square_oscillator = WavetableOscillator::new(Waveforms::SQUARE_WAVE, 1.0, 128);
/// let mut player = AudioPlayer::try_new().unwrap();
/// player.set_synth(square_oscillator);
/// player.push(&Note::from_string("A4").unwrap(), &Beat::WHOLE);
/// player.play();
/// ```
#[derive(Clone, Debug)]
pub struct WavetableOscillator {
    wavetable: Vec<f32>,
    voices: Vec<WavetableVoice>,
    volume: f32,
}

impl WavetableOscillator {
    /// Creates a new wavetable given a function of the height of the audio signal between -1 and 1
    /// with respect to time, returning the index of this new wavetable. All values outside this
    /// range are clamped. The function returns a [`usize`] which is the index of the stored
    /// wavetable, it can be used to reference or play the wavetable later.
    ///
    /// # Parameters
    ///
    /// - `wave_function`: The function used to generate the shape of the wave that will be played
    ///   by the new wavetable. It must receive a parameter of type [`f32`] representing the time
    ///   value of the wave between 0 and `max_time`, and it must return an [`f32`] representing the
    ///   height of the wave at that time between -1 and 1.
    /// - `max_time`: This parameter scales the time variable that is passed to `wave_function`.
    /// - `wavetable_size`: The amount of points to store in the wavetable. The higher this value,
    /// the higher the quality of the signal at the cost of a higher memory consumption. A value of
    /// 128 is recommended.
    pub fn new(wave_function: fn(f32) -> f32, max_time: f32, wavetable_size: usize) -> Self {
        let mut wavetable = Vec::with_capacity(wavetable_size);
        for i in 0..wavetable_size {
            let time_value = i as f32 / wavetable_size as f32;
            let wave_value = wave_function(max_time * time_value).clamp(-1.0, 1.0);
            wavetable.push(wave_value);
        }
        Self {
            wavetable,
            voices: Vec::new(),
            volume: 0.2,
        }
    }

    pub fn get_wavetable(&self) -> Vec<f32> {
        self.wavetable.clone()
    }
}

impl Synth for WavetableOscillator {
    fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    fn clear_voices(&mut self) {
        self.voices.clear();
    }

    fn add_voice(&mut self, frequency: f32) {
        self.voices.push(WavetableVoice::new(frequency));
    }

    fn remove_voice(&mut self, frequency: f32) {
        if let Some(index) = self
            .voices
            .iter()
            .position(|voice| voice.get_frequency() == frequency)
        {
            self.voices.remove(index);
        }
    }

    fn get_sample(&mut self) -> f32 {
        let mut sample = 0.0;
        let mut active_voices = 0;
        for voice in &mut self.voices {
            let table_size = self.wavetable.len();
            let current_index = voice.get_table_index() as usize;
            let next_index = (current_index + 1) % table_size;
            let lerp_frac = voice.get_table_index() - current_index as f32;
            let current_value = self.wavetable[current_index];
            let next_value = self.wavetable[next_index];
            let lerp_value = current_value + lerp_frac * (next_value - current_value);
            sample += lerp_value;
            active_voices += 1;
        }
        if active_voices == 0 {
            0.0
        } else {
            (sample * self.volume / (active_voices as f32).sqrt()).clamp(-1.0, 1.0)
        }
    }

    fn advance_sample(&mut self, sample_rate: u32) {
        for voice in &mut self.voices {
            let table_size = self.wavetable.len();
            voice.add_delta_time(table_size, sample_rate);
        }
    }
}

impl Default for WavetableOscillator {
    fn default() -> Self {
        Self::new(Waveforms::SINE_WAVE, 1.0, 128)
    }
}

impl PartialEq for WavetableOscillator {
    fn eq(&self, other: &Self) -> bool {
        self.wavetable == other.wavetable
    }
}

impl Eq for WavetableOscillator {}

impl From<&[f32]> for WavetableOscillator {
    fn from(value: &[f32]) -> Self {
        Self {
            wavetable: value.iter().map(|value| value.clamp(-1.0, 1.0)).collect(),
            voices: Vec::new(),
            volume: 0.2,
        }
    }
}
