from settings import Parameters, Result

class Params:
    number_of_states: int
    number_of_repeats: int
    number_of_concurrent_threads: int
    min_temperature: float
    max_temperature: float
    probability_of_shuffle: float
    probability_of_heuristic: float
    temp_beta_a: float
    temp_beta_b: float
    max_length_percent_of_cycle: float
    swap_states_probability: float
    closeness: float
    cooling_rate: float

    def __init__(
        self,
        number_of_states: int,
        number_of_repeats: int,
        number_of_concurrent_threads: int,
        min_temperature: float,
        max_temperature: float,
        probability_of_shuffle: float,
        probability_of_heuristic: float,
        temp_beta_a: float,
        temp_beta_b: float,
        max_length_percent_of_cycle: float,
        swap_states_probability: float,
        closeness: float,
        cooling_rate: float,
    ) -> None: ...

class PtsaAlgorithm:
    params: Params

    def __init__(self, parms: Params) -> None: ...
    def run_till(self, matrix: list[list[float]], deadline_timestamp: str) -> Result:
        """
        Run the PTSA algorith on a given distance matrix till the specified deadline
        Deadtime must be a string containing the number of seconds from the start of unix time.
        Returns the best solution
        """
        ...
    @staticmethod
    def dimension() -> int: ...
