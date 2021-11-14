use crate::kdtree::Tree;
// use crate::kdpoint::PhysicsPoint3D;
use crate::kdpoint_ref::{create_tree, system};
use std::fs::File;
use std::io::Write;


// use std::f64::consts::PI;

/**
 * Assuming acceleration is velocity independent
 */
pub fn kick_step_once(tree: &Tree<i32>, h: f64) {

    //let mut acc = Vec::new();
    //acc.resize(particle_count, Vect3::ZERO);

    // const h: f64 = 0.1;
    // let mut t = t_0;

    unsafe {
        for (i, p) in system.iter().enumerate() {
            let a = tree.compute_acceleration(&(i as i32));
            a.println();
            system[i].vel = system[i].vel + a * h;
        }
    
        for (i, p) in system.iter().enumerate() {
            system[i].pos = system[i].pos + system[i].vel * h;
        }
    }

    // t += h;

    // vector<Vect3>
}

pub fn integrate(h: f64, end: f64, info: &mut File) {
    let sys = unsafe {
        &system
    };

    let mut tree = create_tree(sys);
    tree.printer();

    println!("end = {:.3}", end);
    let mut t = 0.;
    while t < end {
        println!("t = {:.3}", t);
        kick_step_once(&tree, h);
        tree = create_tree(sys);
        t += h;

        write!(info, "[");
        for i in 0..sys.len() {
            match write!(info, "[{:.e}, {:.e}, {:.e}]", sys[i].pos.x, sys[i].pos.y, sys[i].pos.z) {
                Err(why) => panic!("couldn't write: {}", why),
                Ok(_) => (),
            }
            if i < sys.len() - 1 {
                write!(info, ",");
            }
        }
        write!(info, "]\n");
    }
}