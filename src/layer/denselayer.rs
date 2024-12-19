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

    // Learning rate 
    learning_rate: f32,

    // Input matrix 
    input: Option<Matrix>
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
            input: None,
            learning_rate: 0.1,
        }
    }
}

impl Layer for DenseLayer {
    fn get_params(&self) -> usize {
        self.weights.cols * self.weights.rows
    }

    fn feed_forward<'a>(&mut self, input_mat: &'a mut Matrix) -> &'a mut Matrix {
        self.input = Some(input_mat.clone()); // Store the input for backward pass
        match self.weights.multiply_into(input_mat) {
            Ok(_) => return input_mat,
            Err(err) => {
                println!(
                    "ERROR: {}: Weights: {}, Input: {}",
                    err,
                    self.weights.shape(),
                    input_mat.shape()
                );
                panic!()
            }
        }
    }

    fn backward(&mut self, gradient: Matrix) -> Matrix {
        assert!(self.input.is_some(),"Backprop error: expected the input to be set");

        // Compute weight gradient: input^T * gradient
        let mut weight_gradient = self.input.clone().unwrap().get_transposed().multiply(&gradient).expect("Backprop error: could not multiply gradient");

        // Compute bias gradient: sum of gradient along rows
        let mut bias_gradient = gradient.sum_rows(); 

        // Propagate gradient backward: gradient * weights^T
        let propagated_gradient = gradient.multiply(&self.weights.get_transposed()).unwrap();

        // Update weights and biases (optional, if you're not separating this logic)
        &weight_gradient.scale_f(self.learning_rate); 
        &bias_gradient.scale_f(self.learning_rate);

        
        self.weights.apply_gradient(&weight_gradient, self.learning_rate);
        self.bias.apply_gradient(&bias_gradient, self.learning_rate);

        // Return the propagated gradient
        propagated_gradient
    }


    
}
