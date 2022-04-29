use std::fmt::Debug;

use crate::{
    idk::{forces, Setup},
    kdpoint::PhysicsPoint3D,
    vect3::Vect3,
};

pub trait ApproxEq: Debug {
    fn percent_diff(&self, other: &Self) -> f64;

    /// Compute percent difference and check if less than max_percent_diff
    /// max_percent_diff is out of 1, i.e. 0.05 = 5%
    fn approx_eq(&self, other: &Self, max_percent_diff: f64) -> bool {
        let d = Self::percent_diff(self, other);
        d.abs() < max_percent_diff
    }

    fn assert_approx_eq(&self, other: &Self, max_percent_diff: f64) {
        let d = Self::percent_diff(self, other).abs();

        assert!(
            d < max_percent_diff,
            "{:?} != {:?}, differs by {}%",
            self,
            other,
            d * 100.
        );
    }
}

impl ApproxEq for f64 {
    fn percent_diff(&self, other: &Self) -> f64 {
        (self - other) / self
    }
}

impl ApproxEq for Vect3 {
    fn percent_diff(&self, other: &Self) -> f64 {
        (self - other).mag() / self.mag()
    }
}

pub struct PreTestState {
    pub particles: Vec<PhysicsPoint3D>,
    pub energy: f64,
    pub com: Vect3,
    pub ang_momentum: Vect3,
}

impl PreTestState {
    pub fn new<F>(s: &Setup<F, forces::DampedSpring>) -> Self
    where
        F: forces::GlobalForce,
    {
        PreTestState {
            particles: s.get_particles().clone(),
            energy: s.compute_energy(),
            com: s.center_of_mass(),
            ang_momentum: s.angular_momentum(),
        }
    }

    pub fn particle_cmp<F>(
        &self,
        f: impl Fn(&PhysicsPoint3D, &PhysicsPoint3D),
        post: &Setup<F, forces::DampedSpring>,
    ) -> usize
    where
        F: forces::GlobalForce,
    {
        self.particles
            .iter()
            .zip(post.get_particles().iter())
            .map(|(p1, p2)| f(p1, p2))
            .count()
    }
}
