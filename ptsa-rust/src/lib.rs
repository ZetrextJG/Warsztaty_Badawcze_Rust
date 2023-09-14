use chrono::Utc;
use pyo3::prelude::*;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::{
    f64::INFINITY,
    sync::{Arc, Mutex},
    thread::{self, ScopedJoinHandle},
};
use utils::{
    matrix::DistanceMatrix,
    params::Params,
    solution::{ComputedSolution, Solution},
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
    fn get_best_heuristic_solutions(
        &self,
        dmatrix: &DistanceMatrix,
        backward: bool,
    ) -> Vec<Solution> {
        let create_heuristic_solution = if backward {
            Solution::backwards_nearest_neightbor_solution
        } else {
            Solution::nearest_neightbor_solution
        };
        let mut all_heuristic_solutions: Vec<(Solution, f64)> = (0..dmatrix.size)
            .map(|starting_city| create_heuristic_solution(dmatrix, starting_city))
            .map(|solution| {
                let cost = solution.cost(dmatrix);
                (solution, cost)
            })
            .collect();
        all_heuristic_solutions.sort_by(|(_, a), (_, b)| a.total_cmp(b));
        let takes = (all_heuristic_solutions.len() as f64 * 0.10) as usize;
        all_heuristic_solutions
            .into_iter()
            .take(takes)
            .map(|(solution, _)| solution)
            .collect()
    }

    #[inline]
    fn init_states<'a>(
        &self,
        distance_matrix: &'a DistanceMatrix,
        starting_solutions: Vec<Solution>,
    ) -> StatesContainer<'a> {
        // Initialization
        let problem_size = distance_matrix.size;
        for solution in starting_solutions.iter() {
            assert_eq!(solution.size, problem_size);
        }
        let temp_bounds = TemperatureBounds {
            max: self.params.max_temperature,
            min: self.params.min_temperature,
        };
        // Creating states
        let mut states: StatesContainer =
            StatesContainer::new(temp_bounds.clone(), distance_matrix);
        for solution in starting_solutions.into_iter() {
            let temperature =
                temp_bounds.random_temperature(self.params.temp_beta_a, self.params.temp_beta_b);
            let is_shuffle_transition = rand::random::<f64>() < self.params.probability_of_shuffle;
            let state = State {
                solution,
                temperature,
                is_shuffle_transition,
            };
            states.add(state);
        }
        states
    }

    fn create_inital_states<'a>(
        &self,
        n: usize,
        distance_matrix: &'a DistanceMatrix,
        heuristic_solutions: &Vec<Solution>,
    ) -> StatesContainer<'a> {
        let rng = &mut thread_rng();
        let solutions: Vec<Solution> = (0..n)
            .map(|_| {
                let take_heuristic = rng.gen_range(0.0..1.0) < self.params.probability_of_heuristic;
                if take_heuristic {
                    heuristic_solutions.choose(rng).unwrap().clone()
                } else {
                    Solution::random_solution(distance_matrix.size)
                }
            })
            .collect();
        // TODO: Make it just an iterator. Do not collect into vector
        self.init_states(distance_matrix, solutions)
    }

    fn run_thread(
        &self,
        mut states: StatesContainer,
        time: i64,
        global_best: Arc<Mutex<f64>>,
        thead_id: usize,
    ) -> ComputedSolution {
        let deadline = Utc::now().timestamp() + time;
        // Main loop
        loop {
            // Break condition
            if Utc::now().timestamp() >= deadline {
                return ComputedSolution {
                    solution: states.best_solution.unwrap(),
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

            // Update global best
            {
                let mut global_best_cost = global_best.lock().unwrap();
                if states.best_cost < *global_best_cost {
                    *global_best_cost = states.best_cost;
                    println!("{} -> {} -> {}", Utc::now(), thead_id, states.best_cost)
                }
            }
        }
    }

    fn run(&self, dmatrix: DistanceMatrix, time: i64) -> ComputedSolution {
        let mut heuristic_solutions = self.get_best_heuristic_solutions(&dmatrix, false);
        {
            // This is  here to empty memory faster from second vector
            let mut backward_heuristic_solutions =
                self.get_best_heuristic_solutions(&dmatrix, true);
            heuristic_solutions.append(&mut backward_heuristic_solutions);
        }

        // Just do one run of it
        println!("Starting SEARCH part");
        let n = self.params.number_of_repeats;

        let global_best = Arc::new(Mutex::new(INFINITY));
        let mut results: Vec<ComputedSolution> = thread::scope(|s| {
            let handlers: Vec<ScopedJoinHandle<'_, ComputedSolution>> = (0..n)
                .map(|i| {
                    println!("Starting thread number {}.", i);
                    let initial_states = self.create_inital_states(
                        self.params.number_of_states,
                        &dmatrix,
                        &heuristic_solutions,
                    );
                    let thead_global_best = Arc::clone(&global_best);
                    s.spawn(move || self.run_thread(initial_states, time, thead_global_best, i))
                })
                .collect();
            handlers
                .into_iter()
                .map(|handle| handle.join().unwrap())
                .collect()
        });
        results.sort_by(|a, b| a.cost.total_cmp(&b.cost));
        let best_solution = results.get(0).unwrap().clone();
        println!("Finished searching for solutions.");
        println!(
            "Currently the best solution has cost of: {}",
            best_solution.cost
        );
        best_solution
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
        Ok((best_result.solution.path, best_result.cost))
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn ptsa_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PtsaAlgorithm>()?;
    Ok(())
}
