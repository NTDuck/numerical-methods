use lu_factorization::{ColumnVector, Matrix, solve};

#[test]
fn test() {
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

    println!("{}", x);
}
