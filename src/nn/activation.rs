use std::f32::consts::E;

use crate::matrix::Matrix;

/// Step activation function
/// 
/// Implemented from the function: <https://en.wikipedia.org/wiki/Step_function>
pub fn step(x:f32) -> f32{
    if x >= 0.0{
        return 1.0
    }
    return 0.0
}

/// Sigmoid activation function
/// 
/// Implemented from the function: <https://en.wikipedia.org/wiki/Sigmoid_function>
pub fn sigmoid(x:f32) -> f32{
    return 1.0 / (1.0 + E.powf(-x));
}

/// Sigmoid activation function for `Matrix`
/// 
/// Uses the `sigmoid` function for each element in the matrix
pub fn sigmoid_mat(mat:&mut Matrix){
    for i in 0..mat.rows {
        for j in 0..mat.cols {
            mat.data[i*mat.cols + j] = sigmoid(mat.data[i*mat.cols + j]);
        }
    }
}


/// ReLu activation function
/// 
/// Implemented from the function: <https://en.wikipedia.org/wiki/Rectifier_(neural_networks)>
pub fn relu(x:f32) -> f32{
    if x > 0.0{
        return x;
    }
    return 0.0
}

/// Leaky ReLu activation function
/// 
/// Implemented from the function: <https://en.wikipedia.org/wiki/Rectifier_(neural_networks)>
pub fn leaky_relu(x:f32) -> f32{
    if x > 0.0{
        return x;
    }
    return 0.1*x;
}

/// Softplus activation function
/// 
/// Implemented from the function: <https://en.wikipedia.org/wiki/Activation_function>
pub fn softplus(x:f32) ->f32{
    return (1.0 +  E.powf(x)).ln()
}


/// ELU activation function
/// 
/// Implemented from the function: <https://en.wikipedia.org/wiki/Activation_function>
pub fn elu(x:f32, alpha:f32) -> f32{
    if x > 0.0{
        return x;
    }
    return alpha*(E.powf(x) - 1.0);
}

/// Softmax activation function
/// 
/// Implemented from the function: <https://en.wikipedia.org/wiki/Softmax_function>
pub fn softmax(x:f32) -> f32{
    unimplemented!()
}
