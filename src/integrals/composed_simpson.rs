// uses code from polynom.rs
// use crate::pol::Polynomial;// e mai ok cu super?
use crate::polynom::pol::Polynomial;

impl Polynomial {
    // self     -> the polynomial function coefficients
    // a        -> the lower limit of the interval
    // b        -> the upper limit of the interval
    // n        -> the number of equidistant points in the interval
    // integral -> the approximated value of the integral
    pub fn composed_simpson(&self, a: f64, b: f64, n: usize) -> f64 {


        let h: f64 = (b - a) / (2.0 * (n as f64));
        
        let mut s1: f64 = 0.0;
        let mut s2: f64 = 0.0;

        for i in 0..=n {
            let x: f64 = a + ((i as f64) * 2.0 - 1.0) *h;
            s1 = s1 + self.eval(x);
        }

        for i in 0..=(n - 1) {
            let x: f64 = a + (i as f64) * 2.0 * h;
            s2 = s2 + self.eval(x);
        }

        let sum: f64 = self.eval(a) + self.eval(b) + 4.0 * s1 + 2.0 * s2;
        let integral: f64 = (h * sum) / 3.0;
        return integral;
    }

}
