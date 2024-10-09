use rand::{distributions::Uniform, prelude::Distribution};

use crate::cost::CostFunction;

use super::linear_regression_trait::LinearRegressionTrait;

#[allow(dead_code)]
pub struct SimpleLinearRegression {
    /// Coefficient of the model
    coefficient: f32,

    /// Intercept of the model
    intercept: f32,

    /// Function for calculating the performance of the model
    cost_function: fn(&Vec<f32>, &Vec<f32>) -> f32,

    /// Learning rate of the model. Used during training the model
    learning_rate: f32,

    /// When set to true, will log during training
    log_output: bool,
}

impl SimpleLinearRegression {
    // Perform one step of gradient descent to update coefficient and intercept
    fn gradient_descent_step(&mut self, inputs: &Vec<f32>, targets: &Vec<f32>) {
        let n = inputs.len() as f32;
        let predictions: Vec<f32> = inputs.iter().map(|f| self.predict(*f)).collect();

        // Compute gradients
        let mut d_coefficient: f32 = 0.0;
        let mut d_intercept: f32 = 0.0;

        for (x, (y_pred, y_true)) in inputs.iter().zip(predictions.iter().zip(targets)) {
            let error = y_pred - y_true;
            d_coefficient += error * x;
            d_intercept += error;
        }

        // Make the coefficient the average error
        d_coefficient /= n;
        d_intercept /= n;

        // Update the coefficient and intercept using the learning rate
        self.coefficient -= self.learning_rate * d_coefficient;
        self.intercept -= self.learning_rate * d_intercept;
    }
}

impl LinearRegressionTrait for SimpleLinearRegression {
    fn new() -> Self {
        let uniform: Uniform<f32> = Uniform::new_inclusive(0.0, 1.0);
        let mut rng = rand::thread_rng();
        Self {
            coefficient: uniform.sample(&mut rng),
            intercept: uniform.sample(&mut rng),
            cost_function: CostFunction::mean_square_error,
            learning_rate: 1.0,
            log_output: false,
        }
    }

    fn set_cost(mut self, cost_function: fn(&Vec<f32>, &Vec<f32>) -> f32) -> Self {
        self.cost_function = cost_function;
        self
    }

    fn set_learning_rate(mut self, learning_rate: f32) -> Self {
        self.learning_rate = learning_rate;
        self
    }

    fn set_logging(mut self, log_output: bool) -> Self {
        self.log_output = log_output;
        self
    }

    fn fit(&mut self, x_train: &Vec<f32>, y_train: &Vec<f32>, epochs: usize) {
        for i in 0..epochs {
            // Perform one gradient descent
            self.gradient_descent_step(x_train, y_train);
            let cost = self.cost(x_train, y_train);

            // Log the cost of the epoch
            if self.log_output {
                println!("Epoch {}: {} cost", i + 1, cost);
            }
        }
    }

    fn predict(&self, x_feature: f32) -> f32 {
        self.intercept + self.coefficient * x_feature
    }

    fn cost(&self, x_train: &Vec<f32>, y_train: &Vec<f32>) -> f32 {
        (self.cost_function)(x_train, y_train)
    }
}
