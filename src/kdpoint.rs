use crate::util::Vect3; 

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Dimensions3 {
    X,
    Y,
    Z
}

pub trait KDPoint: Copy {
    fn spread_in_dim(data: &[Self], dim: &Dimensions3) -> f64;
    // fn compute_median_in_dim(&mut self, dim: Dimensions3) -> f64;
    // fn split_on_dim(data: &mut [Self], dim: Dimensions3) -> (&mut [Self], &mut [Self], f64);

    fn cmp_on_dim(&self, other: &Self, dim: &Dimensions3) -> std::cmp::Ordering;

    fn get_value_in_dim(data: &[Self], index: usize, dim: &Dimensions3) -> f64;

    fn compute_com(data: &[Self]) -> Vect3;

    fn get_point(&self) -> Vect3;

    fn print(&self);

    const ZERO: Self;

    fn all_axis() -> Vec<Dimensions3>;

    fn get_radius(&self) -> f64;

    fn get_mass(&self) -> f64;

    /**
     * Compute the acceleration that self will feel from other
     */
    fn compute_acceleration_from(&self, other: &Self) -> Vect3;

    fn compute_acceleration_from_node(&self, other: &crate::kdtree::Node<Self>) -> Vect3;

    // compute center of mass
    // get max radius in node

    // for later -> (not this week)
    // implement for i32 or usize where that is the index to a global vec
    // Particle -> 8 doubles (x, y, z, vx, vy, vz, m, r)
}



#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PhysicsPoint3D {
    pub pos: Vect3,
    pub vel: Vect3,
    pub m: f64,
    pub r: f64
}

impl PhysicsPoint3D {
    pub fn new(x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64, m: f64, r: f64) -> PhysicsPoint3D {
        PhysicsPoint3D { pos: Vect3{x, y, z}, vel: Vect3{x: vx, y: vy, z: vz}, m, r }
    }
}

impl KDPoint for PhysicsPoint3D {
    fn get_point(&self) -> Vect3 {
        self.pos
    }

    fn spread_in_dim(data: &[Self], dim: &Dimensions3) -> f64 {
        let selector: fn(&Self) -> f64 = match dim {
            Dimensions3::X => {
                |pt| pt.pos.x
            },
            Dimensions3::Y => {
                |pt| pt.pos.y
            },
            Dimensions3::Z => {
                |pt| pt.pos.z
            }
        };

        let min = data.iter().map(selector).fold(f64::INFINITY, |a, b| if a < b { a } else { b });
        let max = data.iter().map(selector).fold(f64::NEG_INFINITY, |a, b| if a > b { a } else { b });
        max - min
    }

    fn cmp_on_dim(&self, other: &Self, dim: &Dimensions3) -> std::cmp::Ordering {
        let self_data = self.pos;
        let other_data = other.pos;
        let (s, o) = match dim {
            Dimensions3::X => (self_data.x, other_data.x),
            Dimensions3::Y => (self_data.y, other_data.y),
            Dimensions3::Z => (self_data.z, other_data.z)
        };

        if s < o {
            std::cmp::Ordering::Less
        }
        else if s > o {
            std::cmp::Ordering::Greater
        }
        else {
            std::cmp::Ordering::Equal
        }
    }

    fn get_value_in_dim(data: &[Self], index: usize, dim: &Dimensions3) -> f64 {
        let pos_data = &data[index].pos;
        match dim {
            Dimensions3::X => pos_data.x,
            Dimensions3::Y => pos_data.y,
            Dimensions3::Z => pos_data.z
        }
    }

    fn compute_com(data: &[Self]) -> Vect3 {
        let total_mass: f64 = data.iter().map(|pt| pt.m).sum();
        let avg_x = data.iter().map(|pt| pt.pos.x * pt.m).sum::<f64>() / total_mass;
        let avg_y = data.iter().map(|pt| pt.pos.y * pt.m).sum::<f64>() / total_mass;
        let avg_z = data.iter().map(|pt| pt.pos.z * pt.m).sum::<f64>() / total_mass;

        Vect3::new(avg_x, avg_y, avg_z)
    }

    fn print(&self) {
        print!("<{:0>5.2} {:0>5.2} {:0>5.2} {:0>5.2} {:0>5.2} {:0>5.2}>", self.pos.x, self.pos.y, self.pos.z, self.vel.x, self.vel.y, self.vel.z);
    }

    const ZERO: Self = PhysicsPoint3D { pos: Vect3::ZERO, vel: Vect3::ZERO, m: 0., r: 0. };

    fn all_axis() -> Vec<Dimensions3> {
        vec![Dimensions3::X, Dimensions3::Y, Dimensions3::Z]
    }

    fn get_radius(&self) -> f64 {
        self.r
    }

    fn get_mass(&self) -> f64 {
        self.m
    }

    fn compute_acceleration_from_node(&self, other: &super::kdtree::Node<Self>) -> Vect3 {
        const G: f64 = 1.;
        
        let r = self.pos - other.com;
        // let r = (sq(self.x - other.x) + sq(self.y - other.y) + sq(self.z - other.z)).sqrt();

        let mag = -G * other.mass / r.mag_sq();

        let r_hat = r.norm();
        // let point_vec = ((other.x - self.x) / r, (other.y - self.y) / r, (other.z - self.z) / r);

        // Vect3::new(point_vec.0 * mag, point_vec.1 * mag, point_vec.2 * mag)
        r_hat * mag
    }

    fn compute_acceleration_from(&self, other: &Self) -> Vect3 {
        // F = G m1 m2 / r^2
        // a m1 = G m1 m2 / r^2
        // a = G m2 / r^2
        const G: f64 = 1.;

        if self.pos == other.pos {
            // if a bunch of doubles are exactly the same, then self is other
            Vect3::ZERO // particles are not attracted to themselves
        }
        else {
            let r = self.pos - other.pos;
            // let r = (sq(self.x - other.x) + sq(self.y - other.y) + sq(self.z - other.z)).sqrt();

            let mag = -G * other.m / r.mag_sq();

            let r_hat = r.norm();
            // let point_vec = ((other.x - self.x) / r, (other.y - self.y) / r, (other.z - self.z) / r);

            // Vect3::new(point_vec.0 * mag, point_vec.1 * mag, point_vec.2 * mag)
            r_hat * mag
        }
    }
}