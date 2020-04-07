use libc::{c_int};
use ndarray::*;

const FE_TONEAREST: c_int = 0x000;
#[cfg(feature = "vc")]
const FE_DOWNWARD: c_int = 0x100;
#[cfg(feature = "vc")]
pub const FE_UPWARD : c_int = 0x200;

#[cfg(not(feature = "vc"))]
const FE_DOWNWARD: c_int = 0x400;
#[cfg(not(feature = "vc"))]
const FE_UPWARD : c_int = 0x800;

extern {
    fn fesetround(rdir: c_int) -> c_int;
}

fn main() {
    let x = arr1(&[1.0f64, 2.0f64, 3.0f64]);
    let y = arr1(&[5.0f64, 6.0f64, 7.0f64]);

    println!("x = [1.0f64, 2.0f64, 3.0f64]");
    println!("y = [5.0f64, 6.0f64, 7.0f64]");
    println!("");
    println!("up:");
    unsafe { fesetround(FE_UPWARD) };
    let xu = x.clone() / 10.0;
    let yu = y.clone() / 10.0;
    println!("xu = x / 10.0");
    println!("yu = y / 10.0");
    let su = xu.dot(&yu);
    println!("xu.dot(&yu) = ");
    println!("{:25.20}\n", su);

    println!("down:");
    unsafe { fesetround(FE_DOWNWARD) };
    let xl = x.clone() / 10.0;
    let yl = y.clone() / 10.0;
    println!("xl = x / 10.0");
    println!("yl = y / 10.0");
    let sl = xl.dot(&yl);
    println!("xl.dot(&yl) = ");
    println!("{:25.20}\n", sl);
}