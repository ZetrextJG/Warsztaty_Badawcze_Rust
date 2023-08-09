use rand_distr::{Beta, Distribution};

#[derive(Debug)]
pub struct TemperatureBounds {
    pub max: f64,
    pub min: f64,
}

impl TemperatureBounds {
    pub fn new(min: f64, max: f64) -> TemperatureBounds {
        assert!(max >= min);
        TemperatureBounds { max, min }
    }

    pub fn init_temperatures(&self, n: usize, a: f64, b: f64) -> Vec<f64> {
        let beta = Beta::new(a, b).unwrap();
        let beta_samples: Vec<f64> = beta.sample_iter(rand::thread_rng()).take(n).collect();
        beta_samples
            .iter()
            .map(|x| x * (self.max - self.min) + self.min)
            .collect()
    }
}
