use std::ops::{Add,Sub,Neg,Mul,Div,Index,IndexMut};

const F64_SMALL_NUM: f64 = 1.0e-10;

pub fn almost_same(x: f64, y:f64) -> bool {
    f64::abs(x - y) < F64_SMALL_NUM
}

#[derive(Debug,Clone,Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        almost_same(self.x, other.x)
            && almost_same(self.y, other.y) 
            && almost_same(self.z, other.z) 
            && almost_same(self.w, other.w)
    }
}

impl Index<usize> for Tuple {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index out of bounds in Tuple indexing"),
        }
    }
}

impl IndexMut<usize> for Tuple {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Index out of bounds in Tuple indexing"),
        }
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
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self {x, y, z, w}
    }

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

//////////////////////////////////////////////////////////////////////////
/// Matrices
/// 
#[derive(Debug,Clone,Copy)]
pub struct Matrix4 {
    pub m00: f64, pub m01: f64, pub m02: f64, pub m03: f64,
    pub m10: f64, pub m11: f64, pub m12: f64, pub m13: f64,
    pub m20: f64, pub m21: f64, pub m22: f64, pub m23: f64,
    pub m30: f64, pub m31: f64, pub m32: f64, pub m33: f64,
}


#[derive(Debug,Clone,Copy)]
pub struct Matrix3 {
    pub m00: f64, pub m01: f64, pub m02: f64,
    pub m10: f64, pub m11: f64, pub m12: f64,
    pub m20: f64, pub m21: f64, pub m22: f64,
}

#[derive(Debug,Clone,Copy)]
pub struct Matrix2 {
    pub m00: f64, pub m01: f64,
    pub m10: f64, pub m11: f64,
}

impl PartialEq for Matrix4 {
    fn eq(&self, other: &Self) -> bool {
        return almost_same(self.m00, other.m00)
            && almost_same(self.m01, other.m01) 
            && almost_same(self.m02, other.m02)
            && almost_same(self.m03, other.m03)

            && almost_same(self.m10, other.m10) 
            && almost_same(self.m11, other.m11) 
            && almost_same(self.m12, other.m12)
            && almost_same(self.m13, other.m13)

            && almost_same(self.m20, other.m20) 
            && almost_same(self.m21, other.m21) 
            && almost_same(self.m22, other.m22)
            && almost_same(self.m23, other.m23)

            && almost_same(self.m30, other.m30) 
            && almost_same(self.m31, other.m31) 
            && almost_same(self.m32, other.m32)
            && almost_same(self.m33, other.m33);
    }
}

impl PartialEq for Matrix3 {
    fn eq(&self, other: &Self) -> bool {
        return almost_same(self.m00, other.m00)
            && almost_same(self.m01, other.m01) 
            && almost_same(self.m02, other.m02)

            && almost_same(self.m10, other.m10) 
            && almost_same(self.m11, other.m11) 
            && almost_same(self.m12, other.m12)

            && almost_same(self.m20, other.m20) 
            && almost_same(self.m21, other.m21) 
            && almost_same(self.m22, other.m22);
    }
}

impl PartialEq for Matrix2 {
    fn eq(&self, other: &Self) -> bool {
        return almost_same(self.m00, other.m00)
            && almost_same(self.m01, other.m01) 

            && almost_same(self.m10, other.m10) 
            && almost_same(self.m11, other.m11);
    }
}

impl Mul for Matrix4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut m = Self::zero();
        for i in 0..4 {
            for j in 0..4 {
                let mut val = 0.0;
                for k in 0..4 {
                    val += self.at(i,k)*other.at(k, j);
                }
                m.set(val, i, j);
            }
        }
        m
    }
}

impl Mul<Tuple> for Matrix4 {
    type Output = Tuple;

    fn mul(self, v: Tuple) -> Tuple {
        let mut rv = Tuple::new(0.0,0.0,0.0,0.0);
        for i in 0..4 {
            let mut val = 0.0;
            for k in 0..4 {
                val += self.at(i,k)*v[k];
            }
            rv[i] = val;
        }
        rv
    }
}

impl Matrix4 {
    pub fn new(
        m00: f64, m01: f64, m02: f64, m03: f64,
        m10: f64, m11: f64, m12: f64, m13: f64,
        m20: f64, m21: f64, m22: f64, m23: f64,
        m30: f64, m31: f64, m32: f64, m33: f64) -> Self {
        Self {
            m00, m01, m02, m03,
            m10, m11, m12, m13,
            m20, m21, m22, m23,
            m30, m31, m32, m33,       
        }
    }

    pub fn zero() -> Self {
        Self::new(
            0.0, 0.0, 0.0, 0.0,        
            0.0, 0.0, 0.0, 0.0,        
            0.0, 0.0, 0.0, 0.0,        
            0.0, 0.0, 0.0, 0.0)        
    }

    pub fn identity() -> Self {
        Self::new(
            1.0, 0.0, 0.0, 0.0,        
            0.0, 1.0, 0.0, 0.0,        
            0.0, 0.0, 1.0, 0.0,        
            0.0, 0.0, 0.0, 1.0)        
    }

    pub fn at(&self, x: usize, y: usize) -> f64 {
        match (x, y) {
            (0, 0) => self.m00,
            (0, 1) => self.m01,
            (0, 2) => self.m02,
            (0, 3) => self.m03,

            (1, 0) => self.m10,
            (1, 1) => self.m11,
            (1, 2) => self.m12,
            (1, 3) => self.m13,

            (2, 0) => self.m20,
            (2, 1) => self.m21,
            (2, 2) => self.m22,
            (2, 3) => self.m23,

            (3, 0) => self.m30,
            (3, 1) => self.m31,
            (3, 2) => self.m32,
            (3, 3) => self.m33,

            _ => panic!("Out of bounds index in Matrix4::at()"),
        }
    }

    pub fn set(&mut self, val: f64, x: usize, y: usize) {
        match (x, y) {
            (0, 0) => self.m00 = val,
            (0, 1) => self.m01 = val,
            (0, 2) => self.m02 = val,
            (0, 3) => self.m03 = val,

            (1, 0) => self.m10 = val,
            (1, 1) => self.m11 = val,
            (1, 2) => self.m12 = val,
            (1, 3) => self.m13 = val,

            (2, 0) => self.m20 = val,
            (2, 1) => self.m21 = val,
            (2, 2) => self.m22 = val,
            (2, 3) => self.m23 = val,

            (3, 0) => self.m30 = val,
            (3, 1) => self.m31 = val,
            (3, 2) => self.m32 = val,
            (3, 3) => self.m33 = val,

            _ => panic!("Out of bounds index in Matrix4::at()"),
        }
    }

    pub fn transpose(&self) -> Self {
        let mut m = *self;
        for i in 0..3 {
            for j in i+1..4 {
                let tmp = self.at(i,j);
                m.set(m.at(j,i), i, j);
                m.set(tmp, j, i);
            }
        }
        m
    }
}

impl Matrix3 {
    pub fn new(
        m00: f64, m01: f64, m02: f64,
        m10: f64, m11: f64, m12: f64,
        m20: f64, m21: f64, m22: f64) -> Self {
        Self {
            m00, m01, m02,
            m10, m11, m12,
            m20, m21, m22,
        }
    }
}

impl Matrix2 {
    pub fn new(
        m00: f64, m01: f64,
        m10: f64, m11: f64) -> Self {
        Self {
            m00, m01,
            m10, m11,
        }
    }
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

    #[test]
    fn matrix() {
        let m4 = Matrix4::new(
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 10.0, 11.0, 12.0,
            13.0, 14.0, 15.0, 16.0);
        assert!(almost_same(m4.m02, 3.0));
        assert!(almost_same(m4.m10, 5.0));
        assert!(almost_same(m4.m23, 12.0));
        assert!(almost_same(m4.m33, 16.0));

        let m4_ = Matrix4::new(
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 10.0, 11.0, 12.0,
            13.0, 14.0, 15.0, 16.0);
        assert_eq!(m4, m4_);
        let m4_ = Matrix4::new(
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 10.0, 11.1, 12.0,
            13.0, 14.0, 15.0, 16.0);
            assert_ne!(m4, m4_);
        
        let m3 = Matrix3::new(
            1.0, 2.0, 3.0,
            5.0, 6.0, 7.0,
            9.0, 10.0, 11.0);
        assert!(almost_same(m3.m02, 3.0));
        assert!(almost_same(m3.m10, 5.0));
        assert!(almost_same(m3.m22, 11.0));

        let m3_ = Matrix3::new(
            1.0, 2.0, 3.0,
            5.0, 6.0, 7.0,
            9.0, 10.0, 11.0);
        assert_eq!(m3, m3_);
        let m3_ = Matrix3::new(
            1.0, 2.0, 3.0,
            5.0, 6.0, 7.1,
            9.0, 10.0, 11.0);
        assert_ne!(m3, m3_);

        let m2 = Matrix2::new(
            1.0, 2.0, 
            5.0, 6.0);
        assert!(almost_same(m2.m01, 2.0));
        assert!(almost_same(m2.m10, 5.0));

        let m2_ = Matrix2::new(
            1.0, 2.0,
            5.0, 6.0);
        assert_eq!(m2, m2_);
        let m2_ = Matrix2::new(
            1.0, 2.0,
            5.0, 6.1);
        assert_ne!(m2, m2_);    

        let m4 = Matrix4::new(
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0);
        let m4_ = Matrix4::new(
            -2.0, 1.0, 2.0, 3.0,
            3.0, 2.0, 1.0, -1.0,
            4.0, 3.0, 6.0, 5.0,
            1.0, 2.0, 7.0, 8.0);
        let prod =  Matrix4::new(
            20.0, 22.0, 50.0, 48.0,
            44.0, 54.0, 114.0, 108.0,
            40.0, 58.0, 110.0, 102.0,
            16.0, 26.0, 46.0, 42.0);
        assert_eq!(m4 * m4_, prod);

        let m4 = Matrix4::new(
            1.0, 2.0, 3.0, 4.0,
            2.0, 4.0, 4.0, 2.0,
            8.0, 6.0, 4.0, 1.0,
            0.0, 0.0, 0.0, 1.0);
        let v = Tuple::new(1.0, 2.0, 3.0, 1.0);
        let prod = Tuple::new(18.0, 24.0, 33.0, 1.0);
        assert_eq!(m4*v, prod);

        assert_eq!(Matrix4::identity() * m4_, m4_);
        assert_eq!(m4_ * Matrix4::identity(), m4_);
        assert_eq!(Matrix4::identity() * v, v);

        let m4_t = Matrix4::new(
            1.0, 2.0, 8.0, 0.0,
            2.0, 4.0, 6.0, 0.0,
            3.0, 4.0, 4.0, 0.0,
            4.0, 2.0, 1.0, 1.0);
        assert_eq!(m4.transpose(), m4_t);
        assert_eq!(m4_t.transpose(), m4);
        assert_eq!(Matrix4::identity().transpose(), Matrix4::identity());
    }
}