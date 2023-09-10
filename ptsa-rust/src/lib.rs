use chrono::Utc;
use pyo3::prelude::*;
use rand::{seq::SliceRandom, thread_rng, Rng};
use utils::{
    helpers::initialize_transition_function_types,
    matrix::DistanceMatrix,
    params::Params,
    solution::Solution,
    state::{State, StatesContainer},
    temp::TemperatureBounds,
};
mod utils;

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

    fn run(&self, dmatrix: DistanceMatrix, deadline: i64) -> (Solution, f64) {
        // Initialization
        let size = dmatrix.size;
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
        let heuristic_solutions = self.get_best_heuristic_solutions(&dmatrix);

        // Creating states
        let mut states: StatesContainer = StatesContainer::new(temp_bounds, dmatrix);
        let mut rng = thread_rng();
        for (temperature, is_transion_shuffle) in temperatures.into_iter().zip(shuffle_bool_vector)
        {
            let take_heuristic = rng.gen_range(0.0..1.0) < self.params.probability_of_heuristic;
            let solution: Solution = if take_heuristic {
                heuristic_solutions.choose(&mut rng).unwrap().clone()
            } else {
                Solution::random_solution(size)
            };
            let state = State {
                solution,
                temperature,
                is_transion_shuffle,
            };
            states.add(state);
        }

        // Main loop
        let mut loop_counter: usize = 0;
        loop {
            // Break condition
            if Utc::now().timestamp() >= deadline {
                println!("This loop was run {} times", loop_counter);
                return (states.best_solution.unwrap(), states.best_cost);
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
            // Additional
            loop_counter += 1;
        }
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
        let deadline = Utc::now().timestamp() + time;
        let dmatrix = DistanceMatrix::new(matrix);

        println!("Rust solver. Start!");
        let (best_solution, cost) = self.run(dmatrix, deadline);
        Ok((best_solution.path, cost))
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn ptsa_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PtsaAlgorithm>()?;
    Ok(())
}
