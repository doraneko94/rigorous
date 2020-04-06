use rigorous::*;
use num_traits::float::Float;

fn main() {
    let pi = "3.14159265358979";
    let i: Interval<f64> = Interval::from_str(pi);
    println!("pi = {}", pi);
    println!("i = [inf, sup]");
    println!("inf = {:.25}", i.inf);
    println!("sup = {:.25}\n", i.sup);

    let pi = "3.141592653589793";
    let i: Interval<f64> = Interval::from_str(pi);
    println!("pi = {}", pi);
    println!("i = [inf, sup]");
    println!("inf = {:.25}", i.inf);
    println!("sup = {:.25}\n", i.sup);

    let pi = "3.1415926535897932";
    let i: Interval<f64> = Interval::from_str(pi);
    println!("pi = {}", pi);
    println!("i = [inf, sup]");
    println!("inf = {:.25}", i.inf);
    println!("sup = {:.25}\n", i.sup);

    let pi = "3.14159265358979323";
    let i: Interval<f64> = Interval::from_str(pi);
    println!("pi = {}", pi);
    println!("i = [inf, sup]");
    println!("inf = {:.25}", i.inf);
    println!("sup = {:.25}\n", i.sup);
    
    let pi = "3.141592653589793238";
    let i: Interval<f64> = Interval::from_str(pi);
    println!("pi = {}", pi);
    println!("i = [inf, sup]");
    println!("inf = {:.25}", i.inf);
    println!("sup = {:.25}\n", i.sup);

    let i: Interval<f64> = Interval::pi();
    println!("i = Interval::pi() = [inf, sup]");
    println!("inf = {:.25}", i.inf);
    println!("sup = {:.25}\n", i.sup);

    let e64: f64 = Float::epsilon();
    println!("e64 = {}", e64);
}