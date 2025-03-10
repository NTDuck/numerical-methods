pub type Number = f64;
pub type Matrix<const N: usize> = [[Number; N]; N];
pub type ColumnVector<const N: usize> = [Number; N];

pub fn factor<const N: usize>(a: Matrix<N>) -> (Matrix<N>, Matrix<N>) {
    let mut l = [[0.0; N]; N];
    let mut u = [[0.0; N]; N];

    // Step 1
    l[0][0] = 1.0;
    u[0][0] = a[0][0] / l[0][0]; // May panic

    // Step 2
    for j in 1..N {
        u[0][j] = a[0][j] / l[0][0]; // First row of U
        l[j][0] = a[j][0] / u[0][0]; // First column of L
    }

    // Step 3
    for i in 1..N - 1 {
        // Step 4
        l[i][i] = 1.0;
        let sum: Number = (0..i)
            .map(|k| l[i][k] * u[k][i])
            .sum();
        u[i][i] = (a[i][i] - sum) / l[i][i]; // May panic

        // Step 5
        for j in i + 1..N {
            let sum_u: Number = (0..i)
                .map(|k| l[i][k] * u[k][j])
                .sum();
            u[i][j] = (a[i][j] - sum_u) / l[i][i]; // i-th row of U

            let sum_l: Number = (0..i)
                .map(|k| l[j][k] * u[k][i])
                .sum();
            l[j][i] = (a[j][i] - sum_l) / u[i][i]; // i-th column of L
        }
    }

    // Step 6
    l[N - 1][N - 1] = 1.0;
    let sum: Number = (0..N - 1)
        .map(|k| l[N - 1][k] * u[k][N - 1])
        .sum();
    u[N - 1][N - 1] = (a[N - 1][N - 1] - sum) / l[N - 1][N - 1]; // May not panic

    // Step 7
    (l, u)
}

// pub fn forward_substitution<const N: usize>(l: Matrix<N>, b: ColumnVector<N>) -> ColumnVector<N> {
//     todo!()
// }

// pub fn backward_substitution<const N: usize>(u: Matrix<N>, y: ColumnVector<N>) -> ColumnVector<N> {
//     todo!()
// }
