use std::fmt::{Display, Formatter, Result};
use std::ops::*;

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }
    fn length(&self) -> f64 {
        self.lenght_squared().sqrt()
    }

    fn lenght_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }
    pub fn dot(lhs: Self, rhs: Self) -> f64 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }
    fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
    pub fn unit_vector(v: Self) -> Self {
        v / v.length()
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
#[test]
fn neg_test() {
    let x = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(-x, Vec3::new(-1.0, -2.0, -3.0));
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid Vec3 index: {idx}"),
        }
    }
}

#[test]
fn index_test() {
    let x = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(x[1], 2.0);
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, idx: usize) -> &mut f64 {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid Vec3 index: {idx}"),
        }
    }
}

#[test]
fn index_mut_test() {
    let mut x = Vec3::new(1.0, 2.0, 3.0);
    x[1] = 5.0;
    assert_eq!(x[1], 5.0);
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

#[test]
fn add_assign_test() {
    let mut x = Vec3::new(1.0, 2.0, 3.0);
    x += Vec3::new(4.0, 3.0, 2.0);
    assert_eq!(x, Vec3::new(5.0, 5.0, 5.0));
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

#[test]
fn mul_assign_test() {
    let mut x = Vec3::new(1.0, 1.0, 1.0);
    x *= Vec3::new(5.0, 5.0, 5.0);
    assert_eq!(x, Vec3::new(5.0, 5.0, 5.0));
}
#[test]
fn mul_assign_f64_test() {
    let mut x = Vec3::new(3.0, 5.0, 6.0);
    x *= 5.0;
    assert_eq!(x, Vec3::new(15.0, 25.0, 30.0));
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x *= 1.0 / rhs.x;
        self.y *= 1.0 / rhs.y;
        self.z *= 1.0 / rhs.z;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1.0 / t;
    }
}

#[test]
fn div_assign_test() {
    let mut x = Vec3::new(10.0, 10.0, 10.0);
    x /= Vec3::new(1.0, 2.0, 5.0);
    assert_eq!(x, Vec3::new(10.0, 5.0, 2.0));
}
#[test]
fn div_assign_f64_test() {
    let mut x = Vec3::new(4.0, 8.0, 12.0);
    x /= 4.0;
    assert_eq!(x, Vec3::new(1.0, 2.0, 3.0));
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{{ x: {} y: {} z: {} }}", self.x, self.y, self.z)
    }
}

#[test]
fn display_test() {
    let x = Vec3::new(1.0, 2.0, 5.0);
    assert_eq!(x.to_string(), "{ x: 1 y: 2 z: 5 }");
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[test]
fn add_test() {
    let c = Vec3::new(1.0, 2.0, 3.0) + Vec3::new(4.0, 3.0, 2.0);
    assert_eq!(c, Vec3::new(5.0, 5.0, 5.0));
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
#[test]
fn sub_test() {
    let c = Vec3::new(5.0, 5.0, 5.0) - Vec3::new(4.0, 3.0, 2.0);
    assert_eq!(c, Vec3::new(1.0, 2.0, 3.0));
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

#[test]
fn mul_test() {
    let c = Vec3::new(5.0, 5.0, 5.0) * Vec3::new(2.0, 2.0, 2.0);
    assert_eq!(c, Vec3::new(10.0, 10.0, 10.0));
}
#[test]
fn mul_f64_test() {
    let c = Vec3::new(5.0, 5.0, 5.0) * 2.0;
    assert_eq!(c, Vec3::new(10.0, 10.0, 10.0));
}

#[test]
fn mul_flipped_f64_test() {
    let c = 2.0 * Vec3::new(5.0, 5.0, 5.0);
    assert_eq!(c, Vec3::new(10.0, 10.0, 10.0));
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x * 1.0 / rhs.x,
            y: self.y * 1.0 / rhs.y,
            z: self.z * 1.0 / rhs.z,
        }
    }
}
#[test]
fn div_test() {
    let x = Vec3::new(8.0, 16.0, 20.0) / Vec3::new(4.0, 8.0, 5.0);
    assert_eq!(x, Vec3::new(2.0, 2.0, 4.0));
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        self * (1.0 / rhs)
    }
}

#[test]
fn div_f64_test() {
    let mut x = Vec3::new(6.0, 4.0, 8.0);
    x = x / 2.0;
    assert_eq!(x, Vec3::new(3.0, 2.0, 4.0));
}
