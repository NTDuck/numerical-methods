use nalgebra::{DMatrix, DVector};

use crate::Num;

// R. L. Burden & J. D. Faires, CH06_1A Linear Systems of Equations.pdf (p. 40-43)
pub fn solve(a: &DMatrix<Num>, b: &DVector<Num>) -> Option<DVector<Num>> {
    if !a.is_square() {
        return None;
    }

    let n = a.nrows();

    let mut a= a.clone_owned();
    let mut b = b.clone_owned();

    // Step 1: Forward elimination
    for i in 0..n {
        // Step 2: Find pivot row
        let mut p = i;
        while p < n && a[(p, i)] == 0.0 {
            p += 1;
        }
        if p == n {
            return None; // No unique solution
        }
        
        // Step 3: Swap rows if needed
        if p != i {
            a.swap_rows(i, p);
            b.swap_rows(i, p);
        }
        
        // Step 4: Eliminate column below pivot
        for j in i + 1..n {
            let m_ji = a[(j, i)] / a[(i, i)];
            for k in i..n {
                a[(j, k)] -= m_ji * a[(i, k)];
            }
            b[j] -= m_ji * b[i];
        }
    }
    
    // Step 7: Check if system has a unique solution
    if a[(n - 1, n - 1)] == 0.0 {
        return None;
    }
    
    // Step 8-10: Back-substitution
    let mut x = DVector::<Num>::zeros(n);
    for i in (0..n).rev() {
        let sum = (i + 1..n).map(|j| a[(i, j)] * x[j]).sum::<Num>();
        x[i] = (b[i] - sum) / a[(i, i)];
    }
    
    Some(x)
}
