use std::{fs::File, io::Write, path::Path};

use idk::{forces::DampedSpring, no_explode, Integrator, LinearWell};
use kdpoint::PhysicsPoint3D;
use vect3::Vect3;

use crate::{
    idk::{KickStepPQCollision, Setup},
    vect3::radius_to_mass,
};

mod idk;
mod kdpoint;
mod util;
mod vect3;

fn main() {
    const RHO: f64 = 0.88;

    let r = 1e-7;
    //let particles: Vec<PhysicsPoint3D> = (0..40).map(|_| PhysicsPoint3D::from_random_2d(c, 15. * r, 2. * r, radius_to_mass(r, RHO), r)).collect();
    let particles = vec![
        PhysicsPoint3D::new(10. * r, 1. * r, 0., 0., 0., 0., radius_to_mass(r, RHO), r),
        PhysicsPoint3D::new(-10. * r, -1. * r, 0., 0., 0., 0., radius_to_mass(r, RHO), r),
    ];

    // mass / 2 cause reduced mass
    let (b, k) = no_explode::lewis::b_and_k(0.0001, radius_to_mass(r, RHO), r);
    println!("k = {:e}, b = {:e}", k, b);

    let k_force = DampedSpring::new(k, b); //b); //(1., 0.1);

    let dt = 0.00001; // 0.0001
    let mut setup = idk::Setup::new(LinearWell::new(Vect3::ZERO, 1e-21), k_force, particles, dt);

    let logger = {
        fn open_file(path: &str) -> File {
            let path = Path::new(path);
            match File::create(&path) {
                Err(why) => panic!("couldn't create {}: {}", path.display(), why),
                Ok(file) => file,
            }
        }

        let mut file = open_file("pipe");
        let mut file2 = open_file("angular.csv");
        let mut file4 = open_file("full_data.csv");

        move |setup: &Setup<_, _>| {
            let p = setup.get_particles();
            for (i, elem) in p.iter().enumerate() {
                if i > 0 {
                    write!(file, "|").expect("");
                }

                write!(file, "{},{},{}", elem.pos.x, elem.pos.y, elem.pos.z).expect("");
            }
            writeln!(file, "").expect("");

            let ang = setup.angular_momentum();
            writeln!(file2, "{}, {}, {}", ang.x, ang.y, ang.z).unwrap();

            let p = setup.get_particles();

            write!(
                file4,
                "{}, {}, {}, {}, {}, {}, ",
                p[0].pos.x, p[0].pos.y, p[0].pos.z, p[1].pos.x, p[1].pos.y, p[1].pos.z
            )
            .unwrap();

            write!(
                file4,
                "{}, {}, {}, {}, {}, {}, ",
                p[0].vel.x, p[0].vel.y, p[0].vel.z, p[1].vel.x, p[1].vel.y, p[1].vel.z
            )
            .unwrap();

            writeln!(
                file4,
                "{}, {}, {}, {}",
                setup.compute_energy(),
                setup.compute_global_u(),
                setup.compute_pairwise_u(),
                setup.compute_ke()
            )
            .unwrap();

            // let vect = setup.angular_momentum();
            // println!("{}, {}, {}", vect.x, vect.y, vect.z);
        }
    };

    KickStepPQCollision::simulate(&mut setup, 1000., Some(logger));
}

#[cfg(test)]
mod tests {
    use crate::{
        idk::{self, forces, no_explode, DampedSpring, Integrator, KickStepPQCollision},
        kdpoint::PhysicsPoint3D,
        util::{ApproxEq, PreTestState},
        vect3::{radius_to_mass, Vect3},
    };
    const RHO: f64 = 0.88;
    const R: f64 = 1e-7;

    #[test]
    fn test_linear_x_axis() {
        //let particles: Vec<PhysicsPoint3D> = (0..40).map(|_| PhysicsPoint3D::from_random_2d(c, 15. * r, 2. * r, radius_to_mass(r, RHO), r)).collect();
        let particles = vec![
            PhysicsPoint3D::new(10. * R, 0., 0., -R, 0., 0., radius_to_mass(R, RHO), R),
            PhysicsPoint3D::new(-10. * R, 0., 0., R, 0., 0., radius_to_mass(R, RHO), R),
        ];

        // x = v * t
        // x / v = t

        let t = ((10. * R - R) / R).abs(); // since it collides when x = r (nonzero radius), subtract that one

        let (_b, k) = no_explode::compute::b_and_k(0.0001, radius_to_mass(R, RHO), R);

        let k_force = DampedSpring::new(k, 0.); // energy should be preserved

        let dt = 0.0001; // 0.0001
        let mut setup = idk::Setup::new(forces::NullForce::default(), k_force, particles, dt);

        let pre_setup = PreTestState::new(&setup);

        assert_eq!(
            setup.center_of_mass(),
            Vect3::ZERO,
            "COM is nonzero, but system is symmetric"
        );
        assert_eq!(
            setup.angular_momentum(),
            Vect3::ZERO,
            "Angular momentum should be zero"
        );

        KickStepPQCollision::simulate(&mut setup, t, None::<fn(&_)>);

        assert_eq!(setup.center_of_mass(), Vect3::ZERO, "COM moved");
        assert_eq!(
            setup.angular_momentum(),
            Vect3::ZERO,
            "Angular momentum changed"
        );

        assert_eq!(
            pre_setup.particle_cmp(
                |pre, post| {
                    assert_eq!(post.pos.y, 0., "Drifted off x-axis");
                    assert_eq!(post.pos.z, 0., "Drifted off x-axis");
                    assert!(
                        pre.pos.x * post.pos.x > 0.,
                        "Particles went through each other"
                    );
                },
                &setup
            ),
            2
        );

        let post_e = setup.compute_energy();
        ApproxEq::assert_approx_eq(&pre_setup.energy, &post_e, 0.001);
    }
}
