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
        setround(0x300);
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
        setround(0x300);
        match "-0.1".parse::<f64>() {
            Ok(e) => ptr::write_volatile(p, e),
            Err(_) => panic!("parse error!"),
        };
    }
    println!("{:.25}\n", tmp);
}