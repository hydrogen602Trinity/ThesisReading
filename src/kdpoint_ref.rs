use crate::kdpoint::*;
use crate::util::Vect3;

const system: Vec<PhysicsPoint3D> = Vec::new();

// fn index(i: &i32) -> &PhysicsPoint3D {
//     if i < &0 {
//         &PhysicsPoint3D::ZERO
//     }
//     else {
//         // &system[*i as usize]
//         // &system[*i as usize]
//         let x = system.get(*i as usize).unwrap();
//         x
//         // system.get(*i as usize).unwrap()
//     }
// }

macro_rules! index {
    ( $i:expr ) => {
        if ($i) < &0 {
            &PhysicsPoint3D::ZERO
        }
        else {
            system.get(*($i) as usize).unwrap()
        }
    }
}

fn pos(i: &i32) -> &Vect3 {
    if i < &0 {
        &PhysicsPoint3D::ZERO.pos
    }
    else {
        &system[*i as usize].pos
    }
}

impl KDPoint for i32 {
    fn spread_in_dim(data: &[Self], dim: &Dimensions) -> f64 {
        let selector: fn(&i32) -> f64 = match dim {
            Dimensions::X => {
                |pt| pos(pt).x
            },
            Dimensions::Y => {
                |pt| pos(pt).y
            },
            Dimensions::Z => {
                |pt| pos(pt).z
            },
            _ => panic!("This Tree is 3D")
        };

        let min = data.iter().map(selector).fold(f64::INFINITY, |a, b| if a < b { a } else { b });
        let max = data.iter().map(selector).fold(f64::NEG_INFINITY, |a, b| if a > b { a } else { b });
        max - min
    }

    fn cmp_on_dim(&self, other: &Self, dim: &Dimensions) -> std::cmp::Ordering {
        let self_data = pos(self);
        let other_data = pos(other);
        let (s, o) = match dim {
            Dimensions::X => (self_data.x, other_data.x),
            Dimensions::Y => (self_data.y, other_data.y),
            Dimensions::Z => (self_data.z, other_data.z),
            _ => panic!("This Tree is 3D")
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

    fn get_value_in_dim(data: &[Self], index: usize, dim: &Dimensions) -> f64 {
        let pos_data = pos(&data[index]);
        match dim {
            Dimensions::X => pos_data.x,
            Dimensions::Y => pos_data.y,
            Dimensions::Z => pos_data.z,
            _ => panic!("This Tree is 3D")
        }
    }

    fn compute_com(data: &[Self]) -> Vect3 {
        let total_mass: f64 = data.iter().map(|pt| index!(pt).m).sum();
        let avg_x = data.iter().map(|pt| pos(pt).x * index!(pt).m).sum::<f64>() / total_mass;
        let avg_y = data.iter().map(|pt| pos(pt).y * index!(pt).m).sum::<f64>() / total_mass;
        let avg_z = data.iter().map(|pt| pos(pt).z * index!(pt).m).sum::<f64>() / total_mass;

        Vect3::new(avg_x, avg_y, avg_z)
    }

    fn print(&self) {
        let data = index!(self);
        print!("<{:0>5.2} {:0>5.2} {:0>5.2} {:0>5.2} {:0>5.2} {:0>5.2}>", data.pos.x, data.pos.y, data.pos.z, data.vel.x, data.vel.y, data.vel.z);
    }

    const ZERO: Self = -1;

    fn all_axis() -> Vec<Dimensions> {
        vec![Dimensions::X, Dimensions::Y, Dimensions::Z]
    }

    fn get_radius(&self) -> f64 {
        index!(self).r
    }

    fn get_mass(&self) -> f64 {
        index!(self).m
    }

    fn compute_acceleration_from(&self, other: &Self) -> Vect3 {
        // F = G m1 m2 / r^2
        // a m1 = G m1 m2 / r^2
        // a = G m2 / r^2
        const G: f64 = 1.;

        let self_pt = index!(self);
        let other_pt = index!(other);

        if self_pt.pos == other_pt.pos {
            // if a bunch of doubles are exactly the same, then self is other
            Vect3::ZERO // particles are not attracted to themselves
        }
        else {
            let sq = |x| x*x;

            let r = (self_pt.pos - other_pt.pos).mag();
            // let r = (sq(self.x - other.x) + sq(self.y - other.y) + sq(self.z - other.z)).sqrt();

            let mag = G * other_pt.m / sq(r);

            let point_vec = (other_pt.pos - self_pt.pos) / r;
            // let point_vec = ((other.x - self.x) / r, (other.y - self.y) / r, (other.z - self.z) / r);

            // Vect3::new(point_vec.0 * mag, point_vec.1 * mag, point_vec.2 * mag)
            point_vec * mag
        }
    }
}