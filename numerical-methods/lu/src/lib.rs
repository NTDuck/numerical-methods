use nalgebra::{DMatrix, DVector};

pub fn solve(a: &DMatrix<f64>, b: &DVector<f64>) -> DVector<f64> {
    let (l, u) = lu_decompose(a);
    let y = forward_substitution(&l, b);
    let x = backward_substitution(&u, &y);

    x
}

fn lu_decompose(a: &DMatrix<f64>) -> (DMatrix<f64>, DMatrix<f64>) {
    assert!(a.is_square());
    let n = a.nrows();

    let mut l = DMatrix::<f64>::identity(n, n);
    let mut u = a.clone();

    for i in 0..n {
        for j in i + 1..n {
            l[(j, i)] = u[(j, i)] / u[(i, i)];
            
            for k in i..n {
                u[(j, k)] -= l[(j, i)] * u[(i, k)];
            }
        }
    }

    (l, u)
}

fn forward_substitution(l: &DMatrix<f64>, b: &DVector<f64>) -> DVector<f64> {
    assert!(l.is_square());
    let n = l.nrows();

    let mut y = DVector::<f64>::zeros(n);

    for i in 0..n {
        let sum = (0..i).map(|j| l[(i, j)] * y[j]).sum::<f64>();
        y[i] = (b[i] - sum) / l[(i, i)];
    }

    y
}

fn backward_substitution(u: &DMatrix<f64>, y: &DVector<f64>) -> DVector<f64> {
    assert!(u.is_square());
    let n = u.nrows();

    let mut x = DVector::<f64>::zeros(n);

    for i in (0..n).rev() {
        let sum = ((i + 1)..n).map(|j| u[(i, j)] * x[j]).sum::<f64>();
        x[i] = (y[i] - sum) / u[(i, i)];
    }

    x
}
