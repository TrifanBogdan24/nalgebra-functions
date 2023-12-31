pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod integrals {
    pub mod integrate;
    pub mod simple_simpson;
    pub mod composed_simpson;
    pub mod simple_trapezoidal;
    pub mod composed_trapezoidal;    
}

pub mod polynom;

#[cfg(test)]
mod tests {
    use crate::add;      // polynom! is a macro

    use crate::polynom::Polynomial; // Polynomial is a struct
    use crate::polynom;             // the macro

    use approx::abs_diff_eq;        // since I compute integrals,
                                    // I am interested in comparing only the first 5 decimals

    #[test]
    fn it_works() {
        
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_polynomial_new_constructor() {
        let pol1: Polynomial = Polynomial::new(&[1.0, 2.0, 3.0, 4.0, 5.0]);

        if (pol1.coeff[0] != 1.0 || pol1.coeff[1] != 2.0 || pol1.coeff[2] != 3.0
            || pol1.coeff[3] != 4.0 || pol1.coeff[4] != 5.0) {
            assert!(false);
        }
        assert!(true);
    }

    #[test]
    fn test_polynomial_macro() {
        let pol1: Polynomial = polynom!(1.0, 2.0, 3.0, 4.0, 5.0);

        if (pol1.coeff[0] != 1.0 || pol1.coeff[1] != 2.0 || pol1.coeff[2] != 3.0
            || pol1.coeff[3] != 4.0 || pol1.coeff[4] != 5.0) {
            assert!(false);
        }
        assert!(true);
    }

    #[test]
    fn test_feval() {
    
        let pol2 = Polynomial::new(&[1.0, 2.0, 3.0, 4.0, 5.0]);
        let _po3 = polynom!(1.0, 2.0, 3.0, 4.0, 5.0);
        // 5 fixed decimals
        let precision = 1e-5;

        let feval_f_2: f64 =  pol2.eval(2.0);
        assert_eq!(abs_diff_eq!(feval_f_2, 57.00000, epsilon = precision), true);
    }


    #[test]
    fn test_actual_integral() {
        let result = add(2, 2);
        let _po3 = polynom!(1.0, 2.0, 3.0, 4.0, 5.0);

        assert_eq!(result, 4);

        let pol2 = Polynomial::new(&[1.0, 2.0, 3.0, 4.0, 5.0]);
        let polint = pol2.integrate();

        let a: f64 = 1.0;       // lower limit of the interval
        let b: f64 = 10.0;      // upper limit of the interval

        let actual_integral: f64=  polint.eval(b) - polint.eval(a);             // 26241.30000

        // 5 fixed decimals
        let precision = 1e-5;

        // unit tests
        assert_eq!(abs_diff_eq!(actual_integral, 26241.30000, epsilon = precision), true);
    }

    #[test]
    fn test_simple_simpson() {
    
        let pol2 = Polynomial::new(&[1.0, 2.0, 3.0, 4.0, 5.0]);
        let _po3 = polynom!(1.0, 2.0, 3.0, 4.0, 5.0);

        let a: f64 = 1.0;       // lower limit of the interval
        let b: f64 = 10.0;      // upper limit of the interval

        let simple_simpson: f64 = pol2.simple_simpson(a, b);                    // 26733.37500

        // 5 fixed decimals
        let precision = 1e-5;

        // unit tests
        assert_eq!(abs_diff_eq!(simple_simpson, 26733.37500, epsilon = precision), true);
    }

    #[test]
    fn test_composed_simpson() {

        let pol2 = Polynomial::new(&[1.0, 2.0, 3.0, 4.0, 5.0]);
        let _po3 = polynom!(1.0, 2.0, 3.0, 4.0, 5.0);


        let a: f64 = 1.0;       // lower limit of the interval
        let b: f64 = 10.0;      // upper limit of the interval
        let nr: usize = 125;    // the number of equidistant points

        let composed_simpson: f64     = pol2.composed_simpson(a, b, nr);        // 26242.34636

        // 5 fixed decimals
        let precision = 1e-5;

        // unit tests
        assert_eq!(abs_diff_eq!(composed_simpson, 26242.34636, epsilon = precision), true);
    }


    #[test]
    fn test_simple_trapezoidal() {
        let pol2: Polynomial = polynom!(1.0, 2.0, 3.0, 4.0, 5.0);

        let a: f64 = 1.0;       // lower limit of the interval
        let b: f64 = 10.0;      // upper limit of the interval

        let simple_trapezoidal: f64   = pol2.simple_trapezoidal(a, b);          // 27810.00000

        // 5 fixed decimals
        let precision = 1e-5;

        // unit tests
        assert_eq!(abs_diff_eq!(simple_trapezoidal, 27810.00000, epsilon = precision), true);
    }


    #[test]
    fn test_composed_trapezoidal() {
        let _pol: Polynomial = Polynomial {
            coeff: vec![1.0, 2.0, 3.0, 4.0, 5.0],
        };
    
        let pol2 = Polynomial::new(&[1.0, 2.0, 3.0, 4.0, 5.0]);
        let _polint = pol2.integrate();

        let a: f64 = 1.0;       // lower limit of the interval
        let b: f64 = 10.0;      // upper limit of the interval
        let nr: usize = 125;    // the number of equidistant points

        let composed_trapezoidal: f64 = pol2.composed_trapezoidal(a, b, nr);    // 26243.30620


        // 5 fixed decimals
        let precision = 1e-5;

        // unit tests
        assert_eq!(abs_diff_eq!(composed_trapezoidal, 26243.30620, epsilon = precision), true);
    }

    #[warn(dead_code)]
    #[warn(unused_imports)]
    #[warn(unused_variables)]
    fn print() {
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
        
        // stdout generated by this code will represent the unit tests
        println!("f(2) = {:.5}", pol2.eval(2.0 as f64));
        println!("actual integral      = {:.5}", (polint.eval(10.0) - polint.eval(1.0)));
        println!("simple simpson       = {:.5}", pol2.simple_simpson(1.0, 10.0));
        println!("composed simpson     = {:.5}", pol2.composed_simpson(1.0, 10.0, 125 as usize));
        println!("simple trapezoidal   = {:.5}", pol2.simple_trapezoidal(1.0, 10.0));
        println!("composed trapezoidal = {:.5}", pol2.composed_trapezoidal(1.0, 10.0, 125 as usize));    
    }
}
