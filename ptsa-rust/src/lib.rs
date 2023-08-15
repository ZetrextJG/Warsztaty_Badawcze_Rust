use chrono::Utc;
use const_env::from_env;
use pyo3::prelude::*;
use rand::{seq::SliceRandom, thread_rng, Rng};
use utils::{
    helpers::initialize_transition_function_types,
    matrix::DistanceMatrix,
    solution::Solution,
    state::{State, StatesContainer},
    temp::TemperatureBounds,
};
mod utils;

#[from_env("DIM")]
const DIMENSION: usize = 10;

#[pyclass]
#[derive(Clone)]
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

#[pymethods]
impl Params {
    #[new]
    pub fn new(
        number_of_states: usize,
        number_of_repeats: usize,
        number_of_concurrent_threads: usize,
        min_temperature: f64,
        max_temperature: f64,
        probability_of_shuffle: f64,
        probability_of_heuristic: f64,
        temp_beta_a: f64,
        temp_beta_b: f64,
        max_length_percent_of_cycle: f64,
        swap_states_probability: f64,
        closeness: f64,
        cooling_rate: f64,
    ) -> Self {
        Params {
            number_of_states,
            number_of_repeats,
            number_of_concurrent_threads,
            min_temperature,
            max_temperature,
            probability_of_shuffle,
            probability_of_heuristic,
            temp_beta_a,
            temp_beta_b,
            max_length_percent_of_cycle,
            swap_states_probability,
            closeness,
            cooling_rate,
        }
    }
}

#[pyclass]
pub struct PtsaAlgorithm {
    pub params: Params,
}

impl PtsaAlgorithm {
    fn run<const N: usize>(
        &self,
        distance_matrix: DistanceMatrix<N>,
        deadline: i64,
    ) -> (Solution<N>, f64) {
        let temp_bounds = TemperatureBounds {
            max: self.params.max_temperature,
            min: self.params.min_temperature,
        };
        let mut all_heuristic_solutions: Vec<(Solution<N>, f64)> = (0..N)
            .map(|starting_city| {
                Solution::nearest_neightbor_solution(&distance_matrix, starting_city)
            })
            // HACK: This is not a solution I would like to use
            .map(|solution| (solution.clone(), solution.cost(&distance_matrix)))
            .collect();

        all_heuristic_solutions.sort_by(|(_, a), (_, b)| a.total_cmp(b));

        // WARN: I choose to use 15% instead of 10% of the solutions
        let takes = (all_heuristic_solutions.len() as f64 * 0.15) as usize;
        let heuristic_solutions: Vec<Solution<N>> = all_heuristic_solutions
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

        let mut states: StatesContainer<N> = StatesContainer::new(temp_bounds, distance_matrix);

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
                    solution: Solution::random_solution(),
                    temperature,
                    is_transion_shuffle,
                })
            }
        }

        loop {
            // This comparation might be a bottleneck
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
    pub fn new(params: Params) -> Self {
        PtsaAlgorithm { params }
    }

    #[staticmethod]
    pub fn dimension() -> usize {
        DIMENSION
    }

    pub fn run_till(&self, matrix: Vec<Vec<f64>>, deadline_timestamp: String) -> (Vec<usize>, f64) {
        // Run the PTSA algorith on a given distance matrix till the specified deadline
        // Deadtime must be a string containing the number of seconds from the start of unix time.
        // Returns the best solution
        let timestamp = deadline_timestamp.parse::<i64>().unwrap();
        let dmatrix: DistanceMatrix<DIMENSION> = matrix.into();
        let (best_solution, cost) = self.run(dmatrix, timestamp);
        (best_solution.path.into(), cost)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn ptsa_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PtsaAlgorithm>()?;
    m.add_class::<Params>()?;
    Ok(())
}
