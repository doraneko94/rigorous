use std::ops::{Add, Sub, Mul, Div};
use std::fmt::Debug;
use std::str::FromStr;
use std::string::ToString;
use std::ptr;
use libc::{c_int};
use num_traits::float::Float;
use num_traits::identities::{Zero, One};

const FE_TONEAREST: c_int = 0x000;
#[cfg(feature = "vc")]
const FE_DOWNWARD: c_int = 0x100;
#[cfg(feature = "vc")]
const FE_UPWARD : c_int = 0x200;

#[cfg(not(feature = "vc"))]
const FE_DOWNWARD: c_int = 0x400;
#[cfg(not(feature = "vc"))]
const FE_UPWARD : c_int = 0x800;

macro_rules! volatile {
    ($val:expr, $mode:ident, $t:ty) => {
        {
            let mut tmp = Zero::zero();
            let p = &mut tmp as *mut $t;
            unsafe {
                setround($mode);
                ptr::write_volatile(p, $val);
                setround(FE_TONEAREST);
            }
            tmp
        }
    };
}

extern "C" {
    fn fesetround(r_dir: c_int) -> c_int;
}

fn max<F: Float>(l: F, r: F) -> F {
    if l < r { r }
    else { l }
}

fn min<F: Float>(l: F, r: F) -> F {
    if l > r { r }
    else { l }
}

fn setround(r_dir: c_int) {
    let res = unsafe { fesetround(r_dir) };
    if res != 0 {
        panic!("failed to change direction to {}!", r_dir);
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Interval<F: Float + Debug + FromStr + ToString> {
    pub inf: F,
    pub sup: F,
}

impl<F: Float + Debug + FromStr + ToString> Interval<F>{
    pub fn bet_str(si: &str, ss: &str) -> Self {
        let mut inf: F = Zero::zero();
        let p = &mut inf as *mut F;
        let mut sup: F = Zero::zero();
        let q = &mut sup as *mut F;
        unsafe {
            setround(FE_DOWNWARD);
            match si.parse::<F>() {
                Ok(e) => ptr::write_volatile(p, e),
                Err(_) => panic!("inf parse error!"),
            };
            setround(FE_UPWARD);
            match ss.parse::<F>() {
                Ok(e) => ptr::write_volatile(q, e),
                Err(_) => panic!("sup parse error!"),
            };
            setround(FE_TONEAREST);
        }
        if inf > sup {
            panic!("inf is larger than sup!");
        }

        setround(FE_DOWNWARD);
        let sis = si.to_string();
        while inf.to_string() > sis {
            inf = inf - Float::epsilon();
        }
        setround(FE_UPWARD);
        let sss = ss.to_string();
        while sup.to_string() < sss {
            sup = sup + Float::epsilon();
        }
        setround(FE_TONEAREST);

        Self { inf, sup }
    }

    pub fn from_str(s: &str) -> Self {
        Self::bet_str(s, s)
    }

    pub fn hull(v: &Vec<F>) -> Self {
        let mut inf = Float::infinity();
        let mut sup = Float::neg_infinity();
        for &e in v.iter() {
            inf = min(inf, e);
            sup = max(sup, e);
        }
        Self { inf, sup }
    }

    pub fn floor(x: F) -> Self {
        let inf = volatile!(x.floor(), FE_DOWNWARD, F);
        let sup = volatile!(x.floor(), FE_UPWARD, F);
        Self { inf, sup }
    }

    pub fn pi() -> Self {
        let spi = "3.14159265358979323846264338327950288419716939937510582097494459230781640628620899";
        Self::from_str(spi)
    }

    pub fn pow(self, n: u32) -> Self {
        let mut ans: Interval<F> = Interval::from_str("1");
        for _ in 0..n {
            ans = ans * self;
        }
        ans
    }

    // prelude
    pub fn cos(self) -> Self {
        if self.inf.is_infinite() || self.sup.is_infinite() {
            Self::bet_str("-1", "1")
        } else {
            
            let mut v = Vec::with_capacity(4);
            v.push(volatile!(self.inf.cos(), FE_DOWNWARD, F));
            v.push(volatile!(self.inf.cos(), FE_UPWARD, F));
            v.push(volatile!(self.sup.cos(), FE_DOWNWARD, F));
            v.push(volatile!(self.sup.cos(), FE_UPWARD, F));
            let ip = Self::pi();
            let i2p = Self::from_str("2") * ip;

            let i_inf = Self { inf: self.inf, sup: self.inf };
            let i_n = i_inf / i2p;
            let self_new = self - i2p * Self::floor(i_n.inf);
            let mut ret = Self::hull(&v);
            let i_one = Self::bet_str("-1", "1");
            
            if self_new.inf <= ip.inf && self_new.sup >= ip.sup {
                ret.inf = i_one.inf;
            }
            if self_new.inf <= i2p.inf && self_new.sup >= i2p.sup {
                ret.sup = i_one.sup;
            }
            ret
        }
    }
}

impl<F: Float + Debug + FromStr + ToString> Add for Interval<F> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut inf = Zero::zero();
        let p = &mut inf as *mut F;
        let mut sup = Zero::zero();
        let q = &mut sup as *mut F;
        unsafe {
            setround(FE_DOWNWARD);
            ptr::write_volatile(p, self.inf + other.inf);
            setround(FE_UPWARD);
            ptr::write_volatile(q, self.sup + other.sup);
            setround(FE_TONEAREST);
        }
        Self { inf, sup }
    }
}

impl<F: Float + Debug + FromStr + ToString> Sub for Interval<F> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut inf = Zero::zero();
        let p = &mut inf as *mut F;
        let mut sup = Zero::zero();
        let q = &mut sup as *mut F;
        unsafe {
            setround(FE_DOWNWARD);
            ptr::write_volatile(p, self.inf - other.sup);
            setround(FE_UPWARD);
            ptr::write_volatile(q, self.sup - other.inf);
            setround(FE_TONEAREST);
        }
        Self { inf, sup }
    }
}

impl<F: Float + Debug + FromStr + ToString> Mul for Interval<F> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut inf = Zero::zero();
        let p = &mut inf as *mut F;
        let mut sup = Zero::zero();
        let q = &mut sup as *mut F;
        let (si, ss, oi, os) = (self.inf, self.sup, other.inf, other.sup);
        unsafe {
            setround(FE_DOWNWARD);
            ptr::write_volatile(p, min(min(si * oi, si * os), min(ss * oi, ss * os)));
            setround(FE_UPWARD);
            ptr::write_volatile(q, max(max(si * oi, si * os), max(ss * oi, ss * os)));
            setround(FE_TONEAREST);
        }
        Self { inf, sup }
    }
}

impl<F: Float + Debug + FromStr + ToString> Div for Interval<F> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let mut tinf = Zero::zero();
        let tp = &mut tinf as *mut F;
        let mut tsup = Zero::zero();
        let tq = &mut tsup as *mut F;
        unsafe {
            setround(FE_DOWNWARD);
            let mut one = Zero::zero();
            let op = &mut one as *mut F;
            ptr::write_volatile(op, One::one());
            ptr::write_volatile(tp, one / other.sup);
            setround(FE_UPWARD);
            let mut one = Zero::zero();
            let op = &mut one as *mut F;
            ptr::write_volatile(op, One::one());
            ptr::write_volatile(tq, one / other.inf);
            setround(FE_TONEAREST);
        }
        let tmp = Self { inf: tinf, sup: tsup };
        self * tmp
    }
}

