import time

from Code.run_algorithm import (iterate_over_all_problems, iterate_over_all_problems_with_time,
                                run_for_one_problem, run_rust_for_one)


def main():
    # uncomment one of the following lines to run the algorithm

    # plain iteration over all problems 5 minutes each, recreation of the version from The Race
    # iterate_over_all_problems()

    # solve one problem given by its name
    run_rust_for_one("rbg403")
    # run_for_one_problem("rbg403")

    # iterate over all problems with time limit in seconds
    # iterate_over_all_problems_with_time(30 * 60)


if __name__ == "__main__":
    start_time = time.time()
    main()
    print("--- %s seconds ---" % (time.time() - start_time))
