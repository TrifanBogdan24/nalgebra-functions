pub struct Polynomial {
    pub coeff: Vec<f64>,
}

impl Polynomial {
    // constructor for Polynomial
    pub fn new(coefficients: &[f64]) -> Self {
        return Polynomial {
            coeff: coefficients.to_vec(),
        };

    }
}

impl Polynomial {
    pub fn printfunc(&self) {
        let n: usize = self.coeff.len();

        for i in 0..=(n - 1) {

            if self.coeff[i] == 0.0 {
                continue;
            }

            let power: usize = n - i - 1;

            if self.coeff[i] < 0.0 {
                print!("-");
            } else {
                print!("+");
            } 

            print!(" ");

            if self.coeff[i] != -1.0 && self.coeff[i] != 1.0 {
                print!("{}", self.coeff[i]);
                if power != 0 {
                    print!("*");
                }
            }

            if power == 1 {
                print!("X");
            } else if power != 0 {
                print!("X^{}", power);
            }

            print!(" ");
        }

    }

}

impl Polynomial {
    pub fn to_string(&self) -> String {
        let mut res = String::from("");
        let n: usize = self.coeff.len();

        for i in 0..=(n - 1) {

            if self.coeff[i] == 0.0 {
                continue;
            }

            let power: usize = n - i - 1;

            if self.coeff[i] < 0.0 {
                res.push_str(&format!("-"));
            } else {
                res.push_str(&format!("+"));
            } 

            res.push_str(&format!(" "));

            if self.coeff[i] != -1.0 && self.coeff[i] != 1.0 {
                res.push_str(&format!("{}", self.coeff[i]));
                if power != 0 {
                    res.push_str(&format!("*"));
                }
            }

            if power == 1 {
                res.push_str(&format!("X"));
            } else if power != 0 {
                res.push_str(&format!("X^{}", power));
            }

            res.push_str(&format!(" "));
        }


        return res;
    }

}


impl Polynomial {
    pub fn eval(&self, x: f64) -> f64 {
        // &self -> a vector of the coefficients of the polynomial functio
        // x     -> the value for which the polynomial function is evalauated
        // f(x) = ?

        let mut y: f64 = 0.0;
        let n: usize = self.coeff.len();

        for i in 0..=(n - 1) {
            let power: f64 = (n - i - 1) as f64;
            y = y + self.coeff[i] * f64::powf(x, power);
        }
    
        return y;
    }
}
