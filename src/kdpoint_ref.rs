use crate::kdpoint::*;
use crate::kdtree::Tree;
use crate::util::Vect3;


pub static mut system: Vec<PhysicsPoint3D> = Vec::new();
// pub const system: Vec<PhysicsPoint3D> = Vec::new();
// static mut systemState: Mutex<Vec<PhysicsPoint3D>> = Mutex::new(Vec::new()); //: Box<Vec<PhysicsPoint3D>> = 


fn create_index(sys: &Vec<PhysicsPoint3D>) -> Vec<i32> {
    let mut v = Vec::new();
    v.resize(sys.len(), 0);
    for i in 0..sys.len() {
        v[i] = i as i32;
    }
    v
}

pub fn create_tree(sys: &Vec<PhysicsPoint3D>) -> Tree<i32> {
    let mut c = create_index(sys);
    Tree::new(&mut c)
}

pub fn print_sys(sys: &Vec<PhysicsPoint3D>) {
    for p in sys.iter() {
        p.print();
        println!();
    }
}


fn index(i: &i32) -> PhysicsPoint3D {
    if i < &0 {
        PhysicsPoint3D::ZERO
    }
    else {
        unsafe {
            system[*i as usize]
        }
    }
}

fn pos(i: &i32) -> Vect3 {
    if i < &0 {
        PhysicsPoint3D::ZERO.pos
    }
    else {
        unsafe {
            system[*i as usize].pos
        }
    }
}

impl KDPoint for i32 {
    fn get_point(&self) -> Vect3 {
        unsafe {
            system[*self as usize].pos
        }
    }

    fn spread_in_dim(data: &[Self], dim: &Dimensions3) -> f64 {


        let selector: fn(&i32) -> f64 = match dim {
            Dimensions3::X => {
                |pt| pos(pt).x
            },
            Dimensions3::Y => {
                |pt| pos(pt).y
            },
            Dimensions3::Z => {
                |pt| pos(pt).z
            },
            _ => panic!("This Tree is 3D")
        };

        let min = data.iter().map(selector).fold(f64::INFINITY, |a, b| if a < b { a } else { b });
        let max = data.iter().map(selector).fold(f64::NEG_INFINITY, |a, b| if a > b { a } else { b });
        max - min
    }

    fn cmp_on_dim(&self, other: &Self, dim: &Dimensions3) -> std::cmp::Ordering {
        let self_data = pos(self);
        let other_data = pos(other);
        let (s, o) = match dim {
            Dimensions3::X => (self_data.x, other_data.x),
            Dimensions3::Y => (self_data.y, other_data.y),
            Dimensions3::Z => (self_data.z, other_data.z),
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

    fn get_value_in_dim(data: &[Self], index: usize, dim: &Dimensions3) -> f64 {
        let pos_data = pos(&data[index]);
        match dim {
            Dimensions3::X => pos_data.x,
            Dimensions3::Y => pos_data.y,
            Dimensions3::Z => pos_data.z,
            _ => panic!("This Tree is 3D")
        }
    }

    fn compute_com(data: &[Self]) -> Vect3 {
        let total_mass: f64 = data.iter().map(|pt| index(pt).m).sum();
        let avg_x = data.iter().map(|pt| pos(pt).x * index(pt).m).sum::<f64>() / total_mass;
        let avg_y = data.iter().map(|pt| pos(pt).y * index(pt).m).sum::<f64>() / total_mass;
        let avg_z = data.iter().map(|pt| pos(pt).z * index(pt).m).sum::<f64>() / total_mass;

        Vect3::new(avg_x, avg_y, avg_z)
    }

    fn print(&self) {
        let data = index(self);
        print!("<{:0>5.2} {:0>5.2} {:0>5.2} {:0>5.2} {:0>5.2} {:0>5.2}>", data.pos.x, data.pos.y, data.pos.z, data.vel.x, data.vel.y, data.vel.z);
    }

    const ZERO: Self = -1;

    fn all_axis() -> Vec<Dimensions3> {
        vec![Dimensions3::X, Dimensions3::Y, Dimensions3::Z]
    }

    fn get_radius(&self) -> f64 {
        index(self).r
    }

    fn get_mass(&self) -> f64 {
        index(self).m
    }

    fn compute_acceleration_from_node(&self, other: &crate::kdtree::Node<Self>) -> Vect3 {
        const G: f64 = 1.;
        
        let self_pt = index(self);
        let r = self_pt.pos - other.com;
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

        let self_pt = index(self);
        let other_pt = index(other);

        if self_pt.pos == other_pt.pos {
            // if a bunch of doubles are exactly the same, then self is other
            Vect3::ZERO // particles are not attracted to themselves
        }
        else {
            let r = self_pt.pos - other_pt.pos;
            // let r = (sq(self.x - other.x) + sq(self.y - other.y) + sq(self.z - other.z)).sqrt();

            let mag = -G * other_pt.m / r.mag_sq();

            let r_hat = r.norm();
            // let point_vec = ((other.x - self.x) / r, (other.y - self.y) / r, (other.z - self.z) / r);

            // Vect3::new(point_vec.0 * mag, point_vec.1 * mag, point_vec.2 * mag)
            r_hat * mag
        }
    }
}

