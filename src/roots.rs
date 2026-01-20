use crate::polynomial::*;
use nalgebra::{Complex, DMatrix, DVector};
use pyo3::*;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use std::fmt;

pub fn newton_raphson(
    starting_guess: f64,
    iters: usize,
    tolerance: f64,
    p: &Polynomial<f64>,
) -> Option<(f64, bool)> {
    // the boolean means whether or not it was under the tolerance
    let mut root = starting_guess;
    for _ in 0..iters {
        let f_x = (p.function)(root);
        let df_x = (p.derivative)(root);

        if df_x.abs() < 1e-12 {
            println!("derivative got too small!!");
            return None;
        }

        root = root - (f_x / df_x);
        if f_x.abs() < tolerance {
            return Some((root, true));
        }
    }

    Some((root, false))
}

#[pyfunction]
pub fn companion(coefficients: Vec<f64>) -> Vec<(f64, f64)> {
    // you can just pass a vector with the coefficients through here pretty electric
    let n = coefficients.len() - 1;
    let leading_coefficient = coefficients[n];

    let mut normal_coeffs = vec![];
    for i in 0..n - 1 {
        normal_coeffs.push(coefficients[i] / leading_coefficient);
    }

    let mut companion_matrix = DMatrix::from_element(n, n, 0.0);
    for i in 1..n {
        companion_matrix[(i, i - 1)] = 1.0;
    }
    for i in 0..n - 1 {
        companion_matrix[(i, n - 1)] = -normal_coeffs[i];
    }

    let roots = companion_matrix.complex_eigenvalues();

    let result = roots.iter().map(|c| (c.re, c.im)).collect();
    result
}

#[pyfunction]
pub fn batching_companion(flat_coeffs: Vec<f64>, degree: usize) -> (Vec<f64>, Vec<f64>) {
    let chunk_size = degree + 1;
    let results: Vec<(f64, f64)> = flat_coeffs
        .par_chunks(chunk_size)
        .flat_map(|cfs| {
            let leading = cfs[degree];
            let mut m = DMatrix::zeros(degree, degree);

            for i in 1..degree {
                m[(i, i - 1)] = 1.0;
            }
            for i in 0..degree {
                m[(i, degree - 1)] = -cfs[i] / leading;
            }

            let roots = m.complex_eigenvalues();
            let mut owned = Vec::with_capacity(degree);
            for r in roots.iter() {
                owned.push((r.re, r.im));
            }
            owned
        })
        .collect();

    let mut re = Vec::with_capacity(results.len());
    let mut im = Vec::with_capacity(results.len());
    for (r, i) in results {
        re.push(r);
        im.push(i);
    }
    (re, im)
}
