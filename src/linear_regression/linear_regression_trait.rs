/// Trait for all types of linear regression models
pub trait LinearRegressionTrait {
    fn new() -> Self;
    fn set_cost(self, cost_function: fn(&Vec<f32>, &Vec<f32>) -> f32) -> Self;
    fn set_learning_rate(self, learning_rate:f32) -> Self;
    fn fit(&mut self, x_train: &Vec<(f32, f32)>, y_train: &Vec<f32>);
    fn score(&self) -> f32;   
}