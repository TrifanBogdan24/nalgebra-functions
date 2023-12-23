struct Polynomial {
    coeff: Vec<f64>,
}

impl Polynomial {
    // constructor for Polynomial
    fn new(coefficients: &[f64]) -> Self {
        return Polynomial {
            coeff: coefficients.to_vec(),
        };

    }
}

impl Polynomial {
    fn printfunc(&self) {
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
    fn to_string(&self) -> String {
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
    fn integrate(&self) -> Self {

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

impl Polynomial {
    fn eval(&self, x: f64) -> f64 {
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

impl Polynomial {
    fn simple_simpson(&self, a: f64, b: f64) -> f64 {
        // self     -> the polynomial function coefficients
        // a        -> the lower limit of the interval
        // b        -> the upper limit of the interval
        // integral -> the approximated value of the integral
        let sum = self.eval(a) + self.eval(b) + 4.0 * self.eval((a + b) / 2.0);
        let integral: f64 = ((b - a) * (sum)) / 6.0;
        return integral;
    }
}



impl Polynomial {
    fn composed_simpson(&self, a: f64, b: f64, n: usize) -> f64 {
        // self     -> the polynomial function coefficients
        // a        -> the lower limit of the interval
        // b        -> the upper limit of the interval
        // n        -> the number of equidistant points in the interval
        // integral -> the approximated value of the integral

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


impl Polynomial {
    fn simple_trapezoidal(&self, a: f64, b: f64) -> f64 {
        // self     -> the polynomial function coefficients
        // a        -> the lower limit of the interval
        // b        -> the upper limit of the interval
        // integral -> the approximated value of the integral

        let h: f64 = (b - a) / 2.0;
        let integral: f64 = h * (self.eval(a) + self.eval(b)) / 2.0;
        return integral;
    }
}


impl Polynomial {
    fn composed_trapezoidal(&self, a: f64, b: f64, n: usize) -> f64 {
        // self     -> the polynomial function coefficients
        // a        -> the lower limit of the interval
        // b        -> the upper limit of the interval
        // n        -> the number of equidistant points in the interval
        // integral -> the approximated value of the integral

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


fn main() {
    let pol: Polynomial = Polynomial {
        coeff: vec![1.0, 2.0, 3.0, 4.0, 5.0],
    };

    let pol2 = Polynomial::new(&[1.0, 2.0, 3.0, 4.0, 5.0]);
    let polint = pol2.integrate();
    
    print!("f(x) = ");
    pol.printfunc();
    println!();
    println!("f(x) = {}", pol2.to_string());

    println!("∫ f(X) dx = {}", polint.to_string());

    // unit testing starts here
    println!("f(2) = {:.5}", pol2.eval(2.0 as f64));
    println!("actual integral      = {:.5}", (polint.eval(10.0) - polint.eval(1.0)));
    println!("simple simpson       = {:.5}", pol2.simple_simpson(1.0, 10.0));
    println!("composed simpson     = {:.5}", pol2.composed_simpson(1.0, 10.0, 125 as usize));
    println!("simple trapezoidal   = {:.5}", pol2.simple_trapezoidal(1.0, 10.0));
    println!("composed trapezoidal = {:.5}", pol2.composed_trapezoidal(1.0, 10.0, 125 as usize));

    return;
}
