#[derive(Debug)]
pub struct DistanceMatrix<const N: usize> {
    pub matrix: [[f64; N]; N],
}

impl<const N: usize> DistanceMatrix<N> {}
