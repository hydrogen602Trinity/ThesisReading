use super::kdpoint::{PhysicsPoint3D, KDPoint};
use std::iter::Iterator;

// pub static mut system: Vec<PhysicsPoint3D> = Vec::new();

pub fn print_sys<'a, T: IntoIterator<Item=&'a PhysicsPoint3D>>(sys: T) { //Iterator<Item=PhysicsPoint3D>
    for p in sys {
        p.print();
        println!();
    }
}
