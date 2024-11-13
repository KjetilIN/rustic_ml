use crate::layer::layer_trait::Layer;

#[allow(dead_code)]
pub struct Network {
    // Learning rate for the whole network
    lr: f32,

    // Represent trainable params in the neural network
    params: usize,
}

#[allow(dead_code)]
impl Network {
    pub fn init() -> Self {
        Self { lr: 0.0, params: 0 }
    }

    pub fn with_topology() -> Self {
        unimplemented!()
    }

    // Adding a layer would
    pub fn add_layer(self, _d: impl Layer) {
        unimplemented!()
    }

    /// Set learning rate of the network
    ///
    /// Uses the given learning rate to update the weights for the back propagation stage
    pub fn learning_rate(mut self, learning_rate: f32) -> Self {
        self.lr = learning_rate;
        self
    }

    pub fn trainable_params(&self) -> usize {
        self.params
    }
}
