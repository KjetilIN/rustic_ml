use rand::{distributions::Uniform, prelude::Distribution};

use crate::cost::CostFunction;

#[allow(dead_code)]
pub struct LinearRegression {
    /// Weights of the model
    weights: Vec<f32>,

    /// Intercept of the model
    intercept: f32,

    /// Function for calculating the performance of the model
    cost_function: fn(&Vec<f32>, &Vec<f32>) -> f32,

    /// Learning rate of the model. Used during training the model
    learning_rate: f32,

    /// When set to true, will log during training
    log_output: bool,
}

impl LinearRegression {
    pub fn new(weights_count: usize) -> Self {
        let uniform: Uniform<f32> = Uniform::new_inclusive(0.0, 1.0);
        let mut rng = rand::thread_rng();
        Self {
            weights: (0..weights_count)
                .map(|_| uniform.sample(&mut rng))
                .collect(),
            intercept: uniform.sample(&mut rng),
            cost_function: CostFunction::mean_absolute_error,
            learning_rate: 1.0,
            log_output: false,
        }
    }

    pub fn set_cost(mut self, cost_function: fn(&Vec<f32>, &Vec<f32>) -> f32) -> Self {
        self.cost_function = cost_function;
        self
    }

    pub fn set_learning_rate(mut self, learning_rate: f32) -> Self {
        self.learning_rate = learning_rate;
        self
    }

    pub fn set_logging(mut self, log_output: bool) -> Self {
        self.log_output = log_output;
        self
    }

    pub fn fit(&mut self, x_train: &Vec<f32>, y_train: &Vec<f32>, epochs: usize) {
        unimplemented!()
    }

    pub fn predict(&self, x_feature: f32) -> f32 {
        unimplemented!()
    }

    pub fn cost(&self, x_train: &Vec<f32>, y_train: &Vec<f32>) -> f32 {
        (self.cost_function)(x_train, y_train)
    }
}
