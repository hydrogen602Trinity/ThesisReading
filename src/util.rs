use core::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vect3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Default for Vect3 {
    fn default() -> Self {
        Vect3::ZERO
    }
}

impl fmt::Display for Vect3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}

impl From<(f64, f64, f64)> for Vect3 {
    fn from(d: (f64, f64, f64)) -> Self {
        Vect3 {
            x: d.0,
            y: d.1,
            z: d.2,
        }
    }
}

impl Vect3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vect3 {
        Vect3 { x, y, z }
    }

    pub fn norm(&self) -> Vect3 {
        self / self.mag()
    }

    pub fn mag_sq(&self) -> f64 {
        self * self
    }

    pub fn mag(&self) -> f64 {
        (self * self).sqrt()
    }

    pub fn cross(&self, other: &Self) -> Vect3 {
        // [ a2 b3 - a3 b2 , a3 b1 - a1 b3 , a1 b2 - a2 b1 ]

        Vect3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn print(&self) {
        print!("<{:.2e} {:.2e} {:.2e}>", self.x, self.y, self.z);
    }

    pub fn println(&self) {
        println!("<{:.2e} {:.2e} {:.2e}>", self.x, self.y, self.z);
    }

    pub const ZERO: Vect3 = Vect3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
}

// #[macro_export]
macro_rules! helper {
    ( $type:ty ) => {
        impl AddAssign<$type> for Vect3 {
            fn add_assign(&mut self, rhs: $type) {
                self.x += rhs.x;
                self.y += rhs.y;
                self.z += rhs.z;
            }
        }

        impl SubAssign<$type> for Vect3 {
            fn sub_assign(&mut self, rhs: $type) {
                self.x -= rhs.x;
                self.y -= rhs.y;
                self.z -= rhs.z;
            }
        }

        impl Div<f64> for $type {
            type Output = Vect3;

            fn div(self, scalar: f64) -> Self::Output {
                Vect3::new(self.x / scalar, self.y / scalar, self.z / scalar)
            }
        }

        impl Mul<f64> for $type {
            type Output = Vect3;

            fn mul(self, scalar: f64) -> Self::Output {
                Vect3::new(self.x * scalar, self.y * scalar, self.z * scalar)
            }
        }

        impl Neg for $type {
            type Output = Vect3;

            fn neg(self) -> Self::Output {
                Vect3::new(-self.x, -self.y, -self.z)
            }
        }
    };
    ( $type:ty, $type2:ty ) => {
        impl Add<$type2> for $type {
            type Output = Vect3;

            fn add(self, other: $type2) -> Self::Output {
                Vect3::new(self.x + other.x, self.y + other.y, self.z + other.z)
            }
        }

        impl Mul<$type2> for $type {
            type Output = f64;

            fn mul(self, other: $type2) -> f64 {
                self.x * other.x + self.y * other.y + self.z * other.z
            }
        }

        impl Sub<$type2> for $type {
            type Output = Vect3;

            fn sub(self, other: $type2) -> Self::Output {
                Vect3::new(self.x - other.x, self.y - other.y, self.z - other.z)
            }
        }
    };
}

helper!(Vect3, Vect3);
helper!(&Vect3, Vect3);
helper!(Vect3, &Vect3);
helper!(&Vect3, &Vect3);
helper!(Vect3);
helper!(&Vect3);

pub fn radius_to_mass(r: f64, rho: f64) -> f64 {
    rho * r * r * r * 4. / 3. * std::f64::consts::PI
}
