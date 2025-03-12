use system_of_linear_equations::lu_factorization::solve as lu_factorization;

use crate::utils::test;

test!(
    id = 0000,
    algo = lu_factorization,
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
    algo = lu_factorization,
    N = 100,
    epsilon = 1e-9,
);

test!(
    id = 0002,
    algo = lu_factorization,
    N = 200,
    epsilon = 1e-6,
);
