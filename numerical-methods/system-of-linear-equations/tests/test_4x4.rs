use system_of_linear_equations::gaussian_elimination_with_backward_substitution;

// Verified by: https://matrixcalc.org/slu.html#solve-using-Gaussian-elimination%28%7B%7B1,1,0,3,1%7D,%7B2,1,-1,1,1%7D,%7B3,-1,-1,2,-3%7D,%7B-1,2,3,-1,4%7D%7D%29
#[test]
fn test() {
    use approx::assert_relative_eq;
    use nalgebra::{SMatrix, SVector};
    use system_of_linear_equations::Num;

    const N: usize = 4;
    let a = SMatrix::<Num, N, N>::from_vec(vec![
        1.0, 1.0, 0.0, 3.0,
        2.0, 1.0, -1.0, 1.0,
        3.0, -1.0, -1.0, 2.0,
        -1.0, 2.0, 3.0, -1.0,
    ]); // Transpose because SMatrix::from_vec fills column-by-column
    let b = SVector::<Num, N>::from_vec(vec![
        1.0,
        1.0,
        -3.0,
        4.0,
    ]);
    let x = gaussian_elimination_with_backward_substitution::solve(a, b);
    let expected_x = a.lu().solve(&b);

    match (x, expected_x) {
        (None, None) => {},
        (Some(x), Some(expected_x)) => {
            assert_relative_eq!(x, expected_x, epsilon = Num::EPSILON);
        },
        (None, Some(_)) => panic!("Expected a unique solution, found no solution"),
        (Some(_), None) => panic!("Expected no solution, found a unique solution"),
    }
}
