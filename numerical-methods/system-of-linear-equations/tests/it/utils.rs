use std::ops::Range;

use system_of_linear_equations::Num;

macro_rules! test {
    // Specify id, multiple algos, N, epsilon, omit A, b
    (id = $id:expr, algos = [$($solver:ident,)+], N = $N:expr, epsilon = $epsilon:expr) => {
        test!(id = $id, algos = [$($solver,)+], N = $N, A = crate::utils::__random_vec::<{ $N * $N }>(crate::utils::__LOWER_BOUND..crate::utils::__UPPER_BOUND), b = crate::utils::__random_vec::<$N>(crate::utils::__LOWER_BOUND..crate::utils::__UPPER_BOUND), epsilon = $epsilon);
    };

    (id = $id:expr, algos = [$($solver:ident,)+], N = $N:expr, epsilon = $epsilon:expr,) => {
        test!(id = $id, algos = [$($solver,)+], N = $N, epsilon = $epsilon);
    };

    // Specify id, multiple algos, N, A, b, epsilon
    (id = $id:expr, algos = [$($solver:ident,)+], N = $N:expr, A = [$($a:tt)*], b = [$($b:tt)*], epsilon = $epsilon:expr) => {
        test!(id = $id, algos = [$($solver,)+], N = $N, A = vec![$($a)*], b = vec![$($b)*], epsilon = $epsilon);
    };

    (id = $id:expr, algos = [$($solver:ident,)+], N = $N:expr, A = [$($a:tt)*], b = [$($b:tt)*], epsilon = $epsilon:expr,) => {
        test!(id = $id, algos = [$($solver,)+], N = $N, A = [$($a)*], b = [$($b)*], epsilon = $epsilon);
    };

    (id = $id:expr, algos = [$($solver:ident,)+], N = $N:expr, A = $A:expr, b = $b:expr, epsilon = $epsilon:expr) => {
        $(
            test!(id = $id, algo = $solver, N = $N, A = $A, b = $b, epsilon = $epsilon);
        )+
    };

    // Specify id, algo, N, A, b, epsilon
    (id = $id:expr, algo = $solver:ident, N = $N:expr, A = [$($a:tt)*], b = [$($b:tt)*], epsilon = $epsilon:expr) => {
        test!(id = $id, algo = $solver, N = $N, A = vec![$($a)*], b = vec![$($b)*], epsilon = $epsilon);
    };

    (id = $id:expr, algo = $solver:ident, N = $N:expr, A = [$($a:tt)*], b = [$($b:tt)*], epsilon = $epsilon:expr,) => {
        test!(id = $id, algo = $solver, N = $N, A = [$($a)*], b = [$($b)*], epsilon = $epsilon);
    };

    // Specify id, algo, N, A, b, omit epsilon
    (id = $id:expr, algo = $solver:ident, N = $N:expr, A = [$($a:tt)*], b = [$($b:tt)*]) => {
        test!(id = $id, algo = $solver, N = $N, A = [$($a)*], b = [$($b)*], epsilon = Num::EPSILON);
    };

    (id = $id:expr, algo = $solver:ident, N = $N:expr, A = [$($a:tt)*], b = [$($b:tt)*],) => {
        test!(id = $id, algo = $solver, N = $N, A = [$($a)*], b = [$($b)*]);
    };

    // Specify id, algo, N, epsilon, omit A, b
    (id = $id:expr, algo = $solver:ident, N = $N:expr, epsilon = $epsilon:expr) => {
        test!(id = $id, algo = $solver, N = $N, A = crate::utils::__random_vec::<{ $N * $N }>(crate::utils::__LOWER_BOUND..crate::utils::__UPPER_BOUND), b = crate::utils::__random_vec::<$N>(crate::utils::__LOWER_BOUND..crate::utils::__UPPER_BOUND), epsilon = $epsilon);
    };

    (id = $id:expr, algo = $solver:ident, N = $N:expr, epsilon = $epsilon:expr,) => {
        test!(id = $id, algo = $solver, N = $N, epsilon = $epsilon);
    };

    // Specify id, algo, N, omit A, b, epsilon
    (id = $id:expr, algo = $solver:ident, N = $N:expr) => {
        test!(id = $id, algo = $solver, N = $N, A = crate::utils::__random_vec::<{ $N * $N }>(crate::utils::__LOWER_BOUND..crate::utils::__UPPER_BOUND), b = crate::utils::__random_vec::<$N>(crate::utils::__LOWER_BOUND..crate::utils::__UPPER_BOUND), epsilon = crate::utils::__EPISILON);
    };

    (id = $id:expr, algo = $solver:ident, N = $N:expr,) => {
        test!(id = $id, algo = $solver, N = $N);
    };

    (id = $id:expr, algo = $solver:ident, N = $N:expr, A = $A:expr, b = $b:expr, epsilon = $epsilon:expr) => {
        paste::paste! {
            #[test]
            fn [<test_$id _$solver _$N x$N>]() {
                use approx::relative_eq;
                use nalgebra::{DMatrix, DVector};
                use system_of_linear_equations::Num;

                let a = DMatrix::<Num>::from_vec($N, $N, $A);
                let b = DVector::<Num>::from_vec($b);

                let actual_x = $solver(&a, &b);
                let expected_x = a.lu().solve(&b);
            
                match (actual_x, expected_x) {
                    (None, None) => {},
                    (Some(actual_x), Some(expected_x)) => {
                        if !relative_eq!(actual_x, expected_x, epsilon = $epsilon) {
                            panic!(
                                "Solutions differ beyond precision tolerance ({:?}).\nExpected: {:?}\nFound: {:?}",
                                $epsilon, expected_x.as_slice(), actual_x.as_slice(),
                            );
                        }
                    },
                    (None, Some(expected_x)) => panic!(
                        "Expected a unique solution, found no solution.\nExpected: {:?}",
                        expected_x,
                    ),
                    (Some(actual_x), None) => panic!(
                        "Expected no solution, found a unique solution.\nFound: {:?}",
                        actual_x,
                    ),
                }
            }
        }
    };
}

pub(crate) use test;

pub const __EPISILON: Num = Num::EPSILON;
pub const __LOWER_BOUND: Num = -1e100;
pub const __UPPER_BOUND: Num = 1e100;

pub fn __random_vec<const N: usize>(bounds: Range<Num>) -> Vec<Num> {
    use rand::Rng;

    let mut rng = rand::rng();

    (0..N)
        .map(|_| rng.random_range(bounds.clone()))
        .collect()
}
