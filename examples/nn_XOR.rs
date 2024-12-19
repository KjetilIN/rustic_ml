use rustic_ml::{activation::Activation, data_utils::matrix::Matrix, layer::{activationlayer::ActivationLayer, denselayer::DenseLayer}, network::Network};

fn main() {
    // Create a network with a learning rate of 0.1
    let mut network = Network::init()
        .learning_rate(0.1)
        .add_layer(Box::new(DenseLayer::new(3, 2))) // Dense layer with 3 inputs and 2 outputs
        .add_layer(Box::new(ActivationLayer::with(Activation::RELU))); // ReLU activation

     // Define some mock data for training (e.g., XOR problem)
     let inputs = vec![
        vec![0.0, 0.0, 0.0],
        vec![0.0, 0.0, 1.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 1.0, 1.0],
    ];

    let targets = vec![
        vec![0.0, 0.0], // Expected output for input [0, 0, 0]
        vec![0.0, 1.0], // Expected output for input [0, 0, 1]
        vec![1.0, 0.0], // Expected output for input [0, 1, 0]
        vec![1.0, 1.0], // Expected output for input [0, 1, 1]
    ];

    // Convert inputs and targets to Matrix objects
    let input_matrices: Vec<Matrix> = inputs.into_iter()
        .map(|data| Matrix {
            data,
            rows: 3,     // 1 sample per matrix
            cols: 1,     // 3 inputs per example
        })
        .collect();

    let target_matrices: Vec<Matrix> = targets.into_iter()
        .map(|data| Matrix {
            data,
            rows: 1,     // 1 row per matrix
            cols: 2,     // 2 outputs per example (e.g., one-hot encoded XOR)
        })
        .collect();


    // Train the network for 1000 iterations
    for epoch in 0..1000 {
        for (input, target) in input_matrices.iter().zip(target_matrices.iter()) {
            // Forward pass
            let output = network.forward(&mut input.clone());

            // Calculate the loss (mean squared error for simplicity)
            let loss = calculate_loss(&output, &target);
            println!("Epoch: {}, Loss: {}", epoch, loss);

            // Backward pass (use the target as the gradient for simplicity)
            let gradient = output - target.clone();
            network.backward(gradient);
        }
    }

    // Final output after training
    let final_input = Matrix {
        data: vec![1.0, 1.0, 1.0], // Test input
        rows: 1,
        cols: 3,
    };
    let final_output = network.forward(&mut final_input.clone());
    println!("Final Output for input [1, 1, 1]: {:?}", final_output);
}

// Simple loss function (Mean Squared Error)
fn calculate_loss(output: &Matrix, target: &Matrix) -> f32 {
    let mut loss = 0.0;
    for (o, t) in output.data.iter().zip(target.data.iter()) {
        loss += (o - t).powi(2);
    }
    loss / output.data.len() as f32
}
