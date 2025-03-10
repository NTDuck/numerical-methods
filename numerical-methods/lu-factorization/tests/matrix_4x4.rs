use lu_factorization::{factor, matrix::Matrix};

#[test]
fn test_factor() {
    let a = Matrix::from([
        [1.0, -1.0, 0.0, 3.0],
        [2.0, 1.0, -1.0, 1.0],
        [3.0, -1.0, -1.0, 2.0],
        [-1.0, 2.0, 3.0, -1.0],
    ]);
    let (l, u) = factor(a);

    dbg!(l);
    dbg!(u);
}
