use crate::{matrix::{Matrix, MatrixError}, nn::activation::sigmoid_mat};


/// Neural Network structure
/// 
/// Contains
/// - layers: count that contains how many layers is in the network
/// - sizes: vector of the number of neurons in each layer
/// - biases: biases of the whole network as a vector of matrixes
/// - weights: weights of the whole network as a vector of matrixes 
struct NeuralNetwork{
    pub layers: usize,
    pub sizes: Vec<u32>,
    pub biases: Vec<Matrix>,
    pub weights: Vec<Matrix>
}

impl NeuralNetwork {
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

        NeuralNetwork { 
            layers: sizes.len(), 
            sizes, 
            biases, 
            weights, 
        }
    }


    pub fn feed_forward(&mut self, a: &Matrix) -> Result<Matrix, MatrixError>{
        // Cloning the input layer (ok for now since this clone will not preform that bad)
        let mut layer_output = a.clone();

        for (bias, weight) in self.biases.iter().zip(&self.weights){
            // 1. Multiply the weight by the input layer
            layer_output = weight.multiply(&layer_output)?;

            // 2. Add the vector of biases 
            layer_output.add_vec(bias)?;

            // 3. Use the sigmoid activation function for all values in the input layer 
            sigmoid_mat(&mut layer_output);

            // 4. Given matrix layer is now the new input layer for the next level
        }

        // 5. Return the input layer as the output layer
        Ok(layer_output)
    }

    pub fn apply_stochastic_gradient_descent(){
        todo!()
    }

    fn update_mini_batch(&mut self, mini_batch:Vec<(f32, f32)>, learning_rate:f32){
        // Initialize nabla_b => partial derivative with respect to the biases
        let mut nabla_b: Vec<Matrix> = self.biases.iter().map(|b| Matrix::new(b.rows, b.cols)).collect();
        
        // Initialize nabla_w => partial derivative with respect to the weights
        let mut nabla_w: Vec<Matrix> = self.weights.iter().map(|w| Matrix::new(w.rows, w.cols)).collect();

        // Loop through the training set
        for &(x, y) in mini_batch.iter() {
            let (delta_nabla_b, delta_nabla_w) = self.backprop(&x, &y);
            for nb in nabla_b.iter_mut() {
                *nb += &delta_nabla_b;
            }
            for nw in nabla_w.iter_mut() {
                *nw += &delta_nabla_w;
            }
        }

        let mini_batch_size: f32 = mini_batch.len() as f32;

        // Update weights and biases
        for (w, nw) in self.weights.iter_mut().zip(nabla_w.iter()) {
            *w -= &(nw * (learning_rate / mini_batch_size));
        }
        for (b, nb) in self.biases.iter_mut().zip(nabla_b.iter()) {
            *b -= &(nb * (learning_rate / mini_batch_size));
        }

        todo!()
    }
    
    fn backprop(&self, x: &f32, y:&f32) -> (f32, f32){
        todo!()
    }
}