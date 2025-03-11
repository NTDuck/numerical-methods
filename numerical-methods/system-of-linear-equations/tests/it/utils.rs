macro_rules! test {
    ($solver:path, $N:expr, [$($a:tt)*], [$($b:tt)*]) => {
        use paste::paste;

        paste! {
            #[test]
            fn [<test_$N x$N>]() {
                use approx::relative_eq;
                use nalgebra::{SMatrix, SVector};
                use system_of_linear_equations::Num;

                const N: usize = $N;

                // Transpose because SMatrix::from_vec fills column-by-column
                let a = SMatrix::<Num, N, N>::from_vec(vec![$($a)*]);
                let b = SVector::<Num, N>::from_vec(vec![$($b)*]);

                let actual_x = $solver(a, b);
                let expected_x = a.lu().solve(&b);
            
                match (actual_x, expected_x) {
                    (None, None) => {},
                    (Some(actual_x), Some(expected_x)) => {
                        assert!(relative_eq!(actual_x, expected_x, epsilon = Num::EPSILON),
                            "Solutions differ beyond precision tolerance");
                    },
                    (None, Some(_)) => panic!("Expected a unique solution, found no solution"),
                    (Some(_), None) => panic!("Expected no solution, found a unique solution"),
                }
            }
        }
    };
}

pub(crate) use test;
