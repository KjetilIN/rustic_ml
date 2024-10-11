use std::iter::zip;

pub enum CostFunctionOption {
    /// Mean square error
    MSE,

    /// Root mean square error
    RMSE,

    /// Mean absolute error
    MAE,
}

impl CostFunctionOption {
    /// Get the cost function for the `CostFunctionOption`
    ///
    /// Returns the cost function
    pub fn get_fn(&self) -> fn(&Vec<f32>, &Vec<f32>) -> f32 {
        match &self {
            CostFunctionOption::MSE => CostFunction::mean_absolute_error,
            CostFunctionOption::RMSE => CostFunction::root_mean_squared_error,
            CostFunctionOption::MAE => CostFunction::mean_absolute_error,
        }
    }
}

/// Commonly used implementations of cost functions
///
/// Resource: <https://www.geeksforgeeks.org/ml-linear-regression/#cost-function-for-linear-regression>
pub struct CostFunction {}

#[allow(dead_code)]
impl CostFunction {
    /// MSE - mean square error cost function.
    ///
    /// # Arguments
    /// - `predictions`: a vector of predictions made over the dataset
    /// - `targets`: a vector of ground truth target values.
    ///
    /// # Returns
    /// The calculated mean squared error between the predictions and the target
    pub fn mean_square_error(predictions: &Vec<f32>, targets: &Vec<f32>) -> f32 {
        // Assert that they have the same length
        assert!(predictions.len() == targets.len());
        let n = predictions.len() as f32;
        let mut square_sum: f32 = 0.0;

        // Loop over the predictions
        for (predict, target) in zip(predictions, targets) {
            square_sum += (predict - target).powf(2.0);
        }

        return square_sum / n;
    }

    pub fn mse_derivative() {
        unimplemented!()
    }

    /// RMSE - root mean square error cost function.
    ///
    /// # Arguments
    /// - `predictions`: a vector of predictions made over the dataset
    /// - `targets`: a vector of ground truth target values.
    ///
    /// # Returns
    /// The calculated the root mean squared error between the predictions and the target
    pub fn root_mean_squared_error(predictions: &Vec<f32>, targets: &Vec<f32>) -> f32 {
        return Self::mean_square_error(predictions, targets).sqrt();
    }

    pub fn partial_rmse() {
        unimplemented!()
    }

    /// MAE - mean absolute error cost function.
    ///
    /// # Arguments
    /// - `predictions`: a vector of predictions made over the dataset
    /// - `targets`: a vector of ground truth target values.
    ///
    /// # Returns
    /// The calculated the mean absolute error between the predictions and the target
    pub fn mean_absolute_error(predictions: &Vec<f32>, targets: &Vec<f32>) -> f32 {
        // Assert that they have the same length
        assert!(predictions.len() == targets.len());
        let n = predictions.len() as f32;
        let mut square_sum: f32 = 0.0;

        // Loop over the predictions
        for (predict, target) in zip(predictions, targets) {
            square_sum += (predict - target).abs();
        }

        return square_sum / n;
    }

    pub fn partial_mae() {
        unimplemented!()
    }
}
