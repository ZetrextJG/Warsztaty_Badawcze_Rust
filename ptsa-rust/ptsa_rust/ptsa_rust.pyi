from parameters import Parameters
from result import Result

class PtsaAlgorithm:
    # HACK: This is not true, but will work just fine
    params: Parameters

    def __init__(self, parms: Parameters) -> None: ...
    def run_till(self, matrix: list[list[float]], deadline_timestamp: str) -> Result:
        """
        Run the PTSA algorith on a given distance matrix till the specified deadline
        Deadtime must be a string containing the number of seconds from the start of unix time.
        Returns the best solution
        """
        ...
    @staticmethod
    def dimension() -> int: ...
