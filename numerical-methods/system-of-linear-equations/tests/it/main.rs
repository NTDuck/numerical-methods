use crate::utils::test;

mod utils;

use system_of_linear_equations::lu_factorization::solve as lu_factorization;

test!(
    id = 0000,
    algo = lu_factorization,
    N = 4,
    A = [
        2.0, 1.0, 3.0, 4.0,
        1.0, 2.0, 4.0, 3.0,
        3.0, 4.0, 1.0, 2.0,
        4.0, 3.0, 2.0, 1.0,
    ],
    b = [
        29.0,
        29.0,
        22.0,
        20.0,
    ],
    x = [
        1.0,
        2.0,
        3.0,
        4.0,
    ],
    epsilon = 1e-14,
);

test!(
    id = 0001,
    algo = lu_factorization,
    N = 4,
    A = [
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 12.0, 11.0,
        13.0, 15.0, 14.0, 16.0,
    ],
    b = [
        1030.0,
        2670.0,
        4309.0,
        5949.0,
    ],
    x = [
        101.0,
        102.0,
        103.0,
        104.0,
    ],
    epsilon = 0.0,
);

test!(
    id = 0002,
    algo = lu_factorization,
    N = 4,
    A = [
        2e+300, 3e+300, 5e+300, 7e+300,
        11e+300, 13e+300, 17e+300, 19e+300,
        23e+300, 29e+300, 31e+300, 37e+300,
        41e+300, 43e+300, 47e+300, 53e+300,
    ],
    b = [
        59e+300,
        61e+300,
        67e+300,
        71e+300,
    ],
    x = [
        126.0 / 55.0,
        -1024.0 / 55.0,
        -689.0 / 110.0,
        445.0 / 22.0,
    ],
    epsilon = 1e-13,
);
