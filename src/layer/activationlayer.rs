use crate::{activation::{self, h_step, relu, sigmoid, Activation}, data_utils::matrix::Matrix};

use super::layer_trait::Layer;

/// Activation layer in a network
#[derive(Debug)]
pub struct ActivationLayer{
    // activation function 
    activation: Activation,
    // Learning rate 
    learning_rate: f32,

    // Input matrix 
    input: Option<Matrix>
}

impl ActivationLayer {
    pub fn with(activation: Activation) -> Self{
        Self { activation, learning_rate: 0.1, input: None}
    }
    
}

impl Layer for ActivationLayer{
    fn feed_forward<'a>(&mut self, input_mat: &'a mut Matrix) -> &'a mut Matrix{
        self.input = Some(input_mat.clone());
        match self.activation{
            Activation::STEP => {
                input_mat.apply_activation(h_step);
                input_mat
            },
            Activation::SIGMOID => {
                input_mat.apply_activation(sigmoid);
                input_mat
            },
            Activation::RELU => {
                input_mat.apply_activation(relu);
                input_mat
            },
        }
    }

    fn backward(&mut self, gradient: Matrix) -> Matrix {
        // Retrieve the input matrix stored during feed_forward
        let input = self.input.as_ref().expect("Input not found for backward pass");

        // Create a new matrix to store the gradient with respect to the input
        let mut grad_input = Matrix {
            data: vec![0.0; input.data.len()],
            rows: input.rows,
            cols: input.cols,
        };

        match self.activation {
            Activation::STEP => {
                // Derivative of the step function is 0 or 1 depending on the input
                // In practice, the step function's gradient is often treated as 0 during backprop
                // But for the sake of completeness, we assume the gradient is 1 for activated inputs.
                for i in 0..input.data.len() {
                    grad_input.data[i] = if input.data[i] > 0.0 { 1.0 } else { 0.0 };
                    grad_input.data[i] *= gradient.data[i];
                }
                
                grad_input
            },
            Activation::SIGMOID => {
                // Sigmoid derivative: sigmoid(x) * (1 - sigmoid(x))
                for i in 0..input.data.len() {
                    let sigmoid_value = sigmoid(input.data[i]);
                    grad_input.data[i] = sigmoid_value * (1.0 - sigmoid_value);
                    grad_input.data[i] *= gradient.data[i];
                }
                grad_input
            },
            Activation::RELU => {
                // ReLU derivative: 1 for positive inputs, 0 for non-positive inputs
                for i in 0..input.data.len() {
                    grad_input.data[i] = if input.data[i] > 0.0 { 1.0 } else { 0.0 };
                    grad_input.data[i] *= gradient.data[i];
                }
                grad_input
            },
        }
    }

    fn get_params(&self) -> usize {
        // There are no trainable parameters for an activation layer 
        return 0
    }
}