use crate::kdtree::Tree;
use crate::kdpoint::{PhysicsPoint3D, KDPoint};
use super::util::Vect3;
// use crate::kdpoint_ref::{create_tree, system};
use std::fs::File;
use std::io::Write;


// use std::f64::consts::PI;

/// Compute all accelerations
pub fn compute_acceleration(tree: &Tree<PhysicsPoint3D>, system: &Vec<PhysicsPoint3D>, acc: &mut Vec<Vect3>) {
    for (i, pt) in system.iter().enumerate() {
        acc[i] = tree.compute_acceleration(pt);
    }
}

/// The function needs to have ownership of the tree
/// as system cannot be mutated until the tree is destroyed
pub fn kick_step_once(h: f64, system: &mut Vec<PhysicsPoint3D>, acc: &Vec<Vect3>) {
    // const h: f64 = 0.1;
    // let mut t = t_0;

    for (i, p) in system.iter_mut().enumerate() {
        p.vel = p.vel + acc[i] * h;
        p.pos = p.pos + p.vel * h;
    }
}

pub fn integrate(h: f64, end: f64, info: &mut File, system: &mut Vec<PhysicsPoint3D>) {
    let mut acc = Vec::new();
    acc.resize(system.len(), Vect3::ZERO);

    // let mut tree = Tree::new(system);
    // tree.printer();


    println!("end = {:.3}", end);
    let mut t = 0.;
    while t < end {
        let tree = Tree::new(system);
        // println!("t = {:.3}", t);
        compute_acceleration(&tree, &system, &mut acc);

        kick_step_once(h, system, &acc);
        
        t += h;

        write!(info, "[").unwrap();
        for i in 0..system.len() {
            match write!(info, "[{:.e}, {:.e}, {:.e}]", system[i].pos.x, system[i].pos.y, system[i].pos.z) {
                Err(why) => panic!("couldn't write: {}", why),
                Ok(_) => (),
            }
            if i < system.len() - 1 {
                write!(info, ",").unwrap();
            }
        }
        write!(info, "]\n").unwrap();

        // tree = Tree::new(system);
    }
}