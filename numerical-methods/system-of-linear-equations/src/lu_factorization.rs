use nalgebra::{DMatrix, DVector};

use crate::Num;

/// Algorithm: [R. L. Burden & J. D. Faires, CH06_5A Matrix Factorization.pdf (p. 28-30)](https://www.math.hkust.edu.hk/~mamu/courses/231/Slides/CH06_5A.pdf)
pub fn solve(a: &DMatrix<Num>, b: &DVector<Num>) -> Option<DVector<Num>> {
    assert!(a.is_square());
    let n = a.nrows();

    let mut l = DMatrix::<Num>::identity(n, n);
    let mut u = DMatrix::<Num>::zeros(n, n);

    // Step 1
    // l[(0, 0)] = 1.0;
    u[(0, 0)] = a[(0, 0)] / l[(0, 0)];
    
    if u[(0, 0)] == 0.0 {
        return None;
    }
    
    // Step 2
    for j in 1..n {
        u[(0, j)] = a[(0, j)] / l[(0, 0)]; // First row of U
        l[(j, 0)] = a[(j, 0)] / u[(0, 0)]; // First col of L
    }

    // Step 3
    for i in 1..n {
        // Step 4
        let sum = (0..i)
            .map(|k| l[(i, k)] * u[(k, i)])
            .sum::<Num>();

        // l[(i, i)] = 1.0;
        u[(i, i)] = (a[(i, i)] - sum) / l[(i, i)];

        if u[(i, i)] == 0.0 {
            return None;
        }

        // Step 5
        for j in i + 1..n {
            let sum = (0..i)
                .map(|k| l[(i, k)] * u[(k, j)])
                .sum::<Num>();
            
            u[(i, j)] = (a[(i, j)] - sum) / l[(i, i)]; // i-th row of U

            let sum = (0..i)
                .map(|k| l[(j, k)] * u[(k, i)])
                .sum::<Num>();

            l[(j, i)] = (a[(j, i)] - sum) / u[(i, i)]; // i-th col of L
        }
    }

    // Step 6
    let sum = (0..n - 1)
        .map(|k| l[(n - 1, k)] * u[(k, n - 1)])
        .sum::<Num>();

    // l[(n - 1, n - 1)] = 1.0;
    u[(n - 1, n - 1)] = (a[(n - 1, n - 1)] - sum) / l[(n - 1, n - 1)];

    // Forward substitution
    let mut y = DVector::<Num>::zeros(n);

    for i in 0..n {
        let sum = (0..i)
            .map(|j| l[(i, j)] * y[j])
            .sum::<Num>();
        y[i] = (b[i] - sum) / l[(i, i)];
    }

    // Backward substitution
    let mut x = DVector::<Num>::zeros(n);

    for i in (0..n).rev() {
        let sum = (i + 1..n)
            .map(|j| u[(i, j)] * x[j])
            .sum::<Num>();
        x[i] = (y[i] - sum) / u[(i, i)];
    }

    Some(x)
}
