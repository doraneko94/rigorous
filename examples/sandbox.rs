use rigorous::*;
use std::ptr;
use num_traits::identities::{Zero};



macro_rules! down_up {
    ($func:ident, $name: ident, $val:ident) => {
        setround(FE_DOWNWARD);
        let mut tmp = Zero::zero();
        let p = &mut tmp as *mut f64;
        unsafe { ptr::write_volatile(p, $val.$func()); }
        println!("inf.{} = {:.25}", $name, tmp);
        setround(FE_UPWARD);
        let mut tmp = Zero::zero();
        let p = &mut tmp as *mut f64;
        unsafe { ptr::write_volatile(p, $val.$func()); }
        println!("inf.{} = {:.25}\n", $name, tmp);
    };
}

fn main() {
    let i: Interval<f64> = Interval::from_str("0.1");
    let inf = i.inf;
    let sup = i.sup;

    println!("inf = {:.25}", inf);
    println!("sup = {:.25}\n", sup);

    setround(0x100);
    let mut tmp = Zero::zero();
    let p = &mut tmp as *mut f64;
    unsafe { ptr::write_volatile(p, inf.sqrt()); }
    println!("inf.sqrt = {:.25}", tmp);
    setround(0x200);
    let mut tmp = Zero::zero();
    let p = &mut tmp as *mut f64;
    unsafe { ptr::write_volatile(p, inf.sqrt()); }
    println!("inf.sqrt = {:.25}\n", tmp);

    let name = "floor";
    down_up!(floor, name, inf);
    
    let name = "ceil";
    down_up!(ceil, name, inf);

    let name = "round";
    down_up!(round, name, inf);

    let name = "trunc";
    down_up!(trunc, name, inf);

    let name = "fract";
    down_up!(fract, name, inf);

    let name = "abs";
    down_up!(abs, name, inf);

    let name = "signum";
    down_up!(signum, name, inf);

    let name = "sqrt";
    down_up!(sqrt, name, inf);

    let name = "exp";
    down_up!(exp, name, inf);

    let name = "exp2";
    down_up!(exp2, name, inf);

    let name = "ln";
    down_up!(ln, name, inf);

    let name = "log2";
    down_up!(log2, name, inf);

    let name = "log10";
    down_up!(log10, name, inf);

    let name = "cbrt";
    down_up!(cbrt, name, inf);

    let name = "sin";
    down_up!(sin, name, inf);

    let name = "cos";
    down_up!(cos, name, inf);

    let name = "tan";
    down_up!(tan, name, inf);

    let name = "asin";
    down_up!(asin, name, inf);

    let name = "acos";
    down_up!(acos, name, inf);

    let name = "atan";
    down_up!(atan, name, inf);

    let name = "exp_m1";
    down_up!(exp_m1, name, inf);

    let name = "ln_1p";
    down_up!(ln_1p, name, inf);

    let name = "sinh";
    down_up!(sinh, name, inf);

    let name = "cosh";
    down_up!(cosh, name, inf);

    let name = "tanh";
    down_up!(tanh, name, inf);

    let name = "asinh";
    down_up!(asinh, name, inf);

    let name = "acosh";
    down_up!(acosh, name, inf);

    let name = "atanh";
    down_up!(atanh, name, inf);
}