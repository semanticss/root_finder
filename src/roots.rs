use crate::polynomial::*;
use nalgebra::{Complex, DMatrix, DVector};
use pyo3::*;
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
