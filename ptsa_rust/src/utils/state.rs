use rand::{thread_rng, Rng};

use super::{helpers::acceptance, matrix::DistanceMatrix, path::Solution, temp::TemperatureBounds};

#[derive(Debug, Clone)]
pub struct State<const N: usize> {
    pub solution: Solution<N>,
    pub temperature: f64,
    pub is_transion_shuffle: bool,
}

impl<const N: usize> State<N> {
    pub fn mutate_state(&mut self, max_temp: f64, max_percent: f64) {
        let ratio = self.temperature / max_temp;
        let trans_length: usize = (N as f64 * max_percent * ratio).ceil() as usize;
        if self.is_transion_shuffle {
            let start = thread_rng().gen_range(0..=N);
            self.solution.shuffle(start, trans_length);
        } else {
            // I hate this solution but it is O(1) on average
            let first_index: usize = thread_rng().gen_range(0..=N);
            let mut second_index: usize;
            loop {
                second_index = thread_rng().gen_range(0..=10);
                if second_index != first_index {
                    break;
                }
            }

            self.solution
                .swap_parts(first_index, second_index, trans_length);
        }
    }
}

#[derive(Debug)]
pub struct StatesContainer<const N: usize> {
    pub temp_bounds: TemperatureBounds,
    pub distance_matrix: DistanceMatrix<N>,

    pub states: Vec<State<N>>,
    pub costs: Vec<f64>,
    pub best_solution_cost: f64,
}

impl<const N: usize> StatesContainer<N> {
    pub fn new(
        temp_bounds: TemperatureBounds,
        distance_matrix: DistanceMatrix<N>,
    ) -> StatesContainer<N> {
        StatesContainer {
            temp_bounds,
            distance_matrix,
            states: vec![],
            costs: vec![],
            best_solution_cost: f64::INFINITY,
        }
    }

    pub fn add(&mut self, state: State<N>) {
        let cost = state.solution.cost(&self.distance_matrix);

        self.states.push(state);
        self.costs.push(cost);
        if cost < self.best_solution_cost {
            self.best_solution_cost = cost;
        }
    }

    pub fn cool(&mut self, cooling_rate: f64) {
        self.states.iter_mut().for_each(|state| {
            let new_temperatue = state.temperature * cooling_rate;
            state.temperature = f64::max(new_temperatue, self.temp_bounds.min);
        })
    }

    pub fn metropolis_tranision(&mut self, max_percent_of_cycle: f64) {
        for (state, cost) in self.states.iter_mut().zip(self.costs.iter_mut()) {
            let mut new_state = state.clone();
            new_state.mutate_state(self.temp_bounds.max, max_percent_of_cycle);
            let new_cost = new_state.solution.cost(&self.distance_matrix);

            if acceptance(new_cost, *cost, new_state.temperature) {
                *state = new_state;
                *cost = new_cost;
            }
        }

        for cost in self.costs.iter() {
            if *cost < self.best_solution_cost {
                self.best_solution_cost = *cost;
            }
        }
    }

    pub fn replica_transition(&mut self, swap_probabilty: f64, closeness: f64) {
        assert!(self.states.len() >= 2);

        // I hate this solution but it is O(1) on average
        let mut first_index: usize = thread_rng().gen_range(0..=N);
        let mut second_index: usize;
        loop {
            second_index = thread_rng().gen_range(0..=10);
            if second_index != first_index {
                break;
            }
        }
        if second_index > first_index {
            std::mem::swap(&mut first_index, &mut second_index);
        }

        let cost_upper_bound = closeness * self.best_solution_cost;
        let first_cost_to_much = self.costs[first_index] > cost_upper_bound;
        let second_cost_to_much = self.costs[second_index] > cost_upper_bound;

        if first_cost_to_much && second_cost_to_much {
            // This have to be done to satisfy rust compiler
            let (first_part, second_part) = self.states.split_at_mut(second_index);
            let first_state = &mut first_part[first_index];
            let second_state = &mut second_part[0];

            // Pick at random with given prob
            let mut rng = thread_rng();
            if rng.gen_range(0.0..1.0) < swap_probabilty {
                std::mem::swap(&mut first_state.temperature, &mut second_state.temperature);
            }
        }
    }
}
