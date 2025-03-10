use lu_factorization::factor;

#[test]
fn test_factor() {
    let a = [
        [1.0, -1.0, 0.0, 3.0],
        [2.0, 1.0, -1.0, 1.0],
        [3.0, -1.0, -1.0, 2.0],
        [-1.0, 2.0, 3.0, -1.0],
    ];
    let (l, u) = factor(a);

    dbg!(l);
    dbg!(u);
}
