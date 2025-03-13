use crate::utils::test;

mod utils;

use system_of_linear_equations::gaussian_elimination_with_backward_substitution::solve as gaussian_elimination_with_backward_substitution;
// use system_of_linear_equations::lu_factorization::solve as lu_factorization;

test!(
    id = 0000,
    algo = gaussian_elimination_with_backward_substitution,
    N = 4,
    A = [
        2.0, 1.0, 3.0, 4.0,
        1.0, 2.0, 4.0, 3.0,
        3.0, 4.0, 1.0, 2.0,
        4.0, 3.0, 2.0, 1.0,
    ],
    b = [
        20.0,
        22.0,
        21.0,
        20.0,
    ],
    x = [
        1.0,
        2.0,
        3.0,
        4.0,
    ],
);
