use std::ops::Range;

use system_of_linear_equations::Num;

macro_rules! test {
    (
        id = $id:expr, algo = $solver:ident,
        N = $N:expr, A = [$($a_num:tt)*], b = [$($b_num:tt)*],
    ) => {
        test!(
            id = $id, algo = $solver,
            N = $N, A = [$($a_num)*], b = [$($b_num)*],
            epsilon = crate::utils::__EPSILON,
        );
    };

    (
        id = $id:expr, algo = $solver:ident,
        N = $N:expr, A = [$($a_num:tt)*], b = [$($b_num:tt)*],
        epsilon = $epsilon:expr,
    ) => {
        test!(
            id = $id, algo = $solver,
            N = $N, A = vec![$($a_num)*], b = vec![$($b_num)*],
            epsilon = $epsilon,
        );
    };

    (
        id = $id:expr, algo = $solver:ident,
        N = $N:expr,
    ) => {
        tests!(
            id = $id, algo = $solver,
            N = $N, bounds = (crate::utils::__LOWER_BOUND..crate::utils::__UPPER_BOUND),
            epsilon = crate::utils::__EPSILON,
        );
    };

    (
        id = $id:expr, algo = $solver:ident,
        N = $N:expr,
        epsilon = $epsilon:expr,
    ) => {
        tests!(
            id = $id, algo = $solver,
            N = $N, bounds = (crate::utils::__LOWER_BOUND..crate::utils::__UPPER_BOUND),
            epsilon = $epsilon,
        );
    };

    (
        id = $id:expr, algo = $solver:ident,
        N = $N:expr, bounds = ($lower_bound:tt..$upper_bound:tt),
    ) => {
        test!(
            id = $id, algo = $solver,
            N = $N, bounds = ($lower_bound..$upper_bound),
            epsilon = crate::utils::__EPSILON,
        );
    };

    (
        id = $id:expr, algo = $solver:ident,
        N = $N:expr, bounds = ($lower_bound:tt..$upper_bound:tt),
        epsilon = $epsilon:expr,
    ) => {{
        let a_vec = crate::utils::__random_vec::<{ $N * $N }>($lower_bound..$upper_bound);
        let b_vec = crate::utils::__random_vec::<$N>($lower_bound..$upper_bound);

        test!(
            id = $id, algo = $solver,
            N = $N, A = a_vec, b = b_vec,
            epsilon = $epsilon,
        );
    }};

    (
        id = $id:expr, algo = $solver:ident,
        N = $N:expr, A = $a_vec:expr, b = $b_vec:expr,
        epsilon = $epsilon:expr,
    ) => {
        test!(
            id = $id, algo = $solver,
            N = $N, A = $a_vec, b = $b_vec, x = {
                let a = DMatrix::<Num>::from_vec($N, $N, $a_vec).transpose();
                let b = DVector::<Num>::from_vec($b_vec);
                a.svd(true, true).solve(&b, $epsilon).ok()
            },
            epsilon = $epsilon,
        );
    };

    (
        id = $id:expr, algo = $solver:ident,
        N = $N:expr, A = [$($a_num:tt)*], b = [$($b_num:tt)*], x = [$($x_num:tt)*],
    ) => {
        test!(
            id = $id, algo = $solver,
            N = $N, A = [$($a_num)*], b = [$($b_num)*], x = [$($x_num)*],
            epsilon = crate::utils::__EPSILON,
        );
    };

    (
        id = $id:expr, algo = $solver:ident,
        N = $N:expr, A = [$($a_num:tt)*], b = [$($b_num:tt)*], x = [$($x_num:tt)*],
        epsilon = $epsilon:expr,
    ) => {
        test!(
            id = $id, algo = $solver,
            N = $N, A = vec![$($a_num)*], b = vec![$($b_num)*], x =
                Some(DVector::<Num>::from_vec(vec![$($x_num)*])),
            epsilon = $epsilon,
        );
    };

    (
        id = $id:expr, algo = $solver:ident,
        N = $N:expr, A = $a_vec:expr, b = $b_vec:expr, x = $x:expr,
        epsilon = $epsilon:expr,
    ) => {
        paste::paste! {
            #[test]
            fn [<test_$id _$solver:snake _$N x$N>]() {
                use approx::relative_eq;
                use nalgebra::{DMatrix, DVector};
                use system_of_linear_equations::Num;

                let a = DMatrix::<Num>::from_vec($N, $N, $a_vec).transpose();
                let b = DVector::<Num>::from_vec($b_vec);

                let actual_x = $solver(&a, &b);
                let expected_x = $x;
            
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

pub const __EPSILON: Num = Num::EPSILON;
pub const __LOWER_BOUND: Num = -1e100;
pub const __UPPER_BOUND: Num = 1e100;

pub fn __random_vec<const N: usize>(bounds: Range<Num>) -> Vec<Num> {
    use rand::Rng;

    let mut rng = rand::rng();

    (0..N)
        .map(|_| rng.random_range(bounds.clone()))
        .collect()
}
