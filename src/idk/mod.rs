use crate::{kdpoint::PhysicsPoint3D, util::Vect3};
use std::{iter::zip, cmp::Reverse};

pub mod forces;
pub mod no_explode;
mod util;
pub use forces::*;
use priority_queue::PriorityQueue;

use self::util::{Index, Time};


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

        //println!("x={}, vx={}", setup.sys.particles[0].pos.x, setup.sys.particles[0].vel.x);
    
        // step -> integrate x
    
        for p in &mut setup.sys.particles {
            p.pos += p.vel * setup.dt;
        }
    }
}


pub struct KickStepPQCollision();

impl Integrator for KickStepPQCollision {
    fn step<F: GlobalForce, S: PairwiseSymmetricForce>(setup: &mut Setup<F, S>) {
        let glob_acc = setup.compute_global_acceleration();

        let sub_dt = setup.dt / 100.;

        let mut pq: PriorityQueue<(Index, Index), Reverse<Time>> = PriorityQueue::new();

        let mut last_updated_rel = glob_acc.iter().map(|_| 0.).collect::<Vec<_>>();

        // step velocities for global force
        for (p, a) in zip(&mut setup.sys.particles, glob_acc) {
            p.vel += a * setup.dt;
        }

        // find collisions
        for (i, ei) in setup.sys.particles.iter().enumerate() {
            for (j, ej) in setup.sys.particles.iter().enumerate().skip(i+1) {
                let max_distance = ei.vel.mag() * setup.dt + ej.vel.mag() * setup.dt;
                if (ei.pos - ej.pos).mag() - ei.r - ej.r < max_distance * 1.5 {
                    // could collide?

                    let t: Time = sub_dt.into();
                    pq.push((i.into(), j.into()), t.into());
                }
            }
        }

        
        loop {
            let ((i, j), mut t): ((Index, Index), Time) = match pq.pop() {
                Some((indices, time)) => (indices, time.into()),
                None => break
            };

            // if t is beyond the timestep, set it to the timestep
            // and say over_extends_end = true
            let over_extends_end = if *t > setup.dt {
                t = setup.dt.into();
                true
            }
            else {
                false
            };

            // particles need to know last updated!
            let (last_t_i, last_t_j) = (last_updated_rel[*i], last_updated_rel[*j]);

            // determine and integrate pairwise force
            let (ai, aj) = setup.pairwise_force.acceleration(
                &setup.sys.particles[*i], 
                &setup.sys.particles[*j]);

            setup.sys.particles[*i].vel += ai * (*t - last_t_i);
            setup.sys.particles[*j].vel += aj * (*t - last_t_j);

            // update to current time
            let vi = setup.sys.particles[*i].vel;
            setup.sys.particles[*i].pos += vi * (*t - last_t_i);

            let vj = setup.sys.particles[*j].vel;
            setup.sys.particles[*j].pos += vj * (*t - last_t_j);

            last_updated_rel[*i] = *t;
            last_updated_rel[*j] = *t;

            // when to push to pq again? - if not at end
            if !over_extends_end {
                let t2: Time = (*t + sub_dt).into();
                pq.push((i, j), t2.into());
            }
        }

        for (i, last_t) in last_updated_rel.into_iter().enumerate() {
            let v = setup.sys.particles[i].vel;
            setup.sys.particles[i].pos += v * (setup.dt - last_t);
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

    /// For forces that don't depend on other particles/ won't change much
    fn compute_global_acceleration(&self) -> Vec<Vect3> {
        self.sys.particles.iter().map(|p| self.global_force.force(p) / p.m).collect()
    }

    fn compute_accelerations(&self) -> Vec<Vect3> {
        // global forces
        let mut acc: Vec<Vect3> = self.sys.particles.iter().map(|p| self.global_force.force(p) / p.m).collect();
        // println!("{:?}", acc);

        for i in 0..(self.sys.particles.len()) {
            for j in (i+1)..(self.sys.particles.len()) {

                let (ai, aj) = self.pairwise_force.acceleration(&self.sys.particles[i], &self.sys.particles[j]);

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

