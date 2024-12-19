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
        
        // Create weights matrix with correct dimensions
        // For matrix multiplication: (output_dim x input_dim) * (input_dim x 1) = (output_dim x 1)
        let weights = Matrix::with_rand_bin(output_dim, input_dim);
        
        // Return the dense layer
        Self {
            weights,
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
        // Check dimensions for matrix multiplication
        // weights: (output_dim x input_dim)
        // input: (input_dim x 1)
        if input_mat.cols != 1 || input_mat.rows != self.weights.cols {
            panic!(
                "Input dimensions don't match weight matrix dimensions! Expected input shape: {}x1, but got: {}. Shape weights: {}",
                self.weights.cols,
                input_mat.shape(),
                self.weights.shape()
            );
        }

        // Store the input for backward pass
        self.input = Some(input_mat.clone());

        match self.weights.multiply_with_bias(input_mat, &self.bias) {
            Ok(_) => input_mat,
            Err(err) => {
                panic!(
                    "Matrix multiplication error: {}: Weights: {}, Input: {}",
                    err,
                    self.weights.shape(),
                    input_mat.shape()
                );
            }
        }
    }

    fn backward(&mut self, gradient: Matrix) -> Matrix {
        assert!(self.input.is_some(), "Backprop error: expected the input to be set");

        let input = self.input.clone().unwrap();

        // Print shapes for debugging
        //println!("Gradient shape: {}", gradient.shape());
        //println!("Input shape: {}", input.shape());
        //println!("Weights shape: {}", self.weights.shape());

        // Compute weight gradient: input^T * gradient
        // input shape: (input_dim x 1), gradient shape: (output_dim x 1)
        // input^T shape: (1 x input_dim)
        // weight_gradient shape should be: (output_dim x input_dim)
        let input_transposed = input.get_transposed();
        //println!("Input transposed shape: {}", input_transposed.shape());

        let mut weight_gradient = Matrix::new(self.weights.rows, self.weights.cols);
        for i in 0..self.weights.rows {
            for j in 0..self.weights.cols {
                weight_gradient.data[i * self.weights.cols + j] = 
                    gradient.data[i] * input_transposed.data[j];
            }
        }

        // Compute bias gradient: just use the gradient directly
        // bias gradient should have same shape as bias (output_dim x 1)
        let mut bias_gradient = gradient.clone();

        // Propagate gradient backward: gradient * weights^T
        // gradient shape: (output_dim x 1), weights^T shape: (input_dim x output_dim)
        // result shape should be: (input_dim x 1)
        let weights_transposed = self.weights.get_transposed();
        let mut propagated_gradient = Matrix::new(self.weights.cols, 1);
        
        for i in 0..self.weights.cols {
            let mut sum = 0.0;
            for j in 0..self.weights.rows {
                sum += weights_transposed.data[i * weights_transposed.cols + j] * gradient.data[j];
            }
            propagated_gradient.data[i] = sum;
        }

        // Scale and apply gradients
        weight_gradient.scale_f(self.learning_rate);
        bias_gradient.scale_f(self.learning_rate);
        
        self.weights.apply_gradient(&weight_gradient, self.learning_rate);
        self.bias.apply_gradient(&bias_gradient, self.learning_rate);

        propagated_gradient
    }
}