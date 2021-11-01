use crate::kdtree::Tree;
use crate::kdpoint::PhysicsPoint3D;
use crate::util::Vect3;

pub fn euler(tree: &Tree<PhysicsPoint3D>, particle_count: usize) {
    let mut acc = Vec::new();
    acc.resize(particle_count, Vect3::ZERO);

    

    // vector<Vect3>
}