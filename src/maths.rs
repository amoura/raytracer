use std::ops::{Add,Sub,Neg,Mul,Div};

const F64_SMALL_NUM: f64 = 1.0e-10;

fn almost_same(x: f64, y:f64) -> bool {
    f64::abs(x - y) < F64_SMALL_NUM
}

#[derive(Debug,Clone,Copy)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        return almost_same(self.x, other.x)
            && almost_same(self.y, other.y) 
            && almost_same(self.z, other.z) 
            && almost_same(self.w, other.w);
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: scalar * self.x,
            y: scalar * self.y,
            z: scalar * self.z,
            w: scalar * self.w,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        let factor = 1.0/scalar;
        self * factor
    }
}

impl Tuple {
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn norm2(self) -> f64 {
        Tuple::dot(self, self)
    }

    pub fn norm(self) -> f64 {
        f64::sqrt(self.norm2())
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x*other.x + self.y*other.y + self.z*other.z + self.w*other.w
    }

    pub fn normalised(self) -> Self {
        self / self.norm()
    }

    pub fn cross(self, other: Self) -> Self {
        vector(self.y*other.z - self.z*other.y,
            self.z*other.x - self.x*other.z,
            self.x*other.y - self.y*other.x)
    }
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple{x, y, z, w:0.0}
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple{x, y, z, w:1.0}
}

#[cfg(test)]
mod tst {
    use super::*;

    #[test]
    fn vectors_and_points() {
        let vec = Tuple{x:4.3, y:-2.1, z:3.2, w:0.0};
        assert!(vec.is_vector());
        assert!(!vec.is_point());

        let pt = Tuple{x:4.3, y:-2.1, z:3.2, w:1.0};
        assert!(!pt.is_vector());
        assert!(pt.is_point());
    }

    #[test]
    fn factory() {
        let vec = vector(4.3, 2.3, -9.1);
        assert_eq!(vec, Tuple{x:4.3, y:2.3, z:-9.1, w:0.0});
        let vec = point(4.3, 2.3, -9.1);
        assert_eq!(vec, Tuple{x:4.3, y:2.3, z:-9.1, w:1.0});
    }

    #[test]
    fn simple_arith() {
        let p = Tuple{x:1.0, y:4.0, z:-2.0, w:1.0};
        let v = Tuple{x:-3.0, y:-1.0, z:2.3, w:0.0};
        assert_eq!(p+v, Tuple{x:-2.0, y:3.0, z:0.3, w:1.0});

        let p1 = point(2.0, 3.0, -1.0);
        let p2 = point(3.0, -1.0, 2.0);
        assert_eq!(p2-p1, vector(1.0, -4.0, 3.0));

        let v1 = vector(1.0, 2.0, -3.0);
        assert_eq!(p1-v1, point(1.0, 1.0, 2.0));
        let v2 = vector(0.0, 1.0, -1.0);
        assert_eq!(v1-v2, vector(1.0, 1.0, -2.0));

        assert_eq!(-v1, vector(-1.0, -2.0, 3.0));

        assert_eq!(p*2.0, Tuple{x:2.0, y:8.0, z:-4.0, w:2.0});
        assert_eq!(p*0.5, Tuple{x:0.5, y:2.0, z:-1.0, w:0.5});
        assert_eq!(p/2.0, Tuple{x:0.5, y:2.0, z:-1.0, w:0.5});
    }

    #[test]
    fn norm() {
        assert_eq!(vector(1.0,0.0,0.0).norm(), 1.0);
        assert_eq!(vector(0.0,1.0,0.0).norm(), 1.0);
        assert_eq!(vector(0.0,0.0,1.0).norm(), 1.0);
        assert_eq!(vector(1.0,2.0,3.0).norm(), f64::sqrt(14.0));
        assert_eq!(vector(-1.0,-2.0,-3.0).norm(), f64::sqrt(14.0));

        assert_eq!(vector(1.0,0.0,0.0).normalised(), vector(1.0,0.0,0.0));
        assert_eq!(vector(0.0,1.0,0.0).normalised(), vector(0.0,1.0,0.0));
        assert_eq!(vector(0.0,0.0,1.0).normalised(), vector(0.0,0.0,1.0));
        assert_eq!(vector(1.0,0.0,0.0).normalised().norm(), 1.0);

        let v1 = vector(1.0,2.0,3.0);
        let v2 = vector(2.0,3.0,4.0);
        assert_eq!(Tuple::dot(v1, v2), 20.0);
        assert_eq!(Tuple::cross(v1, v2), vector(-1.0,2.0,-1.0));
        assert_eq!(Tuple::cross(v2, v1), vector(1.0,-2.0,1.0));
    }
}