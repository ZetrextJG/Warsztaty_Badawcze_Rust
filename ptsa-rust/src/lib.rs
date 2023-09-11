use chrono::Utc;
use pyo3::prelude::*;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::thread::{self, ScopedJoinHandle};
use utils::{
    helpers::initialize_transition_function_types,
    matrix::DistanceMatrix,
    params::Params,
    solution::Solution,
    state::{State, StatesContainer},
    temp::TemperatureBounds,
};
mod utils;

#[derive(Clone, Debug)]
struct AlgResult {
    path: Vec<usize>,
    cost: f64,
}

#[pyclass]
pub struct PtsaAlgorithm {
    pub params: Params,
}

impl PtsaAlgorithm {
    #[inline]
    fn get_best_heuristic_solutions(&self, dmatrix: &DistanceMatrix) -> Vec<Solution> {
        let mut all_heuristic_solutions: Vec<(Solution, f64)> = (0..dmatrix.size)
            .map(|starting_city| Solution::nearest_neightbor_solution(dmatrix, starting_city))
            .map(|solution| {
                let cost = solution.cost(dmatrix);
                (solution, cost)
            })
            .collect();
        all_heuristic_solutions.sort_by(|(_, a), (_, b)| a.total_cmp(b));
        let takes = (all_heuristic_solutions.len() as f64 * 0.1) as usize;
        all_heuristic_solutions
            .into_iter()
            .take(takes)
            .map(|(solution, _)| solution)
            .collect()
    }

    fn init_states<'a>(&self, distance_matrix: &'a DistanceMatrix) -> StatesContainer<'a> {
        let problem_size = distance_matrix.size;
        let shuffle_bool_vector = initialize_transition_function_types(
            self.params.number_of_states,
            self.params.probability_of_shuffle,
        );
        let temp_bounds = TemperatureBounds {
            max: self.params.max_temperature,
            min: self.params.min_temperature,
        };
        let temperatures = temp_bounds.init_temperatures(
            self.params.number_of_states,
            self.params.temp_beta_a,
            self.params.temp_beta_b,
        );
        let heuristic_solutions = self.get_best_heuristic_solutions(distance_matrix);

        // Creating states
        let mut states: StatesContainer = StatesContainer::new(temp_bounds, distance_matrix);
        let mut rng = thread_rng();
        for (temperature, is_transion_shuffle) in temperatures.into_iter().zip(shuffle_bool_vector)
        {
            let take_heuristic = rng.gen_range(0.0..1.0) < self.params.probability_of_heuristic;
            let solution: Solution = if take_heuristic {
                heuristic_solutions.choose(&mut rng).unwrap().clone()
            } else {
                Solution::random_solution(problem_size)
            };
            let state = State {
                solution,
                temperature,
                is_transion_shuffle,
            };
            states.add(state);
        }

        states
    }

    fn run_single(&self, dmatrix: &DistanceMatrix, time: i64) -> AlgResult {
        let deadline = Utc::now().timestamp() + time;
        // Initialization
        let mut states = self.init_states(dmatrix);
        // Main loop
        loop {
            // Break condition
            if Utc::now().timestamp() >= deadline {
                return AlgResult {
                    path: states.best_solution.unwrap().path,
                    cost: states.best_cost,
                };
            }
            // Metropolis and replica transitions
            for _ in 0..self.params.number_of_repeats {
                states.metropolis_tranision(self.params.max_length_percent_of_cycle);
                for _ in 0..states.states.len() {
                    states.replica_transition(
                        self.params.swap_states_probability,
                        self.params.closeness,
                    )
                }
            }
            // Cooling
            states.cool(self.params.cooling_rate);
        }
    }

    fn run(&self, dmatrix: DistanceMatrix, time: i64) -> AlgResult {
        let mut results: Vec<AlgResult> = thread::scope(|s| {
            let n = self.params.number_of_threads;
            let handlers: Vec<ScopedJoinHandle<'_, AlgResult>> = (0..n)
                .map(|i| {
                    println!("Starting thread number {}.", i);
                    s.spawn(|| self.run_single(&dmatrix, time))
                })
                .collect();
            handlers
                .into_iter()
                .map(|handle| handle.join().unwrap())
                .collect()
        });
        // Get the best result
        results.sort_by(|a, b| a.cost.total_cmp(&b.cost));
        results[0].clone()
    }
}

#[pymethods]
impl PtsaAlgorithm {
    #[new]
    pub fn new(parameters: PyObject) -> Self {
        Python::with_gil(|py| {
            let params: Params = parameters.extract(py).unwrap();
            PtsaAlgorithm { params }
        })
    }

    pub fn run_for(&self, matrix: Vec<Vec<f64>>, time: i64) -> PyResult<(Vec<usize>, f64)> {
        // Run the PTSA algorithm on a given distance matrix
        // for specified about of time (in seconds)
        let dmatrix = DistanceMatrix::new(matrix);

        println!("Rust solver. Start!");
        println!("See you in {} seconds!", time);
        let best_result = self.run(dmatrix, time);

        Ok((best_result.path, best_result.cost))
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn ptsa_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PtsaAlgorithm>()?;
    Ok(())
}
