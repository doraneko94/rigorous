use rigorous::*;

fn main() {
    // x^2 + 10^15 * x + 10^14 = 0
    let a: Interval<f64> = Interval::from_str("1");
    let b: Interval<f64> = Interval::from_str("1000000000000000");
    let c: Interval<f64> = Interval::from_str("100000000000000");
    let f2: Interval<f64> = Interval::from_str("2");
    let f4: Interval<f64> = Interval::from_str("4");
    let fm1: Interval<f64> = Interval::from_str("-1");

    let x1 = ((b * b - f4 * a * c).sqrt() - b) / (f2 * a);
    let x2 = f2 * c / (fm1 * ((b * b - f4 * a * c).sqrt() + b));

    println!("{:?}", x1);
    println!("{:?}", x2);
}