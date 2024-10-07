use rand::{distributions::Uniform, prelude::Distribution};

use super::linear_regression_trait::LinearRegressionTrait;

pub struct SimpleLinearRegression{
    coefficient: f32,
    intercept: f32
}

impl LinearRegressionTrait for SimpleLinearRegression {
    fn new() -> Self {
        let uniform: Uniform<f32> = Uniform::new_inclusive(0.0, 1.0);
        let mut rng = rand::thread_rng();
        Self {
            coefficient: uniform.sample(&mut rng),
            intercept: uniform.sample(&mut rng),
        }
    }

    fn fit(&mut self) {
        todo!()
    }

    fn score(&self) -> f32 {
        todo!()
    }
}