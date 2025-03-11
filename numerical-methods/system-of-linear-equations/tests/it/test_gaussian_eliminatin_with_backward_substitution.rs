use system_of_linear_equations::gaussian_elimination_with_backward_substitution::solve as gaussian_elimination_with_backward_substitution;

use crate::utils::test;

test!(
    id = 0,
    algo = gaussian_elimination_with_backward_substitution,
    N = 4,
    a = [
        1.0, 1.0, 0.0, 3.0,
        2.0, 1.0, -1.0, 1.0,
        3.0, -1.0, -1.0, 2.0,
        -1.0, 2.0, 3.0, -1.0,
    ], b = [
        1.0,
        1.0,
        -3.0,
        4.0,
    ]);
