use crate::{kdpoint::PhysicsPoint3D, util::Vect3};
use std::iter::zip;

pub mod forces;
pub mod no_explode;
pub use forces::*;


// Integrators

pub trait Integrator {
    fn step<F: GlobalForce, S: PairwiseSymmetricForce>(setup: &mut Setup<F, S>);

    fn simulate<F, S, L>(setup: &mut Setup<F, S>, period: f64, mut logger: Option<L>) 
        where F: GlobalForce, S: PairwiseSymmetricForce, L: FnMut(&Setup<F,S>) 
    {
        let mut t = 0.;

        while t < period {
            t += setup.dt;

            // println!("{:?}", setup.get_particles()[0]);

            Self::step(setup);

            if let Some(ref mut log) = logger {
                log(setup);
            }
        }

    } 
}

// kick-step

pub struct KickStep();

impl Integrator for KickStep {
    fn step<F: GlobalForce, S: PairwiseSymmetricForce>(setup: &mut Setup<F, S>) {
        let acc = setup.compute_accelerations();

        for (p, a) in zip(&mut setup.sys.particles, acc) {
            p.vel += a * setup.dt;
        }
    
        // step -> integrate x
    
        for p in &mut setup.sys.particles {
            p.pos += p.vel * setup.dt;
        }
    }
}


// System

pub struct Setup<F: GlobalForce, S: PairwiseSymmetricForce> {
    global_force: F,
    pairwise_force: S,
    sys: System,
    dt: f64
}

impl<F: GlobalForce, S: PairwiseSymmetricForce> Setup<F, S> {
    pub fn new(global_force: F, pairwise_force: S, particles: Vec<PhysicsPoint3D>, dt: f64) -> Self { Setup { global_force, pairwise_force, sys: System::new(particles), dt } }

    pub fn get_particles(&self) -> &Vec<PhysicsPoint3D> {
        &self.sys.particles
    }
}

impl<F: GlobalForce, S: PairwiseSymmetricForce> Setup<F, S> {
    fn compute_accelerations(&self) -> Vec<Vect3> {
        // global forces
        let mut acc: Vec<Vect3> = self.sys.particles.iter().map(|p| self.global_force.force(p) / p.m).collect();
        // println!("{:?}", acc);

        for i in 0..(self.sys.particles.len()) {
            for j in (i+1)..(self.sys.particles.len()) {
                
                let (ai, aj) = self.pairwise_force.force(&self.sys.particles[i], &self.sys.particles[j]);

                acc[i] += ai;
                acc[j] += aj;
            }
        }

        acc
    }
}

struct System {
    particles: Vec<PhysicsPoint3D>
}

impl System {
    pub fn new(particles: Vec<PhysicsPoint3D>) -> Self { System { particles } }
}

