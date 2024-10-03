use criterion::{self, black_box, criterion_group, criterion_main, Criterion};
use rustic_ml::data_utils::matrix::Matrix;

fn benchmark_matrix_multiplication(c: &mut Criterion) {
    // Define matrix sizes and data
    let size = 100;
    let data_a: Vec<f32> = (0..size * size).map(|x| x as f32).collect();
    let data_b: Vec<f32> = (0..size * size).map(|x| x as f32).collect();

    // Create matrices
    let mat_a = Matrix::from_vec(size, data_a);
    let mat_b = Matrix::from_vec(size, data_b);

    // Benchmark the matrix multiplication
    c.bench_function("matrix_multiplication_size_100", |b| {
        b.iter(|| {
            let result: Matrix = mat_a.multiply(black_box(&mat_b)).unwrap();
            let _ = black_box(result);
        });
    });
}

fn criterion_small_config() -> Criterion {
    Criterion::default()
        .sample_size(100)
        .measurement_time(std::time::Duration::new(10, 0))
        .warm_up_time(std::time::Duration::new(3, 0))
}

criterion_group! {
    name = small_benches;
    config = criterion_small_config();
    targets = benchmark_matrix_multiplication
}

// MORE EXTENSIVE BENCHMARK TESTING

#[cfg(feature = "extensive_benchmark")]
fn benchmark_large_matrix_multiplication(c: &mut Criterion) {
    // Define large matrix sizes and data
    let size = 1000;
    let data_a: Vec<f32> = (0..size * size).map(|x| x as f32).collect();
    let data_b: Vec<f32> = (0..size * size).map(|x| x as f32).collect();

    // Create matrices
    let mat_a = Matrix::from_vec(size, data_a);
    let mat_b = Matrix::from_vec(size, data_b);

    // Benchmark the large matrix multiplication
    c.bench_function("matrix_multiplication_size_1000", |b| {
        b.iter(|| {
            let result = mat_a.multiply(black_box(&mat_b)).unwrap();
            black_box(result);
        });
    });
}

#[cfg(feature = "extensive_benchmark")]
fn criterion_large_config() -> Criterion {
    Criterion::default()
        .sample_size(40)
        .measurement_time(std::time::Duration::new(60, 0))
        .warm_up_time(std::time::Duration::new(5, 0))
}

#[cfg(feature = "extensive_benchmark")]
criterion_group! {
    name = large_benches;
    config = criterion_large_config();
    targets = benchmark_large_matrix_multiplication
}

#[cfg(not(feature = "extensive_benchmark"))]
criterion_main!(small_benches);

#[cfg(feature = "extensive_benchmark")]
criterion_main!(small_benches, large_benches);
