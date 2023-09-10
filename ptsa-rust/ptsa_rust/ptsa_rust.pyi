from parameters import Parameters
from result import Result

class PtsaAlgorithm:
    # HACK: This is not true, but will work just fine
    params: Parameters

    def __init__(self, parms: Parameters) -> None: ...
    def run_for(self, matrix: list[list[float]], time: int) -> Result:
        """
        Run the PTSA algorithm on a given distance matrix
        for specified about of time (in seconds)
        """
        ...
