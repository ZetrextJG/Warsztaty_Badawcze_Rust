use pyo3::prelude::*;

#[derive(FromPyObject)]
pub struct Params {
    pub number_of_states: usize,
    pub number_of_repeats: usize,
    pub number_of_concurrent_threads: usize,
    pub min_temperature: f64,
    pub max_temperature: f64,
    pub probability_of_shuffle: f64,
    pub probability_of_heuristic: f64,
    pub temp_beta_a: f64,
    pub temp_beta_b: f64,
    pub max_length_percent_of_cycle: f64,
    pub swap_states_probability: f64,
    pub closeness: f64,
    pub cooling_rate: f64,
}
