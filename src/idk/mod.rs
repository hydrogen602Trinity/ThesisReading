use crate::{kdpoint::PhysicsPoint3D, util::Vect3};
use std::{iter::zip, fs::File, path::Path};
use std::io::Write;


// Forces

pub trait GlobalForce {
    // a force just depedent on one particle, like a global field
    fn force(&self, p: &PhysicsPoint3D) -> Vect3;
}

/// A well with a linear slope to its center
/// U(r) = ar for r > 0
/// F(r) = -dU/dr = -a * |r_vect - center|
/// r = |r_vect - center|
/// dimensional analysis [Force] = -a * [length] => a = [Force/length] = N/m
/// so coeff is Force/length
pub struct LinearWell { center: Vect3, coeff: f64 }

impl LinearWell {
    pub fn new(center: Vect3, coeff: f64) -> Self {
        Self { center, coeff }
    }
}

impl GlobalForce for LinearWell {
    fn force(&self, p: &PhysicsPoint3D) -> Vect3 {
        let dir = -(p.pos - self.center).norm();
        dir * self.coeff
    }
}

// Integrators

pub trait Integrator {
    fn step<F: GlobalForce>(setup: &mut Setup<F>);

    fn simulate<F: GlobalForce>(setup: &mut Setup<F>, period: f64) {
        let mut t = 0.;

        let path = Path::new("pipe");
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        while t < period {
            t += setup.dt;

            Self::step(setup);

            let p = setup.get_particles();
            for (i, elem) in p.iter().enumerate() {
                if i > 0 {
                    write!(file, "|").expect("");
                }

                write!(file, "{},{},{}", elem.pos.x, elem.pos.y, elem.pos.z).expect("");
            }
            writeln!(file, "").expect("");
        }

    } 
}

// kick-step

pub struct KickStep();

impl Integrator for KickStep {
    fn step<F: GlobalForce>(setup: &mut Setup<F>) {
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

pub struct Setup<F: GlobalForce> {
    global_force: F,
    sys: System,
    dt: f64
}

impl<F: GlobalForce> Setup<F> {
    pub fn new(global_force: F, particles: Vec<PhysicsPoint3D>, dt: f64) -> Self { Setup { global_force, sys: System::new(particles), dt } }

    pub fn get_particles(&self) -> &Vec<PhysicsPoint3D> {
        &self.sys.particles
    }
}

impl<F: GlobalForce> Setup<F> {
    fn compute_accelerations(&self) -> Vec<Vect3> {
        // global forces
        self.sys.particles.iter().map(|p| self.global_force.force(p) / p.m).collect()
    }
}

struct System {
    particles: Vec<PhysicsPoint3D>
}

impl System {
    pub fn new(particles: Vec<PhysicsPoint3D>) -> Self { System { particles } }
}

