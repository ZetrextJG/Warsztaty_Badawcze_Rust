#[derive(Debug, Clone)]
pub struct DistanceMatrix {
    pub matrix: Vec<Vec<f64>>,
    pub size: usize,
}

impl DistanceMatrix {
    pub fn new(matrix: Vec<Vec<f64>>) -> Self {
        let size = matrix.len();
        for row in matrix.iter() {
            assert_eq!(row.len(), size)
        }

        DistanceMatrix { matrix, size }
    }
}

#[test]
fn test_from_matrix_vec() {
    let vector: Vec<Vec<f64>> = vec![vec![0.0, 2.0], vec![3.0, 0.0]];
    let dmatrix: DistanceMatrix = DistanceMatrix::new(vector);
    let expected = vec![vec![0.0, 2.0], vec![3.0, 0.0]];
    assert_eq!(dmatrix.matrix, expected);
}
