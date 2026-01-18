use std::fmt;

pub struct Polynomial<T> {
    pub coeffs: Vec<T>,
    pub function: Box<dyn Fn(T) -> T>,
    pub derivative: Box<dyn Fn(T) -> T>,
}

impl Polynomial<f64> {
    pub fn new(cs: Vec<f64>) -> Self {
        let fcs = cs.clone();

        let dcs: Vec<f64> = cs
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, &c)| c * (i as f64))
            .collect();

        let function = Box::new(move |x: f64| {
            fcs.iter()
                .enumerate()
                .map(|(i, &c)| c as f64 * x.powf(i as f64))
                .sum()
        });

        let derivative = Box::new(move |x: f64| {
            dcs.iter()
                .enumerate()
                .map(|(i, &c)| c as f64 * x.powf(i as f64))
                .sum()
        });

        Polynomial {
            coeffs: cs,
            function,
            derivative,
        }
    }
}

impl fmt::Display for Polynomial<f64> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = Vec::new();

        for (i, coeff) in self.coeffs.iter().enumerate().rev() {
            if *coeff == 0.0 {
                continue;
            }
            let next_term = match i {
                0 => format!("{}", coeff),
                1 => format!("{}x", coeff),
                _ => format!("{}x^{}", coeff, i),
            };
            result.push(next_term);
        }
        write!(f, "{}", result.join(" + "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_derivative() {
        let p1 = Polynomial::new(vec![45., 20., 4.]);
        println!("{}", p1);
        println!("{}", (p1.derivative)(1.));
        assert_eq!((p1.derivative)(1.), 28.);
    }
}
