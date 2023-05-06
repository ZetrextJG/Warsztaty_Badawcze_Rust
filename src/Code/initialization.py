from numpy.random import beta
from random import random


def initialize_temperatures(
    n: int, min: float, max: float, a: float = 1, b: float = 1
) -> list:
    """
    Returns a list of n temperatures between min and max,
    with a and b as parameters for the beta distribution.
    if a = b = 1, the temperatures are uniformly distributed between min and max.
    """
    return beta(a, b, n) * (max - min) + min


def initialize_transition_function_types(n: int, probability_of_shuffle: float) -> list:
    """
    Returns a bool list of length n, where
    q[i] = True means that the transition
    function at index i is a shuffle transition function.
    """
    return [random.random() < probability_of_shuffle for _ in range(n)]


def initialize_initial_solutions(
    n: int, distance_matrix: list[list[float]], probability_of_heuristic: float
) -> list:
    """
    Returns a list of n solutions, where
    q[i] has probability_of_heuristic chance of being a heuristic initial solution.
    heuristic used here is nearest neighbor.
    """
    # calculate nearest neighbor solution only once
    nearest_neighbor_solution = nearest_neighbor_initial_solution(distance_matrix)

    # create initial solution list and fill it with
    # either nearest neighbor solution or random solution
    initial_solution = [None for _ in range(n)]
    for i in range(n):
        if random.random() < probability_of_heuristic:
            initial_solution[i] = nearest_neighbor_solution
        else:
            initial_solution[i] = random_initial_solution(distance_matrix)
    return initial_solution


def nearest_neighbor_initial_solution(distance_matrix: list[list[float]]) -> list:
    """
    Finds a suboptimal solution to the asymmetric Traveling Salesman Problem
    It is irrelevant what values are on the diagonal of the matrix

    :return: list:
            A list of integers representing the order in which
            cities should be visited to obtain a suboptimal
            solution to the TSP. The first city in the path is always city 0.
    """
    size = distance_matrix.shape[0]
    unvisited = set(range(1, size))
    path = [0]
    current_city = 0
    while unvisited:
        nearest_neighbor = min(
            unvisited, key=lambda city: distance_matrix[current_city][city]
        )
        unvisited.remove(nearest_neighbor)
        path.append(nearest_neighbor)
        current_city = nearest_neighbor
    return path


def random_initial_solution(distance_matrix: list[list[float]]) -> list:
    """
    Finds a completely random solution to the asymmetric Traveling Salesman Problem

    :return: list:
        A list of integers representing the order in which cities should be visited
    """
    size = distance_matrix.shape[0]
    solution = list(range(size))
    random.shuffle(solution)
    return solution


def initialization(
    distance_matrix: list[list[float]],
    n: int,
    min_temperature: float,
    max_temperature: float,
    probability_of_shuffle: float,
    probability_of_heuristic: float,
    a: float,
    b: float,
) -> tuple:
    """
    Returns a tuple of lists, where
    the first list is a list of n temperatures between min_temperature and max_temperature,
    with a and b as parameters for the beta distribution.
    the second list is a bool list of length n, where
    q[i] = True means that the transition
    function at index i is a shuffle transition function,
    the third list is a list of n initial solutions, where
    q[i] has probability_of_heuristic chance of being a heuristic initial solution.
    """
    temperatures = initialize_temperatures(n, min_temperature, max_temperature, a, b)
    transition_function_types = initialize_transition_function_types(
        n, probability_of_shuffle
    )
    initial_solutions = initialize_initial_solutions(
        n, distance_matrix, probability_of_heuristic
    )
    return temperatures, transition_function_types, initial_solutions
