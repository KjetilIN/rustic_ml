use rustic_ml::{activation::Activation, data_utils::matrix::Matrix, layer::{activationlayer::ActivationLayer, denselayer::DenseLayer}, network::Network};

fn main() {
    // Create a network with appropriate architecture for XOR
    let mut network = Network::init()
        .learning_rate(0.01)
        .add_layer(Box::new(DenseLayer::new(2, 2)))  // 2 inputs -> 2 hidden
        .add_layer(Box::new(ActivationLayer::with(Activation::SIGMOID)))
        .add_layer(Box::new(DenseLayer::new(2, 1)))  // 2 hidden -> 1 output
        .add_layer(Box::new(ActivationLayer::with(Activation::SIGMOID)));  // Output activation

    // XOR training data
    let inputs = vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ];

    let targets = vec![
        vec![0.0],  // 0 XOR 0 = 0
        vec![1.0],  // 0 XOR 1 = 1
        vec![1.0],  // 1 XOR 0 = 1
        vec![0.0],  // 1 XOR 1 = 0
    ];

    // Convert to matrices
    let input_matrices: Vec<Matrix> = inputs.into_iter()
        .map(|data| Matrix {
            data,
            rows: 2,
            cols: 1,
        })
        .collect();

    let target_matrices: Vec<Matrix> = targets.into_iter()
        .map(|data| Matrix {
            data,
            rows: 1,
            cols: 1,
        })
        .collect();

    // Training loop
    let epochs = 10000;  // Increased epochs for better convergence
    for epoch in 0..epochs {
        let mut epoch_loss = 0.0;
        
        // Train on each example
        for (input, target) in input_matrices.iter().zip(target_matrices.iter()) {
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

        if epoch % 1000 == 0 {
            println!("Epoch: {}, Average Loss: {:.6}", epoch, epoch_loss / input_matrices.len() as f32);
        }
    }

    // Test the network
    println!("\nTesting XOR function:");
    for (input, target) in input_matrices.iter().zip(target_matrices.iter()) {
        let output = network.forward(&mut input.clone());
        println!(
            "{:.0} XOR {:.0} = {:.3} (expected {:.0})", 
            input.data[0], 
            input.data[1], 
            output.data[0], 
            target.data[0]
        );
    }
}

fn calculate_loss(output: &Matrix, target: &Matrix) -> f32 {
    let mut loss = 0.0;
    for (o, t) in output.data.iter().zip(target.data.iter()) {
        loss += (o - t).powi(2);
    }
    loss / output.data.len() as f32
}