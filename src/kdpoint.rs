use crate::util::Vect3; 

// #[derive(Debug, PartialEq, Copy, Clone)]
// pub enum Dimensions {
//     X,
//     Y,
//     Z,
//     VX,
//     VY,
//     VZ
// }

// pub trait KDPoint: Copy {
//     fn spread_in_dim(data: &[Self], dim: &Dimensions) -> f64;
//     // fn compute_median_in_dim(&mut self, dim: Dimensions) -> f64;
//     // fn split_on_dim(data: &mut [Self], dim: Dimensions) -> (&mut [Self], &mut [Self], f64);

//     fn cmp_on_dim(&self, other: &Self, dim: &Dimensions) -> std::cmp::Ordering;

//     fn get_value_in_dim(data: &[Self], index: usize, dim: &Dimensions) -> f64;

//     fn compute_com(data: &[Self]) -> Vect3;

//     fn get_point(&self) -> Vect3;

//     fn print(&self);

//     const ZERO: Self;

//     fn all_axis() -> Vec<Dimensions>;

//     fn get_radius(&self) -> f64;

//     fn get_mass(&self) -> f64;

//     /**
//      * Compute the acceleration that self will feel from other
//      */
//     fn compute_acceleration_from(&self, other: &Self) -> Vect3;

//     fn compute_acceleration_from_node(&self, other: &crate::kdtree::Node<Self>) -> Vect3;

//     // compute center of mass
//     // get max radius in node

//     // for later -> (not this week)
//     // implement for i32 or usize where that is the index to a global vec
//     // Particle -> 8 doubles (x, y, z, vx, vy, vz, m, r)
// }

/*
impl KDPoint for f64 {
    fn spread_in_dim(data: &[Self], dim: &Dimensions) -> f64 {
        if dim != &Dimensions::X {
            panic!("This tree is 1D");
        }
        else {
            let min = data.iter().fold(f64::INFINITY, |a, &b| if a < b { a } else { b });
            let max = data.iter().fold(f64::NEG_INFINITY, |a, &b| if a > b { a } else { b });
            max - min
        }
    }

    fn get_value_in_dim(data: &[Self], index: usize, dim: &Dimensions) -> f64 {
        if dim != &Dimensions::X {
            panic!("This tree is 1D");
        }
        else {
            data[index]
        }
    }

    fn compute_com(data: &[Self]) -> (f64, f64, f64) {
        let center: f64 = data.iter().sum::<f64>() / data.len() as f64; // no mass specified so just avg
        (center, 0., 0.)
    }

    fn cmp_on_dim(&self, other: &Self, dim: &Dimensions) -> std::cmp::Ordering {
        if dim != &Dimensions::X {
            panic!("This tree is 1D");
        }
        else if self < other {
            std::cmp::Ordering::Less
        }
        else if self > other {
            std::cmp::Ordering::Greater
        }
        else {
            std::cmp::Ordering::Equal
        }
    }

    fn print(&self) {
        print!("{:0>5.2}", self);
    }

    const ZERO: Self = 0.;
    fn all_axis() -> Vec<Dimensions> { vec!(Dimensions::X) }

    fn get_mass(&self) -> f64 { 0. }
    fn get_radius(&self) -> f64 { 0. }

    
}

impl KDPoint for (f64, f64) {
    fn spread_in_dim(data: &[Self], dim: &Dimensions) -> f64 {
        if dim == &Dimensions::X {
            let min = data.iter().fold(f64::INFINITY, |a, &(b, _)| if a < b { a } else { b });
            let max = data.iter().fold(f64::NEG_INFINITY, |a, &(b, _)| if a > b { a } else { b });
            max - min
        }
        else if dim == &Dimensions::Y {
            let min = data.iter().fold(f64::INFINITY, |a, &(_, b)| if a < b { a } else { b });
            let max = data.iter().fold(f64::NEG_INFINITY, |a, &(_, b)| if a > b { a } else { b });
            max - min
        }
        else {
            panic!("This tree is 2D");
        }
    }

    fn get_value_in_dim(data: &[Self], index: usize, dim: &Dimensions) -> f64 {
        if dim == &Dimensions::X { // make this a switch
            let (x, _) = data[index];
            x
        }
        else if dim == &Dimensions::Y {
            let (_, y) = data[index];
            y
        }
        else {
            panic!("This tree is 2D");
        }
    }

    fn compute_com(data: &[Self]) -> (f64, f64, f64) {
        let center_x: f64 = data.iter().fold(0., |acc, (x,_)| acc + x) / data.len() as f64; // no mass specified so just avg
        let center_y: f64 = data.iter().fold(0., |acc, (_,y)| acc + y) / data.len() as f64;
        (center_x, center_y, 0.)
    }

    fn cmp_on_dim(&self, other: &Self, dim: &Dimensions) -> std::cmp::Ordering {
        let (self_val, other_val) = if dim == &Dimensions::X {
            let (s, _) = self;
            let (o, _) = other;
            (s, o)
        }
        else if dim == &Dimensions::Y {
            let (_, s) = self;
            let (_, o) = other;
            (s, o)
        }
        else {
            panic!("This tree is 2D");
        };
        
        if self_val < other_val {
            std::cmp::Ordering::Less
        }
        else if self_val > other_val {
            std::cmp::Ordering::Greater
        }
        else {
            std::cmp::Ordering::Equal
        }
    }

    fn print(&self) {
        print!("<{:0>5.2} {:0>5.2}>", self.0, self.1);
    }

    const ZERO: Self = (0., 0.);
    fn all_axis() -> Vec<Dimensions> { vec!(Dimensions::X, Dimensions::Y) }

    fn get_mass(&self) -> f64 { 0. }
    fn get_radius(&self) -> f64 { 0. }
}
*/

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PhysicsPoint3D {
    pub pos: Vect3,
    pub vel: Vect3,
    pub m: f64,
    pub r: f64
}

use rand::Rng;

impl PhysicsPoint3D {
    pub fn new(x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64, m: f64, r: f64) -> PhysicsPoint3D {
        PhysicsPoint3D { pos: Vect3{x, y, z}, vel: Vect3{x: vx, y: vy, z: vz}, m, r }
    }

    pub fn from_random(center: Vect3, max_offset: f64, max_vel_offset: f64, m: f64, r: f64) -> Self {
        let mut rng = rand::thread_rng();

        let x = center + Vect3::new(
            rng.gen_range(-max_offset..max_offset), 
            rng.gen_range(-max_offset..max_offset), 
            rng.gen_range(-max_offset..max_offset));
        let v = Vect3::new(
            rng.gen_range(-max_vel_offset..max_vel_offset),
            rng.gen_range(-max_offset..max_offset),
            rng.gen_range(-max_offset..max_offset));

        PhysicsPoint3D { pos: x, vel: v, m, r }
    }

    pub fn from_random_2d(center: Vect3, max_offset: f64, max_vel_offset: f64, m: f64, r: f64) -> Self {
        let mut rng = rand::thread_rng();

        let x = center + Vect3::new(
            rng.gen_range(-max_offset..max_offset), 
            rng.gen_range(-max_offset..max_offset), 
            0.);
        let v = Vect3::new(
            rng.gen_range(-max_vel_offset..max_vel_offset),
            rng.gen_range(-max_offset..max_offset),
            0.);

        PhysicsPoint3D { pos: x, vel: v, m, r }
    }
}

// impl KDPoint for PhysicsPoint3D {
//     fn get_point(&self) -> Vect3 {
//         self.pos
//     }

//     fn spread_in_dim(data: &[Self], dim: &Dimensions) -> f64 {
//         let selector: fn(&PhysicsPoint3D) -> f64 = match dim {
//             Dimensions::X => {
//                 |pt| pt.pos.x
//             },
//             Dimensions::Y => {
//                 |pt| pt.pos.y
//             },
//             Dimensions::Z => {
//                 |pt| pt.pos.z
//             },
//             _ => panic!("This Tree is 3D")
//         };

//         let min = data.iter().map(selector).fold(f64::INFINITY, |a, b| if a < b { a } else { b });
//         let max = data.iter().map(selector).fold(f64::NEG_INFINITY, |a, b| if a > b { a } else { b });
//         max - min
//     }

//     fn cmp_on_dim(&self, other: &Self, dim: &Dimensions) -> std::cmp::Ordering {
//         let (s, o) = match dim {
//             Dimensions::X => (self.pos.x, other.pos.x),
//             Dimensions::Y => (self.pos.y, other.pos.y),
//             Dimensions::Z => (self.pos.z, other.pos.z),
//             _ => panic!("This Tree is 3D")
//         };

//         if s < o {
//             std::cmp::Ordering::Less
//         }
//         else if s > o {
//             std::cmp::Ordering::Greater
//         }
//         else {
//             std::cmp::Ordering::Equal
//         }
//     }

//     fn get_value_in_dim(data: &[Self], index: usize, dim: &Dimensions) -> f64 {
//         match dim {
//             Dimensions::X => data[index].pos.x,
//             Dimensions::Y => data[index].pos.y,
//             Dimensions::Z => data[index].pos.z,
//             _ => panic!("This Tree is 3D")
//         }
//     }

//     fn compute_com(data: &[Self]) -> Vect3 {
//         let total_mass: f64 = data.iter().map(|pt| pt.m).sum();
//         let avg_x = data.iter().map(|pt| pt.pos.x * pt.m).sum::<f64>() / total_mass;
//         let avg_y = data.iter().map(|pt| pt.pos.y * pt.m).sum::<f64>() / total_mass;
//         let avg_z = data.iter().map(|pt| pt.pos.z * pt.m).sum::<f64>() / total_mass;

//         Vect3::new(avg_x, avg_y, avg_z)
//     }

//     fn print(&self) {
//         print!("<{:0>5.2} {:0>5.2} {:0>5.2} {:0>5.2} {:0>5.2} {:0>5.2}>", self.pos.x, self.pos.y, self.pos.z, self.vel.x, self.vel.y, self.vel.z);
//     }

//     const ZERO: Self = PhysicsPoint3D { pos: Vect3::ZERO, vel: Vect3::ZERO, m: 0., r: 0. };

//     fn all_axis() -> Vec<Dimensions> {
//         vec![Dimensions::X, Dimensions::Y, Dimensions::Z]
//     }

//     fn get_radius(&self) -> f64 {
//         self.r
//     }

//     fn get_mass(&self) -> f64 {
//         self.m
//     }

//     fn compute_acceleration_from_node(&self, other: &crate::kdtree::Node<Self>) -> Vect3 {
//         // F = G m1 m2 / r^2
//         // a m1 = G m1 m2 / r^2
//         // a = G m2 / r^2
//         const G: f64 = 1.;

//         let r = self.pos - other.com;
//         // let r = (sq(self.x - other.x) + sq(self.y - other.y) + sq(self.z - other.z)).sqrt();

//         let mag = -G * other.mass / r.mag_sq();

//         let r_hat = r.norm();
//         // let point_vec = ((other.x - self.x) / r, (other.y - self.y) / r, (other.z - self.z) / r);

//         // Vect3::new(point_vec.0 * mag, point_vec.1 * mag, point_vec.2 * mag)
//         r_hat * mag
//     }

//     fn compute_acceleration_from(&self, other: &Self) -> Vect3 {
//         // F = G m1 m2 / r^2
//         // a m1 = G m1 m2 / r^2
//         // a = G m2 / r^2
//         const G: f64 = 1.;

//         if self.pos == other.pos {
//             // if a bunch of doubles are exactly the same, then self is other
//             Vect3::ZERO // particles are not attracted to themselves
//         }
//         else {
//             let r = self.pos - other.pos;
//             // let r = (sq(self.x - other.x) + sq(self.y - other.y) + sq(self.z - other.z)).sqrt();

//             let mag = -G * other.m / r.mag_sq();

//             let r_hat = r.norm();
//             // let point_vec = ((other.x - self.x) / r, (other.y - self.y) / r, (other.z - self.z) / r);

//             // Vect3::new(point_vec.0 * mag, point_vec.1 * mag, point_vec.2 * mag)
//             r_hat * mag
//         }
//     }
// }
