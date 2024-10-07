/// Trait for all types of linear regression models
pub trait LinearRegressionTrait {
    fn new() -> Self;
    fn fit(&mut self);
    fn score(&self) -> f32;   
}