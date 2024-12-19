use crate::{data_utils::matrix::Matrix, layer::layer_trait::Layer}; // Assuming your Layer trait is properly defined elsewhere

#[allow(dead_code)]
pub struct Network {
    // Learning rate for the whole network
    lr: f32,

    // Represent the layers of the network (the layers themselves)
    layers: Vec<Box<dyn Layer>>,

    // Total number of trainable parameters in the network
    params: usize,
}

impl Network {
    // Initialize the network with no layers and learning rate set to zero.
    pub fn init() -> Self {
        Self {
            lr: 0.0,
            layers: Vec::new(),
            params: 0,
        }
    }

    // Create a new network with a specified topology (layers)
    pub fn with_topology(layers: Vec<Box<dyn Layer>>, learning_rate: f32) -> Self {
        let params = layers.iter().map(|layer| layer.get_params()).sum::<usize>();
        Self {
            lr: learning_rate,
            layers,
            params,
        }
    }

    // Add a new layer to the network
    pub fn add_layer(mut self, layer: Box<dyn Layer>) -> Self {
        let new_params = layer.get_params();
        self.params += new_params;
        self.layers.push(layer);
        self
    }

    // Set learning rate of the network
    pub fn learning_rate(mut self, learning_rate: f32) -> Self {
        self.lr = learning_rate;
        self
    }

    // Get total trainable parameters of the network
    pub fn trainable_params(&self) -> usize {
        self.params
    }

    // Perform the forward pass for the entire network
    pub fn forward(&mut self, input: &mut Matrix) -> Matrix {
        let mut output = input.clone();
        for layer in self.layers.iter_mut() {
            layer.feed_forward(&mut output); 
        }
        output
    }

    // Perform the backward pass for the entire network
    pub fn backward(&mut self, gradient: Matrix) -> Matrix {
        let mut grad = gradient;
        for layer in self.layers.iter_mut().rev() {
            grad = layer.backward(grad); // Update gradient by each layer's backward pass
        }
        grad
    }
}
