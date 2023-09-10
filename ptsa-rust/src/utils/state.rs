use rand::{thread_rng, Rng};

use super::{
    helpers::acceptance, matrix::DistanceMatrix, solution::Solution, temp::TemperatureBounds,
};

#[derive(Debug, Clone)]
pub struct State {
    pub solution: Solution,
    pub temperature: f64,
    pub is_transion_shuffle: bool,
}

impl State {
    pub fn size(&self) -> usize {
        self.solution.size
    }

    pub fn mutate_state(&mut self, max_temp: f64, max_percent: f64) {
        let n = self.size();
        let ratio = self.temperature / max_temp;
        let trans_length: usize = (n as f64 * max_percent * ratio).ceil() as usize;
        if self.is_transion_shuffle {
            let start = thread_rng().gen_range(0..n);
            self.solution.shuffle(start, trans_length);
        } else {
            // I hate this solution but it is O(1) on average
            let first_index: usize = thread_rng().gen_range(0..n);
            let mut second_index: usize;
            loop {
                second_index = thread_rng().gen_range(0..n);
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
pub struct StatesContainer {
    pub temp_bounds: TemperatureBounds,
    pub distance_matrix: DistanceMatrix,

    pub states: Vec<State>,
    pub costs: Vec<f64>,

    pub best_cost: f64,
    pub best_solution: Option<Solution>,
}

impl StatesContainer {
    pub fn new(temp_bounds: TemperatureBounds, distance_matrix: DistanceMatrix) -> StatesContainer {
        StatesContainer {
            temp_bounds,
            distance_matrix,
            states: vec![],
            costs: vec![],

            best_cost: f64::INFINITY,
            best_solution: None,
        }
    }

    fn size(&self) -> usize {
        self.distance_matrix.size
    }

    pub fn add(&mut self, state: State) {
        assert!(state.size() == self.size());

        let cost = state.solution.cost(&self.distance_matrix);
        if cost < self.best_cost {
            self.best_cost = cost;
            self.best_solution = Some(state.solution.clone());
        }

        self.states.push(state);
        self.costs.push(cost);
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

            if acceptance(*cost, new_cost, new_state.temperature) {
                *state = new_state;
                *cost = new_cost;
            }
        }

        for (i, cost) in self.costs.iter().enumerate() {
            if *cost < self.best_cost {
                self.best_cost = *cost;
                self.best_solution = Some(self.states[i].solution.clone());
            }
        }
    }

    pub fn replica_transition(&mut self, swap_probabilty: f64, closeness: f64) {
        assert!(self.states.len() >= 2);

        // I hate this solution but it is O(1) on average
        let n = self.states.len();
        let mut first_index: usize = thread_rng().gen_range(0..n);
        let mut second_index: usize;
        loop {
            second_index = thread_rng().gen_range(0..n);
            if second_index != first_index {
                break;
            }
        }
        if second_index > first_index {
            std::mem::swap(&mut first_index, &mut second_index);
        }

        let cost_upper_bound = closeness * self.best_cost;
        let first_cost_to_much = self.costs[first_index] > cost_upper_bound;
        let second_cost_to_much = self.costs[second_index] > cost_upper_bound;

        if first_cost_to_much && second_cost_to_much {
            // Create raw pointers to swap
            let first_temp: *mut f64 = &mut self.states[first_index].temperature as *mut f64;
            let second_temp: *mut f64 = &mut self.states[second_index].temperature as *mut f64;

            let mut rng = thread_rng();
            // Pick at random with given prob
            if rng.gen_range(0.0..1.0) < swap_probabilty {
                unsafe {
                    core::ptr::swap(first_temp, second_temp);
                }
            }
        }
    }
}
