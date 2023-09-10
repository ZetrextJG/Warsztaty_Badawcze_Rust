use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};

use super::matrix::DistanceMatrix;

#[inline]
fn gen_index(rng: &mut ThreadRng, lbound: usize, ubound: usize) -> usize {
    if ubound <= (core::u32::MAX as usize) {
        rng.gen_range(lbound as u32..ubound as u32) as usize
    } else {
        rng.gen_range(lbound..ubound)
    }
}

fn shuffle_slice(slice: &mut [usize]) {
    slice.shuffle(&mut thread_rng())
}

#[derive(Debug, Clone)]
pub struct Solution {
    pub path: Vec<usize>,
    pub size: usize,
}

impl Solution {
    pub fn new(path: Vec<usize>) -> Self {
        let size = path.len();
        assert!(size > 0);
        Solution { path, size }
    }

    pub fn random_solution(size: usize) -> Self {
        let mut path: Vec<usize> = (0..size).collect();
        path.shuffle(&mut thread_rng());
        Solution::new(path)
    }

    pub fn nearest_neightbor_solution(dmatrix: &DistanceMatrix, starting_city: usize) -> Self {
        if starting_city >= dmatrix.size {
            panic!("Impossible choice for the first city")
        }

        let mut path: Vec<usize> = Vec::with_capacity(dmatrix.size);
        let mut visited: Vec<bool> = vec![false; dmatrix.size];

        path.push(starting_city);
        visited[starting_city] = true;

        let mut current_city = starting_city;
        for _ in 0..(dmatrix.size - 1) {
            let next_city = dmatrix.matrix[current_city]
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != current_city && !(visited[*i]))
                .min_by(|(_, a), (_, b)| a.total_cmp(b))
                .map(|(i, _)| i)
                .unwrap();
            path.push(next_city);
            visited[next_city] = true;
            current_city = next_city;
        }
        Solution::new(path)
    }
}

impl Solution {
    pub fn shuffle(&mut self, start: usize, length: usize) {
        assert!(start < self.size);
        if length > self.size {
            eprintln!(
                "Could not perform the shuffle operation with len {} decreasing to {}",
                length, self.size
            );
            shuffle_slice(&mut self.path);
            return;
        }

        let rng = &mut thread_rng();
        let max_length = self.path.len();
        for i in (start..(start + length)).rev() {
            let new_index = gen_index(rng, start, i + 1);
            self.path.swap(i % max_length, new_index % max_length);
        }
    }

    fn find_swap_indices(
        &self,
        first_index: usize,
        second_index: usize,
        length: usize,
    ) -> Option<(usize, usize)> {
        assert!(first_index < self.size);
        assert!(second_index < self.size);

        let first_fit_between_second = first_index + length <= second_index;
        if first_fit_between_second {
            return Some((first_index, second_index));
        }

        let first_underflow = first_index < length - 1;
        if !first_underflow {
            return Some((first_index - length + 1, second_index));
        }

        let underflow_first: usize = first_index + self.path.len() + 1 - length;

        let first_move_overlap = second_index + length > underflow_first;
        if !first_move_overlap {
            return Some((underflow_first, second_index));
        }

        None
    }

    pub fn swap_parts(&mut self, mut first_index: usize, mut second_index: usize, length: usize) {
        assert!(first_index < self.size);
        assert!(second_index < self.size);

        // Current implementation assumes that 3 * length - 2 <= N
        // This assumption allows for the swap to always be deterministic and possible
        if 3 * length - 2 > self.size {
            panic!("Not possible swap")
        }

        if first_index > second_index {
            std::mem::swap(&mut first_index, &mut second_index);
        }

        let max_len = self.path.len();
        match self.find_swap_indices(first_index, second_index, length) {
            Some((first, second)) => {
                for offset in 0..length {
                    let first_replace = (first + offset) % max_len;
                    let second_replace = (second + offset) % max_len;
                    self.path.swap(first_replace, second_replace);
                }
            }
            None => panic!("Impossible indicies"),
        }
    }

    pub fn cost(&self, dmatrix: &DistanceMatrix) -> f64 {
        assert_eq!(dmatrix.size, self.size);
        // Calculate the length of that cycle using the distance matrix
        let mut length = 0.0;
        for i in 0..(self.size - 1) {
            let j = (i + 1) % self.size;
            length += dmatrix.matrix[self.path[i]][self.path[j]];
        }
        length + dmatrix.matrix[self.path[self.size - 1]][self.path[0]]
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::helpers::CountUnique;

    use super::*;

    #[test]
    fn test_shuffle_slice() {
        let mut data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        shuffle_slice(&mut data[0..]);
        assert!(data.len() == 9);
    }

    #[test]
    fn test_solution_shuffle() {
        let path = vec![0, 1, 2, 3, 4, 5];
        let mut solution = Solution::new(path);
        solution.shuffle(2, 3);
        assert_eq!(solution.path[0], 0);
        assert_eq!(solution.path[1], 1);
        assert_eq!(solution.path[5], 5);
    }

    #[test]
    fn test_solution_past_index() {
        let path = vec![0, 1, 2, 3, 4, 5];
        let mut solution = Solution::new(path);
        solution.shuffle(5, 3);
        assert_eq!(solution.path[1], 1);
        assert_eq!(solution.path[2], 2);
        assert_eq!(solution.path[3], 3);
    }

    #[test]
    fn test_solution_swap() {
        let path = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut solution = Solution::new(path);
        solution.swap_parts(0, 3, 3);
        assert!(solution.path == [4, 5, 6, 1, 2, 3, 7, 8]);
    }

    #[test]
    fn test_solution_swap_opposities() {
        let path = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let mut solution = Solution::new(path);
        solution.swap_parts(5, 6, 3);
        assert!(solution.path == vec![0, 1, 2, 6, 7, 8, 3, 4, 5]);
    }

    #[test]
    fn test_solution_swap_with_overflow() {
        let path = vec![0, 1, 2, 3, 4, 5, 6];
        let mut solution = Solution::new(path);
        solution.swap_parts(2, 5, 3);
        // Slice [2, 3, 4]
        // Then swap with [5, 6, 0]
        assert!(solution.path == vec![4, 1, 5, 6, 0, 2, 3]);
    }

    #[test]
    fn test_solution_swap_with_underflow() {
        let path = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let mut solution = Solution::new(path);
        solution.swap_parts(1, 2, 3);
        // Slice [2, 3, 4]
        // Then swap with [8, 0, 1]
        assert!(solution.path == vec![3, 4, 8, 0, 1, 5, 6, 7, 2]);
    }

    #[test]
    fn test_cost() {
        let matrix = vec![vec![0.0, 1.0], vec![1.0, 0.0]];
        let dmatrix = DistanceMatrix::new(matrix);
        let path = vec![0, 1];
        let solution = Solution::new(path);
        assert_eq!(solution.cost(&dmatrix), 2.0);
    }

    #[test]
    fn test_random_solution() {
        let random_sol: Solution = Solution::random_solution(10);
        assert_eq!(random_sol.path.len(), 10);
        assert_eq!(random_sol.path.iter().unique(), 10);
    }

    #[test]
    fn test_heuristic_approach() {
        let dmatrix = DistanceMatrix::new(vec![vec![0.0, 1.0], vec![2.0, 0.0]]);
        let solution: Solution = Solution::nearest_neightbor_solution(&dmatrix, 0);
        assert_eq!(solution.path.len(), 2);
        assert_eq!(solution.path.iter().unique(), 2);
        assert_eq!(*(solution.path.first().unwrap()), 0);
    }
}
