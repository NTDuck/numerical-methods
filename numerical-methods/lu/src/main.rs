use nalgebra::{DMatrix, DVector};

fn main() {
    let a = DMatrix::<f64>::from_vec(4, 4, vec![
        1.0, 1.0, 0.0, 3.0,
        2.0, 1.0, -1.0, 1.0,
        3.0, -1.0, -1.0, 2.0,
        -1.0, 2.0, 3.0, -1.0
    ]);

    let b = DVector::<f64>::from_vec(vec![
        1.0,
        1.0,
        -3.0,
        4.0,
    ]);

    let x = lu::solve(&a, &b);
    let x = x.normalize();

    println!("{}", x);
    assert_eq!(x, DVector::<f64>::from_vec(vec![
        3.0,
        -1.0,
        0.0,
        -2.0,
    ]).normalize());
}
