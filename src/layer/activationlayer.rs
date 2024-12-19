use crate::{activation::{self, h_step, relu, sigmoid, Activation}, data_utils::matrix::Matrix};

use super::layer_trait::Layer;

/// Activation layer in a network
#[derive(Debug)]
pub struct ActivationLayer{
    activation: Activation
}

impl ActivationLayer {
    pub fn with(activation: Activation) -> Self{
        Self { activation }
    }
    
}

impl Layer for ActivationLayer{
    fn feed_forward<'a>(&self, input_mat: &'a mut Matrix) -> &'a mut Matrix{
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

    fn backward(&mut self, gradient: &Matrix) -> &Matrix {
        todo!()
    }

    fn get_params(&self) -> usize {
        // There are no trainable parameters for an activation layer 
        return 0
    }
}