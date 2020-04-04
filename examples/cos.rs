use rigorous::*;

fn main() {
    let i: Interval<f64> = Interval::bet_str("5", "17");
    let c = i.cos();
    println!("i = [5, 7]");
    println!("c = i.cos() = [inf, sup]");
    println!("inf = {:.25}", c.inf);
    println!("sup = {:.25}\n", c.sup);

    let i: Interval<f64> = Interval::bet_str("7", "12");
    let c = i.cos();
    println!("i = [7, 12]");
    println!("c = i.cos() = [inf, sup]");
    println!("inf = {:.25}", c.inf);
    println!("sup = {:.25}\n", c.sup);

    let i: Interval<f64> = Interval::pi();
    let c = i.cos();
    println!("i = [pi, pi]");
    println!("c = i.cos() = [inf, sup]");
    println!("inf = {:.25}", c.inf);
    println!("sup = {:.25}", c.sup);
}