use rigorous::*;

fn main() {
    // f(a, b) = 333.75 * b^6 + a^2 * (11 * a^2 * b^2 - b^6 - 121 * b^4 - 2) + 5.5 * b^8 + a / (2 * b)
    // (a, b) = (77617, 33096)
    let a: Interval<f64> = Interval::from_str("77617");
    let b: Interval<f64> = Interval::from_str("33096");
    let f333_75: Interval<f64> = Interval::from_str("333.75");
    let f11: Interval<f64> = Interval::from_str("11");
    let f121: Interval<f64> = Interval::from_str("121");
    let f2: Interval<f64> = Interval::from_str("2");
    let f5_5: Interval<f64> = Interval::from_str("5.5");

    let f = f333_75 * b * b * b * b * b * b
            + a * a * (f11 * a * a * b * b
                       - b * b * b * b * b * b
                       - f121 * b * b * b * b - f2)
            + f5_5 * b * b * b * b * b * b * b * b
            + a / (f2 * b);
    println!("=== interval type1 ===");
    println!("{:?}", f);

    let ff = (f333_75 - a * a) * b * b * b * b * b * b
            + a * a * (f11 * a * a * b * b
                       - f121 * b * b * b * b - f2)
            + f5_5 * b * b * b * b * b * b * b * b
            + a / (f2 * b);
    println!("=== interval type2 ===");
    println!("{:?}\n", ff);

    let x: f64 = 77617.0;
    let y: f64 = 33096.0;
    let fff = 333.75 * y * y * y * y * y * y
            + x * x * (11.0 * x * x * y * y 
                        - y * y * y * y * y * y 
                        - 121.0 * y * y * y * y - 2.0)
            + 5.5 * y * y * y * y * y * y * y * y
            + x / (2.0 * y);
    println!("=== f64 type1 ===");
    println!("{}", fff);

    let ffff = (333.75 - x * x) * y * y * y * y * y * y
                + x * x * (11.0 * x * x * y * y 
                            - 121.0 * y * y * y * y - 2.0)
                + 5.5 * y * y * y * y * y * y * y * y
                + x / (2.0 * y);
    println!("=== f64 type2 ===");
    println!("{}\n", ffff);

    let x: f32 = 77617.0;
    let y: f32 = 33096.0;
    let fff = 333.75 * y * y * y * y * y * y
            + x * x * (11.0 * x * x * y * y 
                        - y * y * y * y * y * y 
                        - 121.0 * y * y * y * y - 2.0)
            + 5.5 * y * y * y * y * y * y * y * y
            + x / (2.0 * y);
    println!("=== f32 type1 ===");
    println!("{}", fff);

    let ffff = (333.75 - x * x) * y * y * y * y * y * y
                + x * x * (11.0 * x * x * y * y 
                            - 121.0 * y * y * y * y - 2.0)
                + 5.5 * y * y * y * y * y * y * y * y
                + x / (2.0 * y);
    println!("=== f32 type2 ===");
    println!("{}", ffff);
}