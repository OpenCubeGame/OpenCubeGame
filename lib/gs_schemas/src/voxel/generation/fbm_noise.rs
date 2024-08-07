//! bBm noise function with configurable per-octave strength.

use bevy_math::{DVec2, DVec3, DVec4};
use noise::{NoiseFn, Seedable};
use serde::{Deserialize, Serialize};

/// Noise function that outputs fBm (fractal Brownian motion) noise.
///
/// fBm is a _monofractal_ method. In essence, fBm has a _constant_ fractal
/// dimension. It is as close to statistically _homogeneous_ and _isotropic_
/// as possible. Homogeneous means "the same everywhere" and isotropic means
/// "the same in all directions" (note that the two do not mean the same
/// thing).
///
/// The main difference between fractal Brownian motion and regular Brownian
/// motion is that while the increments in Brownian motion are independent,
/// the increments in fractal Brownian motion depend on the previous increment.
///
/// fBm is the result of several noise functions of ever-increasing frequency
/// and ever-decreasing amplitude.
///
/// fBm is commonly referred to as Perlin noise.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fbm<T> {
    /// All frequency octaves to generate the noise with.
    ///
    /// The number of octaves control the _amount of detail_ in the noise
    /// function. Adding more octaves increases the detail, with the drawback
    /// of increasing the calculation time.
    pub octaves: Vec<f64>,

    /// The number of cycles per unit length that the noise function outputs.
    pub frequency: f64,

    /// A multiplier that determines how quickly the frequency increases for
    /// each successive octave in the noise function.
    ///
    /// The frequency of each successive octave is equal to the product of the
    /// previous octave's frequency and the lacunarity value.
    ///
    /// A lacunarity of 2.0 results in the frequency doubling every octave. For
    /// almost all cases, 2.0 is a good value to use.
    pub lacunarity: f64,

    /// A multiplier that determines how quickly the amplitudes diminish for
    /// each successive octave in the noise function.
    ///
    /// The amplitude of each successive octave is equal to the product of the
    /// previous octave's amplitude and the persistence value. Increasing the
    /// persistence produces "rougher" noise.
    pub persistence: f64,

    seed: u32,
    sources: Vec<T>,
    scale_factor: f64,
}

fn calc_scale_factor(octaves: &[f64]) -> f64 {
    let mut lowest_freq_value_factor = 2f64.powf(octaves.len() as f64 - 1.0) / (2f64.powf(octaves.len() as f64) - 1.0);
    let mut value = 0.0;
    for o in octaves.iter() {
        value += o * 2.0 * lowest_freq_value_factor;
        lowest_freq_value_factor /= 2.0;
    }
    value
}

impl<T> Fbm<T>
where
    T: Default + Seedable,
{
    /// Default seed for fBm noise.
    pub const DEFAULT_SEED: u32 = 0;
    /// Default octaves for fBm noise.
    pub const DEFAULT_OCTAVES: [f64; 6] = [1.0; 6];
    /// Default frequency for fBm noise.
    pub const DEFAULT_FREQUENCY: f64 = 1.0;
    /// Default lacynarity for fBm noise.
    pub const DEFAULT_LACUNARITY: f64 = core::f64::consts::PI * 2.0 / 3.0;
    /// Default persistence for fBm noise.
    pub const DEFAULT_PERSISTENCE: f64 = 0.5;
    /// Maximum amount of octaves for fBm noise.
    pub const MAX_OCTAVES: usize = 32;

    /// Creates a new instance of FBM noise.
    pub fn new(seed: u32) -> Self {
        let octaves = Self::DEFAULT_OCTAVES.to_vec();
        Self {
            seed,
            frequency: Self::DEFAULT_FREQUENCY,
            lacunarity: Self::DEFAULT_LACUNARITY,
            persistence: Self::DEFAULT_PERSISTENCE,
            sources: super::build_sources(seed, &octaves),
            scale_factor: calc_scale_factor(&octaves),
            octaves,
        }
    }

    /// Sets the octave list and returns a new fBm noise generator.
    pub fn set_octaves(&self, octaves: Vec<f64>) -> Self {
        Self {
            sources: super::build_sources(self.seed, &octaves),
            scale_factor: calc_scale_factor(&octaves),
            octaves,
            ..*self
        }
    }

    /// Sets the source noise generator for this instance of FBM noise.
    pub fn set_sources(self, sources: Vec<T>) -> Self {
        Self { sources, ..self }
    }

    /// Sets the frequency and returns a new fBm noise generator.
    pub fn set_frequency(self, frequency: f64) -> Self {
        Self { frequency, ..self }
    }

    /// Sets the lacunarity and returns a new fBm noise generator.
    pub fn set_lacunarity(self, lacunarity: f64) -> Self {
        Self { lacunarity, ..self }
    }

    /// Sets the persistence and returns a new fBm noise generator.
    pub fn set_persistence(self, persistence: f64) -> Self {
        Self { persistence, ..self }
    }

    /// Sets the seed for this noise.
    pub fn set_seed(&mut self, seed: u32) {
        if self.seed == seed {
            return;
        }

        self.seed = seed;
        self.sources = super::build_sources(seed, &self.octaves);
    }
}

impl<T> Default for Fbm<T>
where
    T: Default + Seedable,
{
    fn default() -> Self {
        Self::new(Self::DEFAULT_SEED)
    }
}

//impl<T> MultiFractal for Fbm<T>
//where
//    T: Default + Seedable,
//{
//    fn set_octaves(self, mut octaves: usize) -> Self {
//        if *self.octaves.get(0).unwrap() == octaves as u32 {
//            return self;
//        }
//
//
//        for x in 0..octaves {
//            octaves = octaves.clamp(1, Self::MAX_OCTAVES);
//        }
//        Self {
//            octaves,
//            sources: super::build_sources(self.seed, octaves),
//            scale_factor: Self::calc_scale_factor(self.persistence, octaves),
//            ..self
//        }
//    }
//
//    fn set_frequency(self, frequency: f64) -> Self {
//        Self { frequency, ..self }
//    }
//
//    fn set_lacunarity(self, lacunarity: f64) -> Self {
//        Self { lacunarity, ..self }
//    }
//
//    fn set_persistence(self, persistence: f64) -> Self {
//        Self {
//            persistence,
//            scale_factor: Self::calc_scale_factor(persistence, self.octaves),
//            ..self
//        }
//    }
//}

impl<T> Seedable for Fbm<T>
where
    T: Default + Seedable,
{
    fn set_seed(self, seed: u32) -> Self {
        if self.seed == seed {
            return self;
        }

        Self {
            seed,
            sources: super::build_sources(seed, &self.octaves),
            ..self
        }
    }

    fn seed(&self) -> u32 {
        self.seed
    }
}

/// 2-dimensional Fbm noise
impl<T> NoiseFn<f64, 2> for Fbm<T>
where
    T: NoiseFn<f64, 2>,
{
    fn get(&self, point: [f64; 2]) -> f64 {
        let mut point = DVec2::from_array(point);

        let mut result = 0.0;

        let mut attenuation = self.persistence;

        point *= self.frequency;

        for x in 0..self.octaves.len() {
            let mut signal = 0.0;
            let o = self.octaves[x];
            if o != 0.0 {
                // Get the signal.
                signal = self.sources[x].get(point.to_array());

                // Scale the result for this octave
                signal *= o;

                // Scale the amplitude appropriately for this frequency.
                signal *= attenuation;
            }

            // Increase the attenuation for the next octave, to be equal to persistence ^ (x + 1)
            attenuation *= attenuation;

            // Add the signal to the result.
            result += signal;

            // Increase the frequency for the next octave.
            point *= self.lacunarity;
        }

        // Scale the result into the [-1,1] range
        result * self.scale_factor
    }
}

/// 3-dimensional Fbm noise
impl<T> NoiseFn<f64, 3> for Fbm<T>
where
    T: NoiseFn<f64, 3>,
{
    fn get(&self, point: [f64; 3]) -> f64 {
        let mut point = DVec3::from_array(point);

        let mut result = 0.0;

        let mut attenuation = self.persistence;

        point *= self.frequency;

        for x in 0..self.octaves.len() {
            // Get the signal.
            let mut signal = self.sources[x].get(point.to_array());

            // Scale the result for this octave
            signal *= self.octaves[x];

            // Scale the amplitude appropriately for this frequency.
            signal *= attenuation;

            // Increase the attenuation for the next octave, to be equal to persistence ^ (x + 1)
            attenuation *= attenuation;

            // Add the signal to the result.
            result += signal;

            // Increase the frequency for the next octave.
            point *= self.lacunarity;
        }

        // Scale the result into the [-1,1] range
        result * self.scale_factor
    }
}

/// 4-dimensional Fbm noise
impl<T> NoiseFn<f64, 4> for Fbm<T>
where
    T: NoiseFn<f64, 4>,
{
    fn get(&self, point: [f64; 4]) -> f64 {
        let mut point = DVec4::from_array(point);

        let mut result = 0.0;

        let mut attenuation = self.persistence;

        point *= self.frequency;

        for x in 0..self.octaves.len() {
            // Get the signal.
            let mut signal = self.sources[x].get(point.to_array());

            // Scale the amplitude for this octave
            signal *= self.octaves[x];

            // Scale the amplitude appropriately for this frequency.
            signal *= attenuation;

            // Increase the attenuation for the next octave, to be equal to persistence ^ (x + 1)
            attenuation *= attenuation;

            // Add the signal to the result.
            result += signal;

            // Increase the frequency for the next octave.
            point *= self.lacunarity;
        }

        // Scale the result into the [-1,1] range
        result * self.scale_factor
    }
}
