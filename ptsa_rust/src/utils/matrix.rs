pub struct DistanceMatrix<const N: usize> {
    pub matrix: [[f32; N]; N],
}

impl<const N: usize> DistanceMatrix<N> {}
