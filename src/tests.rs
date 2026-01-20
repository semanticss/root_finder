#[cfg(test)]
mod tests {
    use crate::polynomial::*;
    use crate::roots::*;
    use nalgebra::*;

    #[test]
    pub fn test_vectors() {
        let v = vector![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let y = v.data;
        let x = v.get(1).unwrap();
        println!("{}", v);
        println!("{}", x);
    }

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

    #[test]
    fn matrix_testing() {
        let m = DMatrix::from_element(5, 5, 0.0);
        println!("{}", m);
    }

    #[test]
    fn test_polynomial_roots() {
        let p1 = vec![5.0, 2., 1., 1., 1., 7.];
        let p2 = vec![5.0, 2., 1., 1., 1., 7.];
        let roots = companion(p1);
        let p2_as_dvector = DVector::from_vec(p2);
        println!("{}", p2_as_dvector);
        for (i, root) in roots.iter().enumerate() {
            println!("Root {}: {} + {}i", i, root.0, root.1);
        }
    }
}
