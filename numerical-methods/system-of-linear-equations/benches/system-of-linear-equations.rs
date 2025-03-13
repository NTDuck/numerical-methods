use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use system_of_linear_equations::{
    gaussian_elimination_with_backward_substitution::solve as gaussian_elimination,
    lu_factorization::solve as lu_factorization,
};
use nalgebra::{DMatrix, DVector};
use rand::Rng;

fn generate_random_system(n: usize) -> (DMatrix<f64>, DVector<f64>) {
    let mut rng = rand::rng();
    let a = DMatrix::from_fn(n, n, |_, _| rng.random_range(-10.0..10.0));
    let b = DVector::from_fn(n, |_, _| rng.random_range(-10.0..10.0));
    (a, b)
}

fn benchmark_solvers(c: &mut Criterion) {
    let sizes = [10, 50, 100, 200, 500];

    let mut group = c.benchmark_group("Solvers");

    for &size in &sizes {
        let (a, b) = generate_random_system(size);

        group.bench_with_input(BenchmarkId::new("Gaussian", size), &size, |bencher, _| {
            bencher.iter(|| gaussian_elimination(&a, &b))
        });

        group.bench_with_input(BenchmarkId::new("LU Factorization", size), &size, |bencher, _| {
            bencher.iter(|| lu_factorization(&a, &b))
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_solvers);
criterion_main!(benches);
