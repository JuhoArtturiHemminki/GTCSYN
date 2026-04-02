use gtcsyn::PHI;
use std::f64::consts::PI;

pub struct GtcSynthesizer {
    pub sample_rate: f64,
}

impl GtcSynthesizer {
    pub fn new(sample_rate: f64) -> Self {
        Self { sample_rate }
    }

    pub fn generate_schedule(&self, bits: &[u8]) -> Vec<f64> {
        bits.iter().enumerate().map(|(n, _)| {
            let phase = (n as f64 * PHI) % (2.0 * PI);
            let offset = (phase / (2.0 * PI)) * self.sample_rate;
            (n as f64 * self.sample_rate) + offset
        }).collect()
    }
}

