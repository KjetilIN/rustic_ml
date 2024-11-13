use std::fmt::Debug;

use super::layer_trait::Layer;
use crate::data_utils::matrix::Matrix;

#[allow(dead_code)]
#[derive(Debug)]
pub struct DenseLayer {
    // Matrix of the weights connected to the
    pub weights: Matrix,

    // Bias of the matrix
    bias: Matrix,
}

impl DenseLayer {
    pub fn new(input_dim: usize, output_dim: usize) -> Self {
        // Create a bias matrix
        let mut empty_mat = Matrix::new(output_dim, 1);
        empty_mat.add_f(-1.0);
        
        // Return the dense layer
        Self {
            weights: Matrix::with_rand_bin(output_dim, input_dim),
            bias: empty_mat,
        }
    }
}

impl Layer for DenseLayer {
    fn get_params(&self) -> usize {
        self.weights.cols * self.weights.rows
    }

    fn feed_forward(&self, input: &Matrix) -> Matrix {
        match self.weights.multiply(input) {
            Ok(mat) => return mat,
            Err(err) => {
                println!(
                    "ERROR: {}: Weights: {}, Input: {}",
                    err,
                    self.weights.shape(),
                    input.shape()
                );
                panic!()
            }
        }
    }

    fn backward(&mut self, _gradient: &Matrix) -> &Matrix {
        todo!()
    }
}
