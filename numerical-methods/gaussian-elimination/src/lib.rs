use nalgebra::{DMatrix, DVector};

pub fn solve(a: &DMatrix<f64>, b: &DVector<f64>) -> DVector<f64> {
    let n = a.nrows();
    let mut ab = a.clone().insert_column(n, 0.0); // Augmented matrix [A | b]
    
    // Copy b into the last column
    ab.column_mut(n).copy_from(b);

    // Forward elimination with partial pivoting
    for i in 0..n {
        // Pivot: Find the row with the largest element in column i
        let max_row = (i..n).max_by(|&r1, &r2| ab[(r1, i)].abs().partial_cmp(&ab[(r2, i)].abs()).unwrap()).unwrap();
        ab.swap_rows(i, max_row);

        // Make elements below the pivot zero
        for j in (i + 1)..n {
            let factor = ab[(j, i)] / ab[(i, i)];
            for k in i..=n { // Include last column
                ab[(j, k)] -= factor * ab[(i, k)];
            }
        }
    }

    // Back-substitution
    let mut x = DVector::<f64>::zeros(n);
    for i in (0..n).rev() {
        let sum = (i + 1..n).map(|j| ab[(i, j)] * x[j]).sum::<f64>();
        x[i] = (ab[(i, n)] - sum) / ab[(i, i)];
    }

    x
}
