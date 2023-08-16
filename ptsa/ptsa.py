import importlib
import os
import subprocess
import sys
import tempfile
import time
import zipfile
from math import floor
from pathlib import Path

from ptsa_rust import PtsaAlgorithm
from ptsa_rust.parameters import Parameters

RUST_MODULE_PATH = Path(__file__).parent.parent / "ptsa-rust"


class Runner:
    RunnerClass: type[PtsaAlgorithm]
    params: Parameters

    def __init__(self, params: Parameters) -> None:
        self.RunnerClass = PtsaAlgorithm
        self.params = params

    def _build_for_dimension(self, dimension: int) -> Path:
        """Returns the path to the new wheel"""
        my_env = os.environ.copy()
        my_env["DIM"] = str(dimension)
        subprocess.run(
            ["cargo", "build", "--release"],
            env=my_env,
            cwd=RUST_MODULE_PATH,
        )
        subprocess.run(
            ["maturin", "build", "--release"],
            env=my_env,
            cwd=RUST_MODULE_PATH,
        )
        wheels = RUST_MODULE_PATH / "target" / "wheels"
        sorted_wheels = sorted(Path(wheels).iterdir(), key=os.path.getmtime)
        return sorted_wheels[-1]

    def _update_class_from_wheel(self, wheel: Path):
        sys.modules.pop("ptsa_rust")
        with tempfile.TemporaryDirectory() as temp_dir:
            with zipfile.ZipFile(wheel, "r") as whl:
                whl.extractall(temp_dir)

            sys.path.insert(0, os.path.join(temp_dir, "ptsa_rust"))
            module = importlib.import_module("ptsa_rust")
            sys.modules["ptsa_rust"] = module

            self.RunnerClass = module.PtsaAlgorithm

    def prepare_run(self, dimension: int):
        """Forgive me for what I have done"""
        if self.RunnerClass.dimension() == dimension:
            pass

        wheel = self._build_for_dimension(dimension)
        self._update_class_from_wheel(wheel)

    def run(self, matrix: list[list[float]]):
        self.prepare_run(len(matrix))
        solver = self.RunnerClass(self.params)

        deadline = time.time()
        deadline += 5 * 60  # 5 mins
        deadline_timestamp = str(int(floor(deadline)))
        print(len(matrix))
        print(solver.dimension())

        # solution, cost = solver.run_till(matrix, deadline_timestamp)


def main():
    runner = Runner(Parameters())
    runner.run([1, 2, 3, 4, 5, 6, 7])


if __name__ == "__main__":
    main()
