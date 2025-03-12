use system_of_linear_equations::gaussian_elimination_with_backward_substitution::solve as gaussian_elimination_with_backward_substitution;

use crate::utils::test;

test!(
    id = 0000,
    algo = gaussian_elimination_with_backward_substitution,
    N = 4,
    A = [
        1.0, 1.0, 0.0, 3.0,
        2.0, 1.0, -1.0, 1.0,
        3.0, -1.0, -1.0, 2.0,
        -1.0, 2.0, 3.0, -1.0,
    ],
    b = [
        1.0,
        1.0,
        -3.0,
        4.0,
    ],
);

test!(
    id = 0001,
    algo = gaussian_elimination_with_backward_substitution,
    N = 100,
    epsilon = 1e-9,
);

// Slow, consider skipping
test!(
    id = 0002,
    algo = gaussian_elimination_with_backward_substitution,
    N = 1000,
    epsilon = 1e-9,
);

// Very slow, consider skipping
test!(
    id = 0003,
    algo = gaussian_elimination_with_backward_substitution,
    N = 1_000_000,
    epsilon = 1e-9,
);
