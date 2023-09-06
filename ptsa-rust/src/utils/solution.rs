use rand::{seq::SliceRandom, thread_rng};

use super::matrix::DistanceMatrix;

fn shuffle_slice(slice: &mut [usize]) {
    slice.shuffle(&mut thread_rng())
}

fn swap_slices(first: &mut [usize], second: &mut [usize]) {
    assert_eq!(first.len(), second.len());
    for i in 0..first.len() {
        std::mem::swap(&mut first[i], &mut second[i]);
    }
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
        path.push(starting_city);

        let mut current_city = starting_city;
        for _ in 0..(dmatrix.size - 1) {
            let next_city = dmatrix.matrix[current_city]
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != current_city && !path.contains(i))
                .min_by(|(_, a), (_, b)| a.total_cmp(b))
                .map(|(i, _)| i)
                .unwrap();
            path.push(next_city);
            current_city = next_city;
        }
        Solution::new(path)
    }
}

impl Solution {
    pub fn shuffle(&mut self, mut start: usize, length: usize) {
        assert!(start < self.size);
        if length > self.size {
            eprintln!(
                "Could not perform the shuffle operation with len {} decreasing to {}",
                length, self.size
            );
            shuffle_slice(&mut self.path);
            return;
        }
        if start + length > self.size - 1 {
            let overflow = start + length - self.size;
            self.path.rotate_left(overflow);
            start -= overflow;
        }
        shuffle_slice(&mut self.path[start..(start + length)])
    }

    fn find_swap_indices(
        &self,
        first_index: usize,
        second_index: usize,
        length: usize,
    ) -> Option<(usize, usize)> {
        assert!(first_index < self.size);
        assert!(second_index < self.size);

        let both_can_span_right =
            (first_index + length <= second_index) && (second_index + length <= self.size);
        if both_can_span_right {
            return Some((first_index, second_index));
        }

        let both_can_span_middle = second_index - first_index >= 2 * length - 1;
        if both_can_span_middle {
            return Some((first_index, second_index - length + 1));
        }

        let both_can_span_left =
            (first_index >= length - 1) && (second_index - first_index >= length);
        if both_can_span_left {
            return Some((first_index - length + 1, second_index - length + 1));
        }

        let both_span_opposite =
            (first_index >= length - 1) && (second_index + length <= self.size);
        if both_span_opposite {
            return Some((first_index - length + 1, second_index));
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

        match self.find_swap_indices(first_index, second_index, length) {
            // Perfect scenario
            Some((first, second)) => {
                let (head_split, tail_split) = self.path.split_at_mut(second);
                swap_slices(
                    &mut head_split[first..(first + length)],
                    &mut tail_split[0..length],
                );
            }
            // Needs shift
            None => {
                let first_can_span_right = first_index + length <= second_index;
                if first_can_span_right {
                    // -1 is not possbile due to the 3 * length < N assumption
                    self.path.rotate_left(first_index - 1);
                    self.swap_parts(0, second_index - first_index + 1, length);
                } else {
                    // First must span left
                    self.path.rotate_left(first_index - length + 1);
                    self.swap_parts(0, second_index - first_index + length - 1, length);
                }
            }
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
    fn test_swap_slice() {
        let mut data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let (first, second) = data.split_at_mut(3);
        swap_slices(first, &mut second[0..3]);
        assert_eq!(data, vec![4, 5, 6, 1, 2, 3, 7, 8, 9]);
    }

    #[test]
    fn test_solution_shuffle() {
        let path = vec![1, 2, 3, 4, 5, 6];
        let mut solution = Solution::new(path);
        solution.shuffle(2, 3);
    }

    #[test]
    fn test_solution_past_index() {
        let path = vec![1, 2, 3, 4, 5, 6];
        let mut solution = Solution::new(path);
        solution.shuffle(2, 5);
    }

    #[test]
    fn test_solution_swap() {
        let path = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut solution = Solution::new(path);
        solution.swap_parts(0, 3, 3);
        assert!(solution.path == [4, 5, 6, 1, 2, 3, 7, 8]);
    }

    #[test]
    fn test_solution_swap_opposite() {
        let path = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let mut solution = Solution::new(path);
        solution.swap_parts(5, 6, 3);
        assert!(solution.path == vec![0, 1, 2, 6, 7, 8, 3, 4, 5]);
    }

    #[test]
    fn test_solution_swap_with_shift() {
        let path = vec![0, 1, 2, 3, 4, 5, 6];
        let mut solution = Solution::new(path);
        solution.swap_parts(3, 5, 3);
        // First shift [1, 2, 3, 4, 5, 6, 0]
        // Then swap [5, 6, 0, 4, 1, 2, 3]
        assert!(solution.path == vec![5, 6, 0, 4, 1, 2, 3]);
    }

    #[test]
    fn test_solution_swap_with_shift2() {
        let path = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let mut solution = Solution::new(path);
        solution.swap_parts(6, 7, 3);
        // First shift [4, 5, 6, 7, 8, 0, 1, 2, 3]
        // Then swap [7, 8, 0, 4, 5, 6, 1, 2, 3]
        assert!(solution.path == vec![7, 8, 0, 4, 5, 6, 1, 2, 3]);
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
