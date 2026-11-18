use std::ops::{Add, AddAssign, Mul, Neg, Sub};

/// Număr complex în precizie dublă (layout compatibil C: re, im).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct C64 {
    pub re: f64,
    pub im: f64,
}

pub type Mat2 = [[C64; 2]; 2];

impl C64 {
    pub const ZERO: C64 = C64 { re: 0.0, im: 0.0 };
    pub const ONE: C64 = C64 { re: 1.0, im: 0.0 };
    pub const I: C64 = C64 { re: 0.0, im: 1.0 };
    #[inline]
    pub const fn new(re: f64, im: f64) -> Self {
        C64 { re, im }
    }
    #[inline]
    pub fn from_polar(r: f64, theta: f64) -> Self {
        C64::new(r * theta.cos(), r * theta.sin())
    }
    #[inline]
    pub fn norm_sqr(self) -> f64 {
        self.re * self.re + self.im * self.im
    }
    #[inline]
    pub fn conj(self) -> Self {
        C64::new(self.re, -self.im)
    }
    #[inline]
    pub fn arg(self) -> f64 {
        self.im.atan2(self.re)
    }
}
impl Add for C64 {
    type Output = C64;
    #[inline]
    fn add(self, o: C64) -> C64 {
        C64::new(self.re + o.re, self.im + o.im)
    }
}
impl AddAssign for C64 {
    #[inline]
    fn add_assign(&mut self, o: C64) {
        self.re += o.re;
        self.im += o.im;
    }
}
impl Sub for C64 {
    type Output = C64;
    #[inline]
    fn sub(self, o: C64) -> C64 {
        C64::new(self.re - o.re, self.im - o.im)
    }
}
impl Mul for C64 {
    type Output = C64;
    #[inline]
    fn mul(self, o: C64) -> C64 {
        C64::new(self.re * o.re - self.im * o.im, self.re * o.im + self.im * o.re)
    }
}
impl Mul<f64> for C64 {
    type Output = C64;
    #[inline]
    fn mul(self, k: f64) -> C64 {
        C64::new(self.re * k, self.im * k)
    }
}
impl Neg for C64 {
    type Output = C64;
    #[inline]
    fn neg(self) -> C64 {
        C64::new(-self.re, -self.im)
    }
}
