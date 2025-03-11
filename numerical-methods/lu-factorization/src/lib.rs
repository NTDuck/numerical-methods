pub use column_vector::ColumnVector;
pub use matrix::Matrix;

mod column_vector;
mod matrix;

pub type Number = f64;

pub fn solve<const N: usize>(a: Matrix<N>, b: ColumnVector<N>) -> ColumnVector<N> {
    let (l, u) = decompose(a);
    let y = forward_substitution(l, b);
    backward_substitution(u, y)
}

fn decompose<const N: usize>(a: Matrix<N>) -> (Matrix<N>, Matrix<N>) {
    let mut l: Matrix<N> = Matrix::default();
    let mut u: Matrix<N> = Matrix::default();

    // Step 1
    l.set((1, 1), 1.0);
    u.set((1, 1), a.get((1, 1)) / l.get((1, 1)));

    // Step 2
    for j in 2..N + 1 {
        u.set((1, j), a.get((1, j)) / l.get((1, 1))); // First row of U
        l.set((j, 1), a.get((j, 1)) / u.get((1, 1))); // First column of L
    }

    // Step 3
    for i in 2..N {
        // Step 4
        l.set((i, i), 1.0);
        let sum: Number = (1..i)
            .map(|k| l.get((i, k)) * u.get((k, i)))
            .sum();
        u.set((i, i), (a.get((i, i)) - sum) / l.get((i, i))); // May panic

        // Step 5
        for j in i + 1..N + 1 {
            let sum: Number = (1..i)
                .map(|k| l.get((i, k)) * u.get((k, j)))
                .sum();
            u.set((i, j), (a.get((i, j)) - sum) / l.get((i, i))); // i-th row of U

            let sum: Number = (1..i)
                .map(|k| l.get((j, k)) * u.get((k, i)))
                .sum();
            l.set((j, i), (a.get((j, i)) - sum) / u.get((i, i))); // i-th column of L
        }
    }

    // Step 6
    l.set((N, N), 1.0);
    let sum: Number = (1..N)
        .map(|k| l.get((N, k)) * u.get((k, N)))
        .sum();
    u.set((N, N), (a.get((N, N)) - sum) / l.get((N, N))); // May not panic

    // Step 7
    (l, u)
}

fn forward_substitution<const N: usize>(l: Matrix<N>, b: ColumnVector<N>) -> ColumnVector<N> {
    let mut y: ColumnVector<N> = ColumnVector::default();

    for i in 1..N + 1 {
        let sum: Number = (1..i + 1)
            .map(|j| l.get((i, j)) * y.get(j))
            .sum();
        y.set(i, (b.get(i) - sum) / l.get((i, i)));
    }

    y
}

fn backward_substitution<const N: usize>(u: Matrix<N>, y: ColumnVector<N>) -> ColumnVector<N> {
    let mut x: ColumnVector<N> = ColumnVector::default();

    for i in (1..N + 1).rev() {
        let sum: Number = (i + 1..N + 1)
            .map(|j| u.get((i, j)) * x.get(j))
            .sum();
        x.set(i, (y.get(i) - sum) / u.get((i, i)));
    }

    x
}
