use rigorous::*;
use std::ptr;
use num_traits::identities::{Zero};

fn main() {
    let mut tmp: f64 = Zero::zero();
    let p = &mut tmp as *mut f64;
    unsafe {
        setround(FE_TONEAREST);
        match "0.1".parse::<f64>() {
            Ok(e) => ptr::write_volatile(p, e),
            Err(_) => panic!("parse error!"),
        };
    }
    println!("{:.25}\n", tmp);

    let mut tmp: f64 = Zero::zero();
    let p = &mut tmp as *mut f64;
    unsafe {
        setround(FE_DOWNWARD);
        match "0.1".parse::<f64>() {
            Ok(e) => ptr::write_volatile(p, e),
            Err(_) => panic!("parse error!"),
        };
    }
    println!("{:.25}\n", tmp);

    let mut tmp: f64 = Zero::zero();
    let p = &mut tmp as *mut f64;
    unsafe {
        setround(FE_UPWARD);
        match "0.1".parse::<f64>() {
            Ok(e) => ptr::write_volatile(p, e),
            Err(_) => panic!("parse error!"),
        };
    }
    println!("{:.25}\n", tmp);

    let mut tmp: f64 = Zero::zero();
    let p = &mut tmp as *mut f64;
    unsafe {
        setround(FE_TOWARDZERO);
        match "0.1".parse::<f64>() {
            Ok(e) => ptr::write_volatile(p, e),
            Err(_) => panic!("parse error!"),
        };
    }
    println!("{:.25}\n", tmp);

    let mut tmp: f64 = Zero::zero();
    let p = &mut tmp as *mut f64;
    unsafe {
        setround(FE_TONEAREST);
        match "-0.1".parse::<f64>() {
            Ok(e) => ptr::write_volatile(p, e),
            Err(_) => panic!("parse error!"),
        };
    }
    println!("{:.25}\n", tmp);

    let mut tmp: f64 = Zero::zero();
    let p = &mut tmp as *mut f64;
    unsafe {
        setround(FE_DOWNWARD);
        match "-0.1".parse::<f64>() {
            Ok(e) => ptr::write_volatile(p, e),
            Err(_) => panic!("parse error!"),
        };
    }
    println!("{:.25}\n", tmp);

    let mut tmp: f64 = Zero::zero();
    let p = &mut tmp as *mut f64;
    unsafe {
        setround(FE_UPWARD);
        match "-0.1".parse::<f64>() {
            Ok(e) => ptr::write_volatile(p, e),
            Err(_) => panic!("parse error!"),
        };
    }
    println!("{:.25}\n", tmp);

    let mut tmp: f64 = Zero::zero();
    let p = &mut tmp as *mut f64;
    unsafe {
        setround(FE_TOWARDZERO);
        match "-0.1".parse::<f64>() {
            Ok(e) => ptr::write_volatile(p, e),
            Err(_) => panic!("parse error!"),
        };
    }
    println!("{:.25}\n", tmp);

    let i1: Interval<f64> = Interval::from_str("-1");
    let i10: Interval<f64> = Interval::from_str("10");
    let inf1 = i1.inf;
    let inf10 = i10.inf;

    let mut tmp: f64 = Zero::zero();
    let p = &mut tmp as *mut f64;
    unsafe {
        setround(FE_DOWNWARD);
        ptr::write_volatile(p, inf1 / inf10);
    }
    println!("{:.25}\n", tmp);

    let mut tmp: f64 = Zero::zero();
    let p = &mut tmp as *mut f64;
    unsafe {
        setround(FE_UPWARD);
        ptr::write_volatile(p, inf1 / inf10);
    }
    println!("{:.25}\n", tmp);

    println!("{:?}", i1 / i10 + i1 / i10);
}