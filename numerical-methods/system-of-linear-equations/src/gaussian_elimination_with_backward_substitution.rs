use nalgebra::{SMatrix, SVector};

// R. L. Burden & J. D. Faires, CH06_1A Linear Systems of Equations.pdf (p. 40-43)
pub fn solve<const N: usize>(mut a: SMatrix<f64, N, N>, mut b: SVector<f64, N>) -> Option<SVector<f64, N>> {
    // Step 1: Forward elimination
    for i in 0..N {
        // Step 2: Find pivot row
        let mut p = i;
        while p < N && a[(p, i)] == 0.0 {
            p += 1;
        }
        if p == N {
            return None; // No unique solution
        }
        
        // Step 3: Swap rows if needed
        if p != i {
            a.swap_rows(i, p);
            b.swap_rows(i, p);
        }
        
        // Step 4: Eliminate column below pivot
        for j in i+1..N {
            let m_ji = a[(j, i)] / a[(i, i)];
            for k in i..N {
                a[(j, k)] -= m_ji * a[(i, k)];
            }
            b[j] -= m_ji * b[i];
        }
    }
    
    // Step 7: Check if system has a unique solution
    if a[(N-1, N-1)] == 0.0 {
        return None;
    }
    
    // Step 8-10: Back-substitution
    let mut x = SVector::<f64, N>::zeros();
    for i in (0..N).rev() {
        let sum = (i+1..N).map(|j| a[(i, j)] * x[j]).sum::<f64>();
        x[i] = (b[i] - sum) / a[(i, i)];
    }
    
    Some(x)
}
