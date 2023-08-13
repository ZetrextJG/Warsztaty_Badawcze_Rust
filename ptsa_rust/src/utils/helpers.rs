fn initialize_transition_function_types(n: &usize, probability_of_shuffle: &f64) -> Vec<bool> {
    (0..*n)
        .map(|_| rand::random::<f64>() < *probability_of_shuffle)
        .collect()
}

// metropolis_transition.py
pub fn acceptance(solution_length: f64, new_solution_length: f64, temperature: f64) -> bool {
    let acceptance_probability = f64::exp(-(new_solution_length - solution_length) / temperature);
    rand::random::<f64>() < f64::min(1.0, acceptance_probability)
}
