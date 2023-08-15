#[derive(Debug)]
pub struct DistanceMatrix<const N: usize> {
    pub matrix: [[f64; N]; N],
}

impl<const N: usize> DistanceMatrix<N> {}

impl<const N: usize> From<Vec<Vec<f64>>> for DistanceMatrix<N> {
    fn from(vector_matrix: Vec<Vec<f64>>) -> Self {
        let mut matrix = [[0.0; N]; N];
        if vector_matrix.len() != N {
            panic!("Provided matrix does not have the required {} rows", N);
        }
        for (i, values) in vector_matrix.iter().enumerate() {
            if values.len() != N {
                panic!(
                    "Provided matrix in row {} has number of columns not equal to {}",
                    i, N
                );
            }
            for (j, value) in values.iter().enumerate() {
                matrix[i][j] = *value;
            }
        }
        DistanceMatrix { matrix }
    }
}

#[test]
fn test_from_matrix_vec() {
    let vector: Vec<Vec<f64>> = vec![vec![0.0, 2.0], vec![3.0, 0.0]];
    let dmatrix: DistanceMatrix<2> = vector.into();
    let expected = [[0.0, 2.0], [3.0, 0.0]];
    assert_eq!(dmatrix.matrix, expected);
}
