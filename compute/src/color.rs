use derive_more::{
    Add, AddAssign, Constructor, Display, Div, DivAssign, Mul, MulAssign, Neg, Product, Sub,
    SubAssign, Sum,
};

use crate::vec3::Vec3;

#[derive(
    Debug, Copy, Clone, PartialEq, Default, Constructor, Add, AddAssign, Sub, Mul, MulAssign, Div, Product
)]
pub struct Color {
    pub(crate) r: f64,
    pub(crate) g: f64,
    pub(crate) b: f64,
}
impl Color {
    pub const RED: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
    };

    pub const GREEN: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 0.0,
    };

    pub const BLUE: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 1.0,
    };

    pub const YELLOW: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 0.0,
    };

    pub const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };

    pub const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };

    pub(crate) fn sqrt(&self) -> Color {
        Color {
            r: self.r.sqrt(),
            g: self.g.sqrt(),
            b: self.b.sqrt(),
        }
    }

    pub(crate) fn of_vec3(v: Vec3) -> Color {
        Color {
            r: v.x,
            g: v.y,
            b: v.z,
        }
    }

    pub(crate) fn to_bytes(&self) -> [u8; 4] {
        let r = (self.r * 255.999) as u8;
        let g = (self.g * 255.999) as u8;
        let b = (self.b * 255.999) as u8;
        let a = 255;
        [r, g, b, a]
    }

    pub(crate) fn blend(&self, other: &Color, ratio: f64) -> Color {
        let r = self.r * (1.0 - ratio) + other.r * ratio;
        let g = self.g * (1.0 - ratio) + other.g * ratio;
        let b = self.b * (1.0 - ratio) + other.b * ratio;
        Color { r, g, b }
    }
}

impl std::ops::Mul<Self> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}