use crate::polynomial::*;
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

#[cfg(test)]
mod tests {
    use crate::polynomial;

    use super::*;

    #[test]
    fn p_sanity() {
        let polynomial = Polynomial::new(vec![0., 2., 3.]);
        println!("{}", polynomial);
        println!("{}", (polynomial.function)(0.));
    }

    #[test]
    fn calculate_root_via_newton_raphson() {
        let polynomial = Polynomial::new(vec![0., 2., 3.]);

        let root = newton_raphson(-2., 1000, 0.01, &polynomial).unwrap();
        println!("root: {} under tolerance: {} ", root.0, root.1);
    }
}
