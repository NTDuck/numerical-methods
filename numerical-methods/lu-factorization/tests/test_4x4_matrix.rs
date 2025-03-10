use lu_factorization::{column_vector::ColumnVector, matrix::Matrix, solve};

#[test]
fn test_factor() {
    let a = Matrix::from([
        [1.0, -1.0, 0.0, 3.0],
        [2.0, 1.0, -1.0, 1.0],
        [3.0, -1.0, -1.0, 2.0],
        [-1.0, 2.0, 3.0, -1.0],
    ]);
    let b = ColumnVector::from([
        1.0,
        1.0,
        -3.0,
        4.0,
    ]);
    let x = solve(a, b);

    dbg!(x);
}
