# Added feature: Numerical Methods Integrals

## Library for calculating the approximated value of the polynomial function's integral

The data structure `Polynomial` has the following implemented methods:
- `new`
- `polynom!()` macro
- `to_string`
- `printfunc`
- `eval`
- `simple_simpson`
- `composed_simpson`
- `simple_trapezoidal`
- `composed_trapezoidal`


Check the code example from below:

```rust
use crate::polynom::Polynomial;    // the struct
use crate::polynom;                // the macro

fn main() {
    // how to create a polynomial instance? 
    
    // method 1 : manually
    let pol: Polynomial = Polynomial {
        coeff: vec![1.0, 2.0, 3.0, 4.0, 5.0],
    };

    // method 2 : copy constructor
    let pol2 = Polynomial::new(&[1.0, 2.0, 3.0, 4.0, 5.0]);
    
    // method 3: macro
    let pol3 = polynom!(1.0, 2.0, 3.0, 4.0, 5.0);
    
    let polint = pol2.integrate();
    
    print!("f(x) = ");
    pol.printfunc();
    println!();
    println!("f(x) = {}", pol2.to_string());

    println!("∫ f(X) dx = {}", polint.to_string());

    // relevant integral functions for a Polynomial instance
    println!("f(2) = {:.5}", pol2.eval(2.0 as f64));
    println!("actual integral      = {:.5}", (polint.eval(10.0) - polint.eval(1.0)));
    println!("simple simpson       = {:.5}", pol2.simple_simpson(1.0, 10.0));
    println!("composed simpson     = {:.5}", pol2.composed_simpson(1.0, 10.0, 125 as usize));
    println!("simple trapezoidal   = {:.5}", pol2.simple_trapezoidal(1.0, 10.0));
    println!("composed trapezoidal = {:.5}", pol2.composed_trapezoidal(1.0, 10.0, 125 as usize));

    return;
}
```
