use std::ops::{Add, Div, Mul, Sub, Neg};


#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vect3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
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

    pub const ZERO: Vect3 = Vect3 { x: 0., y: 0., z: 0. };
}



// #[macro_export]
macro_rules! helper {
    ( $type:ty ) => {
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
