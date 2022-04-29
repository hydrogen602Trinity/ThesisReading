use crate::vect3::Vect3;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PhysicsPoint3D {
    pub pos: Vect3,
    pub vel: Vect3,
    pub m: f64,
    pub r: f64,
}

use rand::Rng;

impl PhysicsPoint3D {
    pub fn new(
        x: f64,
        y: f64,
        z: f64,
        vx: f64,
        vy: f64,
        vz: f64,
        m: f64,
        r: f64,
    ) -> PhysicsPoint3D {
        PhysicsPoint3D {
            pos: Vect3 { x, y, z },
            vel: Vect3 {
                x: vx,
                y: vy,
                z: vz,
            },
            m,
            r,
        }
    }

    pub fn kinetic_energy(&self) -> f64 {
        // 1/2 mv^2
        self.vel * self.vel * self.m / 2.
    }

    pub fn from_random(
        center: Vect3,
        max_offset: f64,
        max_vel_offset: f64,
        m: f64,
        r: f64,
    ) -> Self {
        let mut rng = rand::thread_rng();

        let x = center
            + Vect3::new(
                rng.gen_range(-max_offset..max_offset),
                rng.gen_range(-max_offset..max_offset),
                rng.gen_range(-max_offset..max_offset),
            );
        let v = Vect3::new(
            rng.gen_range(-max_vel_offset..max_vel_offset),
            rng.gen_range(-max_offset..max_offset),
            rng.gen_range(-max_offset..max_offset),
        );

        PhysicsPoint3D {
            pos: x,
            vel: v,
            m,
            r,
        }
    }

    pub fn from_random_2d(
        center: Vect3,
        max_offset: f64,
        max_vel_offset: f64,
        m: f64,
        r: f64,
    ) -> Self {
        let mut rng = rand::thread_rng();

        let x = center
            + Vect3::new(
                rng.gen_range(-max_offset..max_offset),
                rng.gen_range(-max_offset..max_offset),
                0.,
            );
        let v = Vect3::new(
            rng.gen_range(-max_vel_offset..max_vel_offset),
            rng.gen_range(-max_offset..max_offset),
            0.,
        );

        PhysicsPoint3D {
            pos: x,
            vel: v,
            m,
            r,
        }
    }
}
