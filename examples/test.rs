use rigorous::*;

fn factorial(n: usize) -> Interval<f64> {
    if n <= 1 {
        Interval::from_str("1")
    } else {
        let mut ret = Interval::from_str("1");
        for i in 2..n+1 {
            ret = ret * Interval::from_str(&(i.to_string()));
        }
        ret
    }
}

fn main() {
    let i: Interval<f64> = Interval::from_str("0.1");
    println!("i = [0.1, 0.1] = [inf, sup]");
    println!("inf = {:.25}", i.inf);
    println!("sup = {:.25}\n", i.sup);

    let i1: Interval<f64> = Interval::from_str("1");
    let i10: Interval<f64> = Interval::from_str("10");
    let i110 = i1 / i10;
    println!("i1 = [1, 1]");
    println!("i10 = [10, 10]");
    println!("i110 = i1 / i10 = [inf, sup]");
    println!("inf = {:.25}", i110.inf);
    println!("sup = {:.25}\n", i110.sup);

    let intp: Interval<f64> = Interval::pi();
    println!("intp = [pi, pi] = [inf, sup]");
    println!("inf = {:.25}", intp.inf);
    println!("sup = {:.25}\n", intp.sup);

    let int6: Interval<f64> = Interval::from_str("6");
    println!("int6 = [6, 6] = [inf, sup]");
    println!("inf = {:.25}", int6.inf);
    println!("sup = {:.25}\n", int6.sup);

    let theta = intp / int6;
    println!("theta = intp / int6 = [inf, sup]");
    println!("inf = {:.25}", theta.inf);
    println!("sup = {:.25}\n", theta.sup);

    let mut ans = theta;
    ans = ans - theta.pow(3) / factorial(3);
    ans = ans + theta.pow(5) / factorial(5);
    ans = ans - theta.pow(7) / factorial(7);
    ans = ans + theta.pow(9) / factorial(9);
    println!("=== cos(pi/6) without R11 ===");
    println!("[{:.25} {:.25}]\n", ans.inf, ans.sup);

    let xi: Interval<f64> = Interval::bet_str("0", "1");
    let c = xi.cos();
    ans = ans - c * theta.pow(11) / factorial(11);
    println!("=== cos(pi/6) with R11 ===");
    println!("[{:.25} {:.25}]", ans.inf, ans.sup);
}