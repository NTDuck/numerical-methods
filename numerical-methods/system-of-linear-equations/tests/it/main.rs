use crate::utils::test;

mod test_gaussian_eliminatin_with_backward_substitution;
mod test_lu_factorization;
mod utils;

use system_of_linear_equations::gaussian_elimination_with_backward_substitution::solve as gaussian_elimination_with_backward_substitution;
use system_of_linear_equations::lu_factorization::solve as lu_factorization;

// test!(
//     id = 0000,
//     algo = [
//         gaussian_elimination_with_backward_substitution,
//         lu_factorization,
//     ],
//     N = 4,
//     A = [
//         1.0, 1.0, 0.0, 3.0,
//         2.0, 1.0, -1.0, 1.0,
//         3.0, -1.0, -1.0, 2.0,
//         -1.0, 2.0, 3.0, -1.0,
//     ],
//     b = [
//         1.0,
//         1.0,
//         -3.0,
//         4.0,
//     ],
//     epsilon = Num::EPSILON,
// );

test!(
    id = 0001,
    algos = [
        gaussian_elimination_with_backward_substitution,
        lu_factorization,
    ],
    N = 100,
    epsilon = 1e-9,
);
