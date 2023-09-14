use rand::thread_rng;
use rand_distr::{Beta, Distribution};

#[derive(Debug, Clone)]
pub struct TemperatureBounds {
    pub max: f64,
    pub min: f64,
}

impl TemperatureBounds {
    #[inline]
    pub fn random_temperature(&self, a: f64, b: f64) -> f64 {
        let beta = Beta::new(a, b).unwrap();
        beta.sample_iter(&mut thread_rng()).next().unwrap()
    }
}
