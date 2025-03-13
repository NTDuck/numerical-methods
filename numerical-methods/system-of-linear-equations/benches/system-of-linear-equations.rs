use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use nalgebra::{DMatrix, DVector};
use system_of_linear_equations::{
    gaussian_elimination_with_backward_substitution::solve as gaussian_elimination_with_backward_substitution,
    lu_factorization::solve as lu_factorization, Num,
};
use rand::Rng;

fn benchmark_system_of_linear_equations_solvers(criterion: &mut Criterion) {
    const DIMENSIONS: [usize; 4] = [10, 100, 200, 500];
    const BOUNDS: [Num; 5] = [1e2, 1e10, 1e100, 1e200, 1e300];

    let mut benchmark_group = criterion.benchmark_group("System of Linear Equations solvers");

    for dimension in DIMENSIONS {
        for bound in BOUNDS {
            let (a, b) = generate_random_system(dimension, -bound, bound);
            let configs = (dimension, bound);

            benchmark_group.bench_with_input(
                BenchmarkId::new(format!("Gaussian Elimination, N={}, eps={:e}", dimension, bound), dimension),
                &configs,
                |bencher, _| {
                    bencher.iter(|| gaussian_elimination_with_backward_substitution(&a, &b))
                },
            );

            benchmark_group.bench_with_input(
                BenchmarkId::new(format!("LU Factorization, N={}, eps={:e}", dimension, bound), dimension),
                &configs,
                |bencher, _| {
                    bencher.iter(|| lu_factorization(&a, &b))
                },
            );
        }
    }

    benchmark_group.finish();
}

fn generate_random_system(
    n: usize,
    lower_bound: Num,
    upper_bound: Num,
) -> (DMatrix<f64>, DVector<f64>) {
    assert!(lower_bound < upper_bound);

    let mut rng = rand::rng();

    let a = DMatrix::from_fn(n, n, |_, _|
        rng.random_range(lower_bound..upper_bound));
    let b = DVector::from_fn(n, |_, _|
        rng.random_range(lower_bound..upper_bound));
    
    (a, b)
}

criterion_group!(benches, benchmark_system_of_linear_equations_solvers);
criterion_main!(benches);
