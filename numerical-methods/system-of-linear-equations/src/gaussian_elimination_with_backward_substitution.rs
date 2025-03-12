use nalgebra::{DMatrix, DVector};

use crate::Num;

/// Algorithm: [R. L. Burden & J. D. Faires, CH06_1A Linear Systems of Equations.pdf (p. 40-43)](https://www.math.hkust.edu.hk/~mamu/courses/231/Slides/CH06_1A.pdf)
pub fn solve(a: &DMatrix<Num>, b: &DVector<Num>) -> Option<DVector<Num>> {
    assert!(a.is_square());
    let n = a.nrows();

    let mut a= a.clone_owned();
    let mut b = b.clone_owned();

    // Step 1 (Forward Elimination)
    for i in 0..n {
        // Step 2
        let mut p = i;
        while p < n && a[(p, i)] == 0.0 {
            p += 1;
        }
        if p == n {
            return None;
        }
        
        // Step 3
        if p != i {
            a.swap_rows(i, p);
            b.swap_rows(i, p);
        }
        
        // Step 4
        for j in i + 1..n {
            // Step 5
            let m_ji = a[(j, i)] / a[(i, i)];

            // Step 6
            for k in i..n {
                a[(j, k)] -= m_ji * a[(i, k)];
            }
            b[j] -= m_ji * b[i];
        }
    }
    
    // Step 7
    if a[(n - 1, n - 1)] == 0.0 {
        return None;
    }
    
    // Step 8-9 (Backward Substitution)
    let mut x = DVector::<Num>::zeros(n);
    for i in (0..n).rev() {
        let sum = (i + 1..n)
            .map(|j| a[(i, j)] * x[j])
            .sum::<Num>();
        x[i] = (b[i] - sum) / a[(i, i)];
    }
    
    // Step 10
    Some(x)
}
