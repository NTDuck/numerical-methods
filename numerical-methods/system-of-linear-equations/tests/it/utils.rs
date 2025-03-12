use system_of_linear_equations::Num;

macro_rules! test {
    // Specify A, b, epsilon
    (id = $id:expr, algo = $solver:ident, N = $N:expr, A = [$($a:tt)*], b = [$($b:tt)*], epsilon = $epsilon:expr) => {
        test!(id = $id, algo = $solver, N = $N, A = vec![$($a)*], b = vec![$($b)*], epsilon = $epsilon);
    };

    (id = $id:expr, algo = $solver:ident, N = $N:expr, A = [$($a:tt)*], b = [$($b:tt)*], epsilon = $epsilon:expr,) => {
        test!(id = $id, algo = $solver, N = $N, A = [$($a)*], b = [$($b)*], epsilon = $epsilon);
    };

    // Specify A, b, omit epsilon
    (id = $id:expr, algo = $solver:ident, N = $N:expr, A = [$($a:tt)*], b = [$($b:tt)*]) => {
        test!(id = $id, algo = $solver, N = $N, A = [$($a)*], b = [$($b)*], epsilon = Num::EPSILON);
    };

    (id = $id:expr, algo = $solver:ident, N = $N:expr, A = [$($a:tt)*], b = [$($b:tt)*],) => {
        test!(id = $id, algo = $solver, N = $N, A = [$($a)*], b = [$($b)*]);
    };

    // Specify epsilon, omit A, b
    (id = $id:expr, algo = $solver:ident, N = $N:expr, epsilon = $epsilon:expr) => {
        test!(id = $id, algo = $solver, N = $N, A = crate::utils::__random_vec::<{ $N * $N }>(), b = crate::utils::__random_vec::<$N>(), epsilon = $epsilon);
    };

    (id = $id:expr, algo = $solver:ident, N = $N:expr, epsilon = $epsilon:expr,) => {
        test!(id = $id, algo = $solver, N = $N, epsilon = $epsilon);
    };

    // Omit A, b, epsilon
    (id = $id:expr, algo = $solver:ident, N = $N:expr) => {
        test!(id = $id, algo = $solver, N = $N, A = crate::utils::__random_vec::<{ $N * $N }>(), b = crate::utils::__random_vec::<$N>(), epsilon = Num::EPSILON);
    };

    (id = $id:expr, algo = $solver:ident, N = $N:expr,) => {
        test!(id = $id, algo = $solver, N = $N);
    };

    // Base case
    (id = $id:expr, algo = $solver:ident, N = $N:expr, A = $A:expr, b = $b:expr, epsilon = $epsilon:expr) => {
        paste::paste! {
            #[test]
            fn [<test_$id _$solver _$N x$N>]() {
                use approx::relative_eq;
                use nalgebra::{SMatrix, SVector};
                use system_of_linear_equations::Num;

                #[cfg(test)] 
                use pretty_assertions::assert_eq;

                // Transpose because SMatrix::from_vec fills column-by-column
                let a = SMatrix::<Num, $N, $N>::from_vec($A);
                let b = SVector::<Num, $N>::from_vec($b);

                let actual_x = $solver(a, b);
                let expected_x = a.lu().solve(&b);
            
                match (actual_x, expected_x) {
                    (None, None) => {},
                    (Some(actual_x), Some(expected_x)) => {
                        if !relative_eq!(actual_x, expected_x, epsilon = $epsilon) {
                            assert_eq!(
                                actual_x.as_slice(), expected_x.as_slice(),
                                "Solutions differ beyond precision tolerance",
                            );
                        }
                    },
                    (None, Some(_)) => panic!(
                        "Expected a unique solution, found no solution.\nExpected: {:?}",
                        expected_x,
                    ),
                    (Some(_), None) => panic!(
                        "Expected no solution, found a unique solution.\nFound: {:?}",
                        actual_x,
                    ),
                }
            }
        }
    };
}

pub(crate) use test;

pub fn __random_vec<const N: usize>() -> Vec<Num> {
    use rand::Rng;

    const LOWER_BOUND: Num = -10.0;
    const UPPER_BOUND: Num = 10.0;

    let mut rng = rand::rng();

    (0..N)
        .map(|_| rng.random_range(LOWER_BOUND..UPPER_BOUND))
        .collect()
}
