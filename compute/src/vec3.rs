use derive_more::{
    Add, AddAssign, Constructor, Display, Div, DivAssign, Mul, MulAssign, Product, Sub, SubAssign,
    Sum,
};

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
    pub(crate) fn zero() -> Vec3 {
        Default::default()
    }

    pub(crate) fn one() -> Vec3 {
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
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

pub struct Color(Vec3);
impl Color {
    pub(crate) fn new(r: f64, g: f64, b: f64) -> Color {
        Color(Vec3::new(r, g, b))
    }

    pub(crate) fn to_rgba(&self) -> [u8; 4] {
        let r = (self.0.x * 255.999) as u8;
        let g = (self.0.y * 255.999) as u8;
        let b = (self.0.z * 255.999) as u8;
        let a = 255;
        [r, g, b, a]
    }
}

pub(crate) struct Point3(Vec3);
