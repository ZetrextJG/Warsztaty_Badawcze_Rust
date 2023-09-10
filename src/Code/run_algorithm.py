import os
from pathlib import Path

import pandas as pd
from ptsa_rust import run_ptsa

from src.Code.calculate_distance import cycle_length
from src.Code.pt_sa import pt_sa
from src.Parameters.best_known_solution import best_known_solution
from src.Parameters.problems import problems

RESULTS_DIR = Path(__file__).parent.parent / "Tests" / "Results"


def set_parameters(exec_time: float) -> dict:
    parameters = {
        "n": 20,
        "min_temperature": 0.1,
        "max_temperature": 50,
        "probability_of_shuffle": 0.1,
        "probability_of_heuristic": 0.7,
        "a": 1,
        "b": 1,
        "duration_of_execution_in_seconds": exec_time - 0.1,
        "k": 20,
        "max_length_percent_of_cycle": 0.3,  # max 0.3 more will result in bugs
        "swap_states_probability": 0.1,
        "closeness": 1.5,
        "cooling_rate": 0.95,
    }
    return parameters


def run_for_one_problem(problem_name: str):
    distance_matrix = problems[problem_name]
    N = len(distance_matrix)
    exec_time = 60 * 5
    if N < 300:
        num_of_runs = int(exec_time / 30)
        parameters = set_parameters(30)
    else:
        num_of_runs = int(exec_time / 150)
        parameters = set_parameters(150)
    best_solution = [None for _ in range(len(distance_matrix))]
    best_solution_length = float("inf")
    for _ in range(num_of_runs):
        solution, solution_length = pt_sa(distance_matrix, **parameters)
        if solution_length < best_solution_length:
            best_solution, best_solution_length = solution, solution_length
    print(
        f"Ultimate best solution: {best_solution}\nUltimate best solution length: {best_solution_length}"
    )
    return best_solution, best_solution_length


def iterate_over_all_problems():
    df = pd.DataFrame(columns=["Name", "best_known_sol", "our_solution", "deficit_ratio"])
    for name, length in best_known_solution.items():
        df = pd.concat(
            [df, pd.DataFrame.from_records([{"Name": name, "best_known_sol": length}])],
            ignore_index=True,
        )
    for name in problems.keys():
        print(f"Problem: {name}")
        solution, solution_length = run_for_one_problem(name)
        optimal_solution_length = best_known_solution[name]
        print(f"Problem: {name}")
        print(
            f"Our solution length: {solution_length}, optimal solution length: {optimal_solution_length}"
        )
        deficit_ratio = solution_length / optimal_solution_length * 100 - 100
        print(f"Our solution is worse by {deficit_ratio:.2f}%")
        df.loc[(df["Name"] == name), "our_solution"] = solution_length
        df.loc[(df["Name"] == name), "deficit_ratio"] = deficit_ratio
        break

    output_file = RESULTS_DIR / "My_results.csv"
    df.to_csv(output_file)


def iterate_over_all_problems_with_time(exec_time: float):
    df = pd.DataFrame(columns=["Name", "best_known_sol", "our_solution", "deficit_ratio"])
    for name, length in best_known_solution.items():
        df = pd.concat(
            [df, pd.DataFrame.from_records([{"Name": name, "best_known_sol": length}])],
            ignore_index=True,
        )
    parameters = set_parameters(exec_time)
    for name in problems.keys():
        distance_matrix = problems[name]
        print(f"Problem: {name}")
        solution, solution_length = pt_sa(distance_matrix, **parameters)
        optimal_solution_length = best_known_solution[name]
        print(f"Problem: {name}")
        print(
            f"Our solution length: {solution_length}, optimal solution length: {optimal_solution_length}"
        )
        deficit_ratio = solution_length / optimal_solution_length * 100 - 100
        print(f"Our solution is worse by {deficit_ratio}%")
        df.loc[(df["Name"] == name), "our_solution"] = solution_length
        df.loc[(df["Name"] == name), "deficit_ratio"] = deficit_ratio

    output_file = RESULTS_DIR / "long_term_results.csv"
    df.to_csv(output_file)


### RUST LAND


def run_rust_for_one(problem_name: str) -> tuple[list[float], float]:
    distance_matrix: list[list[float]] = problems[problem_name]  # type: ignore
    exec_time = 60 * 1  # 1 min

    threads: int = os.cpu_count()  # type: ignore
    result = run_ptsa(
        distance_matrix,
        time_s=exec_time,
        number_of_threads=threads - 1,
    )
    solution, solution_length = result
    print(f"Our best solution: {solution}\nOur best solution length: {solution_length}")
    print(
        f"Rust length: {solution_length}, python validation: {cycle_length(solution, distance_matrix)}"
    )
    return solution, solution_length  # type: ignore


def iterate_over_all_problems_rust():
    df = pd.DataFrame(columns=["Name", "best_known_sol", "our_solution", "deficit_ratio"])
    for name, length in best_known_solution.items():
        df = pd.concat(
            [df, pd.DataFrame.from_records([{"Name": name, "best_known_sol": length}])],
            ignore_index=True,
        )
    for name in problems.keys():
        print(f"Problem: {name}")
        solution, solution_length = run_rust_for_one(name)
        optimal_solution_length = best_known_solution[name]
        print(f"Problem: {name}")
        print(
            f"Our solution length: {solution_length}, optimal solution length: {optimal_solution_length}"
        )
        deficit_ratio = solution_length / optimal_solution_length * 100 - 100
        print(f"Our solution is worse by {deficit_ratio:.2f}%")
        print(solution)
        df.loc[(df["Name"] == name), "our_solution"] = solution_length
        df.loc[(df["Name"] == name), "deficit_ratio"] = deficit_ratio

    output_file = RESULTS_DIR / "Rust_results.csv"
    df.to_csv(output_file)
