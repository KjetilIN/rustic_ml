/// Trait for all types of linear regression models
pub trait LinearRegressionTrait {
    fn new() -> Self;
    fn set_cost(self, cost_function: fn(&Vec<f32>, &Vec<f32>) -> f32) -> Self;
    fn set_learning_rate(self, learning_rate:f32) -> Self;
    fn set_logging(self, log_output:bool) -> Self;
    fn fit(&mut self, x_train: &Vec<f32>, y_train: &Vec<f32>, epochs: usize);
    fn predict(&self, x_features: f32) -> f32;
    fn cost(&self, x_train: &Vec<f32>, y_train: &Vec<f32>) -> f32;
}