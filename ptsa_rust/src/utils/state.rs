use rand::{seq::SliceRandom, thread_rng, Rng};

use super::path::Solution;

#[derive(Debug)]
pub struct State<const N: usize> {
    pub temperature: f64,
    pub solution: Solution<N>,
    pub solution_cost: f64,
}

#[derive(Debug)]
pub struct StateSeries {
    pub states: Vec<State>,
    pub min_temperature: f64,
    pub best_solution_cost: f64,
}

impl Default for StateSeries {
    fn default(min_temperature: f64) -> Self {
        Self {
            states: vec![],
            min_temperature: 0.1,
            best_solution_cost: f64::NEG_INFINITY,
        }
    }
}

impl StateSeries {
    pub fn cool(&self, cooling_rate: f64) {
        self.states.iter_mut().for_each(|state| {
            let new_temperatue = state.temperature * cooling_rate;
            state.temperature = f64::max(new_temperatue, self.min_temperature);
        })
    }

    pub fn replica_transition(&self, swap_probabilty: f64, closeness: f64) {
        assert!(self.states.len() >= 2);
        // TODO: Make it more efficient
        let mut numbers: Vec<usize> = [0..self.states.len()];
        numbers.shuffle(&mut thread_rng());

        let first_index = numbers[0];
        let second_index = numbers[1];
        let first_state = &self.states[first_index];
        let second_state = &self.states[second_index];

        let cost_upper_bound = closeness * self.best_solution_cost;
        let first_cost_to_much = first_state.solution_cost > cost_upper_bound;
        let second_cost_to_much = second_state.solution_cost > cost_upper_bound;

        if first_cost_to_much && second_cost_to_much {
            // Pick at random with given prob
            let mut rng = thread_rng();
            if rng.gen_range(0.0..1.0) < swap_probabilty {
                std::mem::swap(&mut first_state.temperature, &mut second_state.temperature);
            }
        }
    }
}
