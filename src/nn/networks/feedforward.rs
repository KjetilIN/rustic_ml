use crate::{matrix::{Matrix, MatrixError}, nn::activation::sigmoid};


/// Neural Network structure
/// 
/// Contains
/// - layers: count that contains how many layers is in the network
/// - sizes: vector of the number of neurons in each layer
/// - biases: biases of the whole network as a vector of matrixes
/// - weights: weights of the whole network as a vector of matrixes 
struct NN{
    pub layers: usize,
    pub sizes: Vec<u32>,
    pub biases: Vec<Matrix>,
    pub weights: Vec<Matrix>
}

impl NN {
    /// Create a new neural network
    /// 
    /// Takes the vector of neurons. 
    /// Note that this constructor assumes that the first layer is the input layer, adn that 
    /// 
    /// # Example 
    /// Creating a network with 3 nodes in the input layer, 5 nodes in the hidden layer
    /// and 2 layers in the output layer: 
    /// ```rust
    /// let net = NN::new(vec![3,5,2]);
    /// assert_eq!(net.layers, 3);
    /// ```
    pub fn new(sizes: Vec<u32>) -> Self{

        let mut biases: Vec<Matrix> = Vec::new();
        let mut weights: Vec<Matrix> = Vec::new();

        // Create biases for all except the first input layer
        // Each bias as a vector for each layer 
        for i in &sizes[1..]{
            let bias = Matrix::with_rand_bin(*i as usize, 1);
            biases.push(bias);
        }

        // Create the weights for all 
        for (x, y) in sizes[0..sizes.len()].iter().zip(&sizes[1..]){
            let weight = Matrix::with_rand_bin(*y as usize, *x as usize);
            weights.push(weight);
        }

        NN { 
            layers: sizes.len(), 
            sizes, 
            biases, 
            weights, 
        }
    }


    pub fn feed_forward(&mut self, mut a: Matrix) -> Result<Matrix, MatrixError>{
        for (bias, weight) in self.biases.iter().zip(&self.weights){
            a = weight.multiply(&a)?;
        }

        unimplemented!("Not done");

        Ok(a)
    }
    
}