use rustic_ml::{activation::Activation, data_utils::matrix::Matrix, layer::{activationlayer::ActivationLayer, denselayer::DenseLayer}, network::Network};

fn main() {
    // Create a network with a higher learning rate for faster convergence
    let mut network = Network::init()
        .learning_rate(0.3)  // Increased learning rate
        // First hidden layer with more neurons
        .add_layer(Box::new(DenseLayer::new(3, 6)))  // 3 inputs -> 6 hidden neurons
        .add_layer(Box::new(ActivationLayer::with(Activation::SIGMOID)))  // Changed to SIGMOID
        // Output layer
        .add_layer(Box::new(DenseLayer::new(6, 2)))  // 6 hidden -> 2 outputs
        .add_layer(Box::new(ActivationLayer::with(Activation::SIGMOID)));  // Output activation

    // Training data
    let inputs = vec![
        vec![0.0, 0.0, 0.0],
        vec![0.0, 0.0, 1.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 1.0, 1.0],
        // Add more training examples with 1 in first position
        vec![1.0, 0.0, 0.0],
        vec![1.0, 0.0, 1.0],
        vec![1.0, 1.0, 0.0],
        vec![1.0, 1.0, 1.0],
    ];

    let targets = vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
        vec![0.0, 0.0],  // Corresponding targets for additional inputs
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ];

    // Convert to matrices
    let input_matrices: Vec<Matrix> = inputs.into_iter()
        .map(|data| Matrix {
            data,
            rows: 3,
            cols: 1,
        })
        .collect();

    let target_matrices: Vec<Matrix> = targets.into_iter()
        .map(|data| Matrix {
            data,
            rows: 2,
            cols: 1,
        })
        .collect();

    // Training loop with mini-batches
    let epochs = 5000;
    for epoch in 0..epochs {
        let mut epoch_loss = 0.0;
        
        // Randomize training order
        let mut indices: Vec<usize> = (0..input_matrices.len()).collect();
        use rand::seq::SliceRandom;
        indices.shuffle(&mut rand::thread_rng());

        for &idx in indices.iter() {
            let input = &input_matrices[idx];
            let target = &target_matrices[idx];

            // Forward pass
            let output = network.forward(&mut input.clone());
            
            // Calculate loss
            let loss = calculate_loss(&output, target);
            epoch_loss += loss;

            // Calculate error gradient
            let mut gradient = Matrix::new(output.rows, output.cols);
            for i in 0..output.data.len() {
                gradient.data[i] = output.data[i] - target.data[i];
            }

            // Backward pass
            network.backward(gradient);
        }

        if epoch % 100 == 0 {
            println!("Epoch: {}, Average Loss: {}", epoch, epoch_loss / input_matrices.len() as f32);
        }
    }

    // Test the network
    println!("\nTesting the network:");
    for input in input_matrices.iter() {
        let output = network.forward(&mut input.clone());
        println!("Input: {:?}", input.data);
        println!("Output: [{:.3}, {:.3}]", output.data[0], output.data[1]);
    }
}

fn calculate_loss(output: &Matrix, target: &Matrix) -> f32 {
    let mut loss = 0.0;
    for (o, t) in output.data.iter().zip(target.data.iter()) {
        loss += (o - t).powi(2);
    }
    loss / output.data.len() as f32
}