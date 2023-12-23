// uses code from polynom.rs
// use crate::pol::Polynomial;
use crate::polynom::pol::Polynomial;


impl Polynomial {
    // self     -> the polynomial function coefficients
    // a        -> the lower limit of the interval
    // b        -> the upper limit of the interval
    // integral -> the approximated value of the integral
    pub fn simple_simpson(&self, a: f64, b: f64) -> f64 {

        let sum = self.eval(a) + self.eval(b) + 4.0 * self.eval((a + b) / 2.0);
        let integral: f64 = ((b - a) * (sum)) / 6.0;
        return integral;
    }
}