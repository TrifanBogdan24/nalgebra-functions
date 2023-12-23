// uses code from polynom.rs
use crate::polynom::pol::Polynomial;

impl Polynomial {
    // self     -> the polynomial function coefficients
    // Self     -> the returned polynomial function
    pub fn integrate(&self) -> Self {

        let mut coeff: Vec<f64> = Vec::new();
        let n: usize = self.coeff.len();

        for i in 0..=(n - 1) {
            let val: f64 = self.coeff[i] / ((n - i) as f64);
            coeff.push(val);
        }

        coeff.push(0.0);

        return Polynomial::new(&coeff);
    }
}
