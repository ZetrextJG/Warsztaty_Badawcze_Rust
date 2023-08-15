from dataclasses import dataclass


@dataclass
class Parameters:
    number_of_states: int = 20
    number_of_repeats: int = 20
    number_of_concurrent_threads: int = 1
    min_temperature: float = 0.1
    max_temperature: float = 50
    probability_of_shuffle: float = 0.1
    probability_of_heuristic: float = 0.7
    temp_beta_a: float = 1
    temp_beta_b: float = 1
    max_length_percent_of_cycle: float = 0.3
    swap_states_probability: float = 0.1
    closeness: float = 1.5
    cooling_rate: float = 0.95
