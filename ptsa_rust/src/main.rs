use std::sync::{Arc, Mutex};
use std::thread;

type Matrix = Vec<Vec<f64>>;

struct PtsaAlgorithmMock {
    distnace_matrix: Arc<Mutex<Matrix>>,
}

impl PtsaAlgorithmMock {
    pub fn new(distnace_matrix: Matrix) -> Self { 
        Self { distnace_matrix: Arc::new(Mutex::new(distnace_matrix)) } 
    }

    pub fn get_values(&self) {
        let mut handles = vec![];
        let matrix_len = self.distnace_matrix.lock().unwrap().len();
        for i in 0..matrix_len {
            let matrix_pointer = Arc::clone(&self.distnace_matrix);
            let handle = thread::spawn(move || {
                let matrix = matrix_pointer.lock().unwrap();
                println!("{:?}", matrix[i]);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

fn main() {
    let matrix = vec![
        vec![0.0, 1.0, 2.0],
        vec![1.0, 1.0, 2.0],
        vec![2.0, 1.0, 2.0],
        vec![3.0, 1.0, 2.0],
    ];
    let alg = PtsaAlgorithmMock::new(matrix);
    alg.get_values()
}
