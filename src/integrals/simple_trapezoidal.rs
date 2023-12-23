// uses code from polynom.rs
// use crate::pol::Polynomial;
use crate::polynom::pol::Polynomial;

impl Polynomial {
    // self     -> the polynomial function coefficients
    // a        -> the lower limit of the interval
    // b        -> the upper limit of the interval
    // integral -> the approximated value of the integral
    pub fn simple_trapezoidal(&self, a: f64, b: f64) -> f64 {

        let h: f64 = (b - a) / 2.0;
        let integral: f64 = h * (self.eval(a) + self.eval(b)) / 2.0;
        return integral;
    }
}
