use system_of_linear_equations::gaussian_elimination_with_backward_substitution;

use crate::utils::test;

test!(gaussian_elimination_with_backward_substitution::solve, 4, [
    1.0, 1.0, 0.0, 3.0,
    2.0, 1.0, -1.0, 1.0,
    3.0, -1.0, -1.0, 2.0,
    -1.0, 2.0, 3.0, -1.0,
], [
    1.0,
    1.0,
    -3.0,
    4.0,
]);
