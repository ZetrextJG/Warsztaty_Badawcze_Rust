use chrono::Utc;
use const_env::from_env;
use pyo3::prelude::*;
use utils::{
    matrix::DistanceMatrix,
    solution::Solution,
    state::{State, StatesContainer},
    temp::TemperatureBounds,
};
mod utils;

#[from_env]
const DIMENSION: usize = 10;

#[pyclass]
#[derive(Clone)]
pub struct Parameters {
    pub min_temperature: f64,
    pub max_temperature: f64,
    pub probability_of_shuffle: f64,
    pub probability_of_heuristic: f64,
    pub number_of_concurrent_threads: i32,
    pub max_length_percent_of_cycle: f64,
    pub swap_states_probability: f64,
    pub closeness: f64,
    pub cooling_rate: f64,
}

#[pyclass]
pub struct PtsaAlgorithm {
    pub parameters: Parameters,
}

impl PtsaAlgorithm {
    fn run<const N: usize>(
        &self,
        distance_matrix: DistanceMatrix<N>,
        deadline: i64,
    ) -> Solution<N> {
        let best_solution = Solution::random_solution();
        let temp_bounds = TemperatureBounds {
            max: self.parameters.max_temperature,
            min: self.parameters.min_temperature,
        };
        let states: StatesContainer<N> = StatesContainer::new(temp_bounds, distance_matrix);
        // let state: State<N> = State {};

        loop {
            // This comparation might be a bottleneck
            // Break condition
            if Utc::now().timestamp() >= deadline {
                return best_solution;
            }
        }
    }
}

#[pymethods]
impl PtsaAlgorithm {
    #[new]
    pub fn new(parameters: Parameters) -> Self {
        PtsaAlgorithm { parameters }
    }

    pub fn run_till(&self, matrix: Vec<Vec<f64>>, deadline_timestamp: String) -> Vec<usize> {
        // Run the PTSA algorith on a given distance matrix till the specified deadline
        // Deadtime must be a string containing the number of seconds from the start of unix time.
        // Returns the best solution
        let timestamp = deadline_timestamp.parse::<i64>().unwrap();
        let dmatrix: DistanceMatrix<DIMENSION> = matrix.into();
        let best_solution = self.run(dmatrix, timestamp);
        best_solution.path.into()
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn ptsa_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PtsaAlgorithm>()?;
    Ok(())
}
