use rand::{distributions::Uniform, prelude::Distribution};

use crate::cost::CostFunction;

use super::linear_regression_trait::LinearRegressionTrait;


#[allow(dead_code)]
pub struct SimpleLinearRegression{
    /// Coefficient of the model
    coefficient: f32,

    /// Intercept of the model 
    intercept: f32,

    /// Function for calculating the performance of the model 
    cost_function: fn(&Vec<f32>, &Vec<f32>) -> f32,

    /// Learning rate of the model. Used during training the model 
    learning_rate: f32
}

impl LinearRegressionTrait for SimpleLinearRegression {
    fn new() -> Self {
        let uniform: Uniform<f32> = Uniform::new_inclusive(0.0, 1.0);
        let mut rng = rand::thread_rng();
        Self {
            coefficient: uniform.sample(&mut rng),
            intercept: uniform.sample(&mut rng),
            cost_function: CostFunction::mean_square_error,
            learning_rate: 1.0
        }
    }

    fn set_cost(mut self, cost_function: fn(&Vec<f32>, &Vec<f32>) -> f32) -> Self {
        self.cost_function = cost_function;
        self
    }

    fn set_learning_rate(mut self, learning_rate:f32) -> Self{
        self.learning_rate = learning_rate;
        self
    }
    
    fn fit(&mut self, x_train: &Vec<(f32, f32)>, y_train: &Vec<f32>) {
        todo!()
    }

    fn score(&self) -> f32 {
        todo!()
    }
}