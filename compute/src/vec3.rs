use derive_more::{
    Add, AddAssign, Constructor, Display, Div, DivAssign, Mul, MulAssign, Neg, Product, Sub,
    SubAssign, Sum,
};

use crate::math::js_rand_range;

#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Constructor,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Div,
    DivAssign,
    Mul,
    MulAssign,
    Neg,
    Product,
    Sum,
    Display,
)]
#[display(fmt = "({}, {}, {})", x, y, z)]
pub(crate) struct Vec3 {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}

impl Vec3 {
    pub(crate) const ZERO: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub(crate) const ONE: Vec3 = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    pub(crate) fn x(x: f64) -> Self {
        Self { x, ..Self::ZERO }
    }

    pub(crate) fn y(y: f64) -> Self {
        Self { y, ..Self::ZERO }
    }

    pub(crate) fn z(z: f64) -> Self {
        Self { z, ..Self::ZERO }
    }

    pub(crate) fn random(min: f64, max: f64) -> Self {
        Self {
            x: js_rand_range(min, max),
            y: js_rand_range(min, max),
            z: js_rand_range(min, max),
        }
    }

    pub(crate) fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub(crate) fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub(crate) fn random_in_hemisphere(&self) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(&self) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub(crate) fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub(crate) fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub(crate) fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub(crate) fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub(crate) fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }
}

impl std::ops::Mul<Self> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default, Constructor)]
pub(crate) struct Point3 {
    pub vec: Vec3,
}

impl Point3 {
    pub(crate) const ZERO: Point3 = Point3 { vec: Vec3::ZERO };

    pub(crate) fn vec(x: f64, y: f64, z: f64) -> Point3 {
        Point3 {
            vec: Vec3::new(x, y, z),
        }
    }
}

impl Into<Vec3> for Point3 {
    fn into(self) -> Vec3 {
        self.vec
    }
}
