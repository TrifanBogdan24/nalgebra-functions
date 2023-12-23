// what am I doing wrong? can't find crate?
// $ rustc test.rs # generates errors (where should I test the correct imports?)
extern crate nalgebra_functions;
use nalgebra_functions::polynom::*;
use nalgebra_functions::integrals::*;


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
    println!("f(2) = {:.3}", pol2.eval(2.0 as f64));

    println!("∫ f(X) dx = {}", polint.to_string());
    println!("actual integral      = {:.3}", (polint.eval(10.0) - polint.eval(1.0)));
    println!("simple simpson       = {:.3}", pol2.simple_simpson(1.0, 10.0));
    println!("composed simpson     = {:.3}", pol2.composed_simpson(1.0, 10.0, 125 as usize));
    println!("simple trapezoidal   = {:.3}", pol2.simple_trapezoidal(1.0, 10.0));
    println!("composed trapezoidal = {:.3}", pol2.composed_trapezoidal(1.0, 10.0, 125 as usize));

    return;
}
