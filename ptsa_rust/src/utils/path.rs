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

pub struct Solution<const N: usize> {
    path: [usize; N],
}

impl<const N: usize> Solution<N> {
    pub fn shuffle(&mut self, mut start: usize, length: usize) {
        if length > N {
            eprintln!(
                "Could not perform the shuffle operation with len {} decreasing to {}",
                length, N
            );
            shuffle_slice(&mut self.path);
            return;
        }
        if start + length > N - 1 {
            let overflow = N - 1 - start - length;
            self.path.rotate_left(overflow);
            start -= overflow;
        }
        shuffle_slice(&mut self.path[start..(start + length)])
    }

    pub fn swap_parts(&mut self, mut first_index: usize, mut second_index: usize, length: usize) {
        if first_index > second_index {
            std::mem::swap(&mut first_index, &mut second_index);
        }

        let first_can_go_right = second_index - first_index >= length;
        let second_can_go_rigth = N - second_index + first_index >= length;

        // WARN: This seems wrong
        if !first_can_go_right && second_can_go_rigth {
            first_index -= length - 1
        } else if first_can_go_right && !second_can_go_rigth {
            second_index -= length - 1
        }

        let (head_split, tail_split) = self.path.split_at_mut(second_index);
        swap_slices(
            &mut head_split[first_index..(first_index + length)],
            &mut tail_split[0..length],
        );
    }

    pub fn cost(&self, dmatrix: &DistanceMatrix<N>) -> f32 {
        // Calculate the length of that cycle using the distance matrix
        let mut length = 0.0;
        for i in 0..(N - 1) {
            let j = (i + 1) % N;
            length += dmatrix.matrix[self.path[i]][self.path[j]];
        }
        length + dmatrix.matrix[self.path[N - 1]][self.path[0]]
    }
}

#[cfg(test)]
mod tests {
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
        let data = vec![1, 2, 3, 4, 5, 6];
        let path: [usize; 6] = data.try_into().unwrap();
        let mut solution = Solution { path };
        solution.shuffle(2, 3);
    }

    #[test]
    fn test_solution_swap() {
        let path = [1, 2, 3, 4, 5, 6];
        let mut solution = Solution { path };
        solution.swap_parts(0, 3, 3);
        assert!(solution.path == [4, 5, 6, 1, 2, 3]);
    }

    #[test]
    fn test_solution_swap_with_shift() {
        let path = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut solution = Solution { path };
        solution.swap_parts(6, 7, 3);
        assert!(solution.path == [1, 2, 3, 7, 8, 9, 4, 5, 6]);
    }

    #[test]
    fn test_cost() {
        let matrix = [[0.0, 1.0], [1.0, 0.0]];
        let dmatrix = DistanceMatrix { matrix };
        let path = [0, 1];
        let solution = Solution { path };
        assert_eq!(solution.cost(&dmatrix), 2.0);
    }
}
