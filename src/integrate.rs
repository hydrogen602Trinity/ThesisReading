use crate::kdtree::Tree;
use crate::kdpoint::PhysicsPoint3D;
use crate::util::Vect3;
use crate::kdpoint_ref::system;

/**
 * Assuming acceleration is velocity independent
 */
pub fn kick_step(tree: &Tree<PhysicsPoint3D>, t_0: f64, t_f: f64) {
    //let mut acc = Vec::new();
    //acc.resize(particle_count, Vect3::ZERO);

    const h: f64 = 0.1;
    let mut t = t_0;

    while t < t_f {
        for (i, p) in system.iter().enumerate() {
            let a = tree.compute_acceleration(p);
            system[i].vel = a * h;
        }
    
        for (i, p) in system.iter().enumerate() {
            system[i].pos = system[i].pos * h;
        }

        t += h;
    }

    // vector<Vect3>
}