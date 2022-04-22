// Forces
use crate::util::Vect3;
use crate::PhysicsPoint3D;

pub trait GlobalForce {
    // a force just depedent on one particle, like a global field
    fn force(&self, p: &PhysicsPoint3D) -> Vect3;
    fn potential(&self, p: &PhysicsPoint3D) -> f64;
}

pub trait PairwiseSymmetricForce {
    fn acceleration(&self, p: &PhysicsPoint3D, other: &PhysicsPoint3D) -> (Vect3, Vect3);

    fn force(&self, p: &PhysicsPoint3D, other: &PhysicsPoint3D) -> (Vect3, Vect3) {
        let (ai, aj) = self.acceleration(p, other);
        // F = ma
        (ai * p.m, aj * other.m)
    }
}


/// A well with a linear slope to its center
/// U(r) = ar for r > 0
/// F(r) = -dU/dr = -a * |r_vect - center|
/// r = |r_vect - center|
/// dimensional analysis [Force] = -a * r hat => a = [Force] = N
/// so coeff is Force
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

    fn potential(&self, p: &PhysicsPoint3D) -> f64 {
        // F = - coeff (x - center) / |x - center|
        // F = - grad U
        // coeff (x - center) / |x - center| = grad U
        // U(x) = coeff |x - center|
        self.coeff * (p.pos - self.center).mag()
    }
}

/// A Damped Spring repulsion force between two particles
pub struct DampedSpring {
    k: f64,
    b: f64
}

impl DampedSpring {
    pub fn new(k: f64, b: f64) -> Self {
        Self { k, b }
    }

    pub fn potential(&self, p1: &PhysicsPoint3D, p2: &PhysicsPoint3D) -> f64 {
        // F = -kx
        // U = 1/2 kx^2

        // x is distance between
        let r = (p2.pos - p1.pos).mag();
        let delta = r - (p1.r + p2.r);

        if delta < 0. {
            0.5 * self.k * delta * delta
        }
        else {
            0.
        }
    }
}

impl PairwiseSymmetricForce for DampedSpring {

    fn acceleration(&self, p: &crate::kdpoint::PhysicsPoint3D, other: &crate::kdpoint::PhysicsPoint3D) -> (Vect3, Vect3) {
        let rji = other.pos - p.pos; // relative pos
        let vji = other.vel - p.vel; // relative vel

        let r = rji.mag();
        let delta = r - (p.r + other.r);

        if delta < 0. {
            let x_hat = rji / r;

            // collision
            let f_spring = x_hat * -self.k * delta;
            let f_damp = x_hat * (vji * x_hat) * -self.b;

            // println!("{}, {}", f_spring.mag(), f_damp.mag());

            let f_total = f_spring + f_damp;

            // F = m_i * a_i
            // a_i = F / m_i
            let ai = f_total / p.m * -1.;
            let aj = f_total / other.m;

            (ai, aj)
        }
        else {
            (Vect3::ZERO, Vect3::ZERO)
        }
    }
}
