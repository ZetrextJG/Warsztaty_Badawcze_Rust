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
    fn run(&self, distance_matrix: DistanceMatrix, deadline: i64) -> (Solution, f64) {
        let size = distance_matrix.size;

        let temp_bounds = TemperatureBounds {
            max: self.params.max_temperature,
            min: self.params.min_temperature,
        };
        let mut all_heuristic_solutions: Vec<(Solution, f64)> = (0..size)
            .map(|starting_city| {
                Solution::nearest_neightbor_solution(&distance_matrix, starting_city)
            })
            // HACK: This is not a solution I would like to use
            .map(|solution| (solution.clone(), solution.cost(&distance_matrix)))
            .collect();

        all_heuristic_solutions.sort_by(|(_, a), (_, b)| a.total_cmp(b));

        // WARN: I choose to use 15% instead of 10% of the solutions
        let takes = (all_heuristic_solutions.len() as f64 * 0.15) as usize;
        let heuristic_solutions: Vec<Solution> = all_heuristic_solutions
            .into_iter()
            .take(takes)
            .map(|(solution, _)| solution)
            .collect();

        let shuffle_bool_vector = initialize_transition_function_types(
            self.params.number_of_states,
            self.params.probability_of_shuffle,
        );
        let temperatures = temp_bounds.init_temperatures(
            self.params.number_of_states,
            self.params.temp_beta_a,
            self.params.temp_beta_b,
        );

        let mut states: StatesContainer = StatesContainer::new(temp_bounds, distance_matrix);

        let mut rng = thread_rng();
        for (temperature, is_transion_shuffle) in temperatures.into_iter().zip(shuffle_bool_vector)
        {
            if rng.gen_range(0.0..1.0) < self.params.probability_of_heuristic {
                states.add(State {
                    solution: heuristic_solutions.choose(&mut rng).unwrap().clone(),
                    temperature,
                    is_transion_shuffle,
                })
            } else {
                states.add(State {
                    solution: Solution::random_solution(size),
                    temperature,
                    is_transion_shuffle,
                })
            }
        }

        loop {
            // This comparison might be a bottleneck
            // Break condition
            if Utc::now().timestamp() >= deadline {
                let (best_solution, cost) = states.best_solution();
                return (best_solution.clone(), cost);
            }

            for _ in 0..self.params.number_of_repeats {
                states.metropolis_tranision(self.params.max_length_percent_of_cycle);
                for _ in 0..states.states.len() {
                    states.replica_transition(
                        self.params.swap_states_probability,
                        self.params.closeness,
                    )
                }
            }
            states.cool(self.params.cooling_rate);
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

    pub fn run_till(&self, matrix: Vec<Vec<f64>>, deadline_timestamp: String) -> (Vec<usize>, f64) {
        // Run the PTSA algorith on a given distance matrix till the specified deadline
        // Deadtime must be a string containing the number of seconds from the start of unix time.
        // Returns the best solution
        let timestamp = deadline_timestamp.parse::<i64>().unwrap();
        let dmatrix = DistanceMatrix::new(matrix);
        let (best_solution, cost) = self.run(dmatrix, timestamp);
        (best_solution.path, cost)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn ptsa_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PtsaAlgorithm>()?;
    Ok(())
}
