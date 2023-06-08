use pyo3::{prelude::*, pyclass_init::PyNativeTypeInitializer};
use std::sync::{Arc, Mutex};
use std::thread;
use rand_distr::{Distribution, Beta};


// calculate_distance.py
fn cycle_length(cycle: &Vec<usize>, distnace_matrix: &Vec<Vec<f64>>) -> f64 {
    // Calculate the length of that cycle using the distance matrix
    //
    let mut length = 0.0;
    for i in 0..cycle.len() {
        let j = (i + 1) % cycle.len();
        length += distnace_matrix[cycle[i]][cycle[j]];
    }
    length += distnace_matrix[cycle[cycle.len() - 1]][cycle[0]];
    return length;
}

// cooling.py
fn cooling(temperature: f64, cooling_rate: &f64, min_temperature: &f64) -> f64 { 
    let new_temperature = temperature * cooling_rate;
    return f64::max(new_temperature, min_temperature.clone());
}

// initialization.py
fn initialize_temperatures(n:&usize, min: &f64, max: &f64, a: &f64, b: &f64) -> Vec<f64> {
    // Probably suboptimal, but it works
    let beta = Beta::new(a.clone(), b.clone()).unwrap();
    let beta_samples: Vec<f64> = beta.sample_iter(rand::thread_rng()).take(n.clone()).collect();
    beta_samples
        .iter()
        .map(|x| x * (max - min) + min)
        .collect()
}

fn initialize_transition_function_types(n: &usize, probability_of_shuffle: &f64) -> Vec<bool> {
    (0..n.clone())
        .map(|_| rand::random::<f64>() < probability_of_shuffle.clone())
        .collect()
}


// metropolis_transition.py
fn acceptance(solution_length: &f64, new_solution_length: &f64, temperature: &f64) -> bool {
    let acceptance_probability = f64::exp(-(new_solution_length - solution_length) / temperature);
    rand::random::<f64>() < f64::min(1.0, acceptance_probability)
}


#[pyclass]
#[derive(Clone)]
pub struct Parameters {
    min_temperature: f64,
    max_temperature: f64,
    probability_of_shuffle: f64,
    probability_of_heuristic: f64,
    max_runtime_sec: f64,
    number_of_concurrent_threads: i32,
    max_length_percent_of_cycle: f64,
    swap_states_probability: f64,
    closeness: f64,
    cooling_rate: f64
}

type Matrix = Vec<Vec<f64>>;
#[pyclass]
pub struct PtsaAlgorithm {
    distnace_matrix: Arc<Mutex<Matrix>>,
    parameters: Arc<Mutex<Parameters>>,

    // initial_temperatures: Arc<Mutex<Vec<f64>>>,

    best_solution: Vec<usize>,
    best_solution_length: f64
}

impl PtsaAlgorithm {
    pub fn run(&self) {

    }
}

#[pymethods]
impl PtsaAlgorithm {
    #[new]
    pub fn new(distnace_matrix: Vec<Vec<f64>>, parameters: Parameters) -> Self {
        PtsaAlgorithm {
            distnace_matrix: Arc::new(Mutex::new(distnace_matrix)),
            // initial_temperatures: Arc::new(Mutex::new(initial_temperatures)),
            parameters: Arc::new(Mutex::new(parameters)),
            best_solution: vec![],
            best_solution_length: f64::MAX
        }
    }

}

/// A Python module implemented in Rust.
#[pymodule]
fn ptsa_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PtsaAlgorithm>()?;
    Ok(())
}
