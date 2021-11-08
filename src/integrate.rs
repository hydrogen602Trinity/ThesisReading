use crate::kdtree::Tree;
use crate::kdpoint::PhysicsPoint3D;
use crate::kdpoint_ref::{create_tree};

use std::f64::consts::PI;

/**
 * Assuming acceleration is velocity independent
 */
pub fn kick_step_once(tree: &Tree<i32>, h: f64) {
    let system = systemState.lock().unwrap();

    //let mut acc = Vec::new();
    //acc.resize(particle_count, Vect3::ZERO);

    // const h: f64 = 0.1;
    // let mut t = t_0;

    for (i, p) in system.iter().enumerate() {
        let a = tree.compute_acceleration(&(i as i32));
        system[i].vel = a * h;
    }

    for (i, p) in system.iter().enumerate() {
        system[i].pos = p.pos * h;
    }

    // t += h;

    // vector<Vect3>
}

pub fn integrate(h: f64, end: f64) {

    let mut tree = create_tree();

    let mut t = 0.;
    while t < end {
        kick_step_once(&tree, h);
        tree = create_tree();
        t += h;
    }
}