// uses code from polynom.rs
use crate::polynom::Polynomial;

impl Polynomial {
    // self     -> the polynomial function coefficients
    // a        -> the lower limit of the interval
    // b        -> the upper limit of the interval
    // n        -> the number of equidistant points in the interval
    // integral -> the approximated value of the integral
    pub fn composed_trapezoidal(&self, a: f64, b: f64, n: usize) -> f64 {

        let h: f64 = (b - a) / (n as f64);
        let mut s: f64 = 0.0;
        
        for i in 1..=(n - 1) {
            s = s + self.eval(a + (i as f64) * h);
        }

        let sum: f64 = self.eval(a) + self.eval(b) + 2.0 * s;
        let integral: f64 = h * sum / 2.0;
        return integral;
    }
}
