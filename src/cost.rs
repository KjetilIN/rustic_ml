use std::iter::zip;


/// Commonly used implementations of cost functions 
/// 
/// Resource: <https://www.geeksforgeeks.org/ml-linear-regression/#cost-function-for-linear-regression>
pub struct CostFunction{}

#[allow(dead_code)]
impl CostFunction{

    /// MSE - mean square error cost function.
    /// 
    /// # Arguments
    /// - `predictions`: a vector of predictions made over the dataset
    /// - `targets`: a vector of ground truth target values.
    /// 
    /// # Returns 
    /// The calculated mean squared error between the predictions and the target
    fn mean_square_error(predictions:&Vec<f32>, targets: &Vec<f32>) -> f32{
        // Assert that they have the same length 
        assert!(predictions.len() == targets.len());
        let n = predictions.len() as f32;
        let mut square_sum: f32 = 0.0;

        // Loop over the predictions 
        for (predict, target) in zip(predictions, targets){
            square_sum += (predict - target).powf(n);
        }

        return square_sum / n;
    }

    /// RMSE - root mean square error cost function.
    /// 
    /// # Arguments
    /// - `predictions`: a vector of predictions made over the dataset
    /// - `targets`: a vector of ground truth target values.
    /// 
    /// # Returns 
    /// The calculated the root mean squared error between the predictions and the target
    fn root_mean_squared_error(predictions:&Vec<f32>, targets: &Vec<f32>) -> f32{
        return Self::mean_square_error(predictions, targets).sqrt();
    }


    /// MAE - mean absolute error cost function.
    /// 
    /// # Arguments
    /// - `predictions`: a vector of predictions made over the dataset
    /// - `targets`: a vector of ground truth target values.
    /// 
    /// # Returns 
    /// The calculated the mean absolute error between the predictions and the target
    fn mean_absolute_error(predictions:&Vec<f32>, targets: &Vec<f32>) ->f32{
        // Assert that they have the same length 
        assert!(predictions.len() == targets.len());
        let n = predictions.len() as f32;
        let mut square_sum: f32 = 0.0;

        // Loop over the predictions 
        for (predict, target) in zip(predictions, targets){
            square_sum += (predict - target).abs();
        }

        return square_sum / n;
    }

}