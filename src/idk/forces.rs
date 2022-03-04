use crate::util::Vect3;
use super::PairwiseSymmetricForce;

pub struct DampedSpring {
    k: f64,
    b: f64
}

impl DampedSpring {
    pub fn new(k: f64, b: f64) -> Self {
        Self { k, b }
    }
}

impl PairwiseSymmetricForce for DampedSpring {

    fn force(&self, p: &crate::kdpoint::PhysicsPoint3D, other: &crate::kdpoint::PhysicsPoint3D) -> (Vect3, Vect3) {
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
