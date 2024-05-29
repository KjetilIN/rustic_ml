use criterion::{self, black_box, criterion_group, criterion_main, Criterion};
use rustic_ml::matrix::Matrix;


fn benchmark_matrix_multiplication(c: &mut Criterion) {
    // Define matrix sizes and data
    let size = 100; // Example size, adjust as needed
    let data_a: Vec<f32> = (0..size * size).map(|x| x as f32).collect();
    let data_b: Vec<f32> = (0..size * size).map(|x| x as f32).collect();

    // Create matrices
    let mat_a = Matrix::from_vec(size, data_a);
    let mat_b = Matrix::from_vec(size, data_b);

    // Benchmark the matrix multiplication
    c.bench_function("matrix_multiplication", |b| {
        b.iter(|| {
            let result: Matrix = mat_a.multiply(black_box(&mat_b)).unwrap();
            let _ = black_box(result);
        });
    });
}

fn criterion_config() -> Criterion {
    Criterion::default().sample_size(60).measurement_time(std::time::Duration::new(10, 0))
}

criterion_group!{
    name = benches;
    config = criterion_config();
    targets = benchmark_matrix_multiplication
}
criterion_main!(benches);