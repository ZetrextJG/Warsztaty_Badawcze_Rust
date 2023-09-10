from ptsa_rust.parameters import Parameters
from ptsa_rust.result import Result

from .ptsa_rust import *


def run_ptsa(distance_matrix: list[list[float]], time_s: int = 60, **kwargs) -> Result:
    "possibilities"
    """
    Run the PTSA algorithm on a given distance matrix
    for specified about of time (in seconds).

    Any additional keyword arguments will be passed to the 
    params object `ptsa_rust.parameters.Parameters` and used by the runner.
    """
    params = Parameters(**kwargs)
    runner = PtsaAlgorithm(params)
    return runner.run_for(distance_matrix, time_s)
