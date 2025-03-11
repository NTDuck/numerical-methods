use lu_factorization::{solve, ColumnVector, Matrix};
use rand::Rng;

#[test]
fn test() {
    const N: usize = 161; // Current limit
    let a: Matrix<N> = random_matrix();
    let b: ColumnVector<N> = random_column_vector();
    let x = solve(a, b);

    println!("{}", x);
}

fn random_matrix<const N: usize>() -> Matrix<N> {
    let mut rng = rand::rng();
    let mut matrix = [[0.0; N]; N];

    for i in 0..N {
        for j in 0..N {
            let mut value = 0.0;
            while value == 0.0 {
                value = rng.random_range(-10.0..10.0)
            }
            matrix[i][j] = value;
        }
    }

    Matrix::from(matrix)
}

fn random_column_vector<const N: usize>() -> ColumnVector<N> {
    let mut rng = rand::rng();
    let mut column_vector = [0.0; N];

    for i in 0..N {
        let mut value = 0.0;
        while value == 0.0 {
            value = rng.random_range(-10.0..10.0)
        }
        column_vector[i] = value;
    }

    ColumnVector::from(column_vector)
}
