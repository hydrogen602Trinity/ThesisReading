// mod kdtree;
// mod kdpoint;
// mod util;
// mod integrate;
// mod kdpoint_ref;

// use std::fs::File;
// use std::io::prelude::*;
// use std::path::Path;

// use kdpoint::PhysicsPoint3D;
// // use kdtree::Tree;
// use kdpoint_ref::{print_sys, system};
// use integrate::integrate;
// // use kdpoint::KDPoint;

// fn main() {    
//     // let mut x: [f64; 50] = [26.21, 9.78, 26.18, 27.38, 8.59, 16.97, 0.31, 18.57, 21.27, 28.89, 28.43, 25.99, 6.74, 18.69, 0.3, 2.72, 14.48, 12.52, 9.49, 7.1, 15.57, 22.23, 17.85, 23.63, 26.12, 10.17, 0.81, 2.42, 0.98, 24.66, 28.93, 11.32, 3.19, 25.29, 6.91, 28.45, 4.41, 12.77, 24.46, 3.34, 4.54, 11.03, 27.08, 2.67, 22.7, 11.12, 14.25, 27.21, 18.1, 17.26];
//     // let t: Tree<f64> = Tree::new(&mut x[..]);

//     // t.printer();

//     // let mut x: [(f64, f64); 50] = [(16.22, 28.89), (2.3, 17.65), (26.87, 18.13), (26.54, 24.77), (14.91, 19.45), (22.49, 21.28), (6.46, 22.47), (0.17, 8.67), (13.99, 20.54), (28.91, 26.66), (1.0, 10.96), (7.54, 13.18), (28.74, 6.75), (0.28, 2.38), (20.56, 11.96), (7.79, 27.25), (16.88, 24.45), (18.77, 16.37), (24.34, 6.16), (29.02, 16.8), (16.83, 2.16), (9.02, 17.43), (9.28, 11.82), (2.01, 26.56), (4.88, 4.14), (15.18, 26.25), (29.19, 11.07), (9.43, 0.92), (23.88, 28.61), (3.17, 11.37), (9.4, 5.13), (13.57, 1.42), (22.38, 23.65), (19.97, 26.91), (8.6, 27.78), (24.46, 29.37), (9.4, 1.9), (22.33, 19.65), (4.71, 16.8), (16.45, 8.56), (25.35, 4.79), (23.0, 1.64), (24.41, 19.23), (5.53, 28.91), (17.18, 26.02), (19.62, 14.32), (16.21, 22.34), (6.04, 29.46), (25.99, 24.85), (22.83, 29.53)];
//     // let t: Tree<(f64, f64)> = Tree::new(&mut x[..]);

//     // let points: [PhysicsPoint3D; 2] = [PhysicsPoint3D::new(0., 0., 0., 0., 0., 0., 1., 0.), PhysicsPoint3D::new(1., 0., 0., 0., 1., 0., 1e-7, 0.)];
//     // let t = Tree::new(&mut points[..]);
//     let h: f64 = 1e-3;

//     // for &p in points.() {
//     //     println!("push {}", system.len());
//     //     system.push(p);
//     // }

//     let path = Path::new("position.txt");
//     let display = path.display();

//     let mut file = match File::create(&path) {
//         Err(why) => panic!("couldn't create {}: {}", display, why),
//         Ok(file) => file,
//     };

//     unsafe {
//         system.push(PhysicsPoint3D::new(0., 0., 0., 0., 0., 0., 1., 0.));
//         system.push(PhysicsPoint3D::new(1., 0., 0., 0., 1., 0., 1e-7, 0.));

//         println!("idk {}", system.len());

//         print_sys(&system);

//         integrate(h, std::f64::consts::PI, &mut file); //, &mut system);

//         print_sys(&system);
//     }
// }

use std::{path::Path, fs::File, io::Write};

use idk::{LinearWell, KickStep, Integrator, forces::DampedSpring, no_explode};
use kdpoint::PhysicsPoint3D;
use util::Vect3;

use crate::{idk::{Setup, KickStepPQCollision}, util::radius_to_mass};

mod idk;
mod kdpoint;
mod util;

fn main() {

    // test();
    // return;

    let c = Vect3::ZERO;

    const RHO: f64 = 0.88;

    let r = 1e-7;
    let particles: Vec<PhysicsPoint3D> = (0..40).map(|_| PhysicsPoint3D::from_random_2d(c, 15. * r, 2. * r, radius_to_mass(r, RHO), r)).collect();

    // mass / 2 cause reduced mass
    let (b, k) = no_explode::compute::b_and_k(0.00016, radius_to_mass(r, RHO), r);
    println!("k = {:e}, b = {:e}", k, b);

    let k_force = DampedSpring::new(k, b); //(1., 0.1);

    let dt = 0.0001;
    let mut setup = idk::Setup::new(
        LinearWell::new(Vect3::ZERO, 1e-21), 
        k_force, 
        particles, 
        dt);

    let logger = {
        let path = Path::new("pipe");
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        let path2 = Path::new("angular.csv");
        let display2 = path.display();

        let mut file2 = match File::create(&path2) {
            Err(why) => panic!("couldn't create {}: {}", display2, why),
            Ok(file) => file,
        };

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

            let vect = setup.angular_momentum();
            println!("{}, {}, {}", vect.x, vect.y, vect.z);
        }
    };

    KickStepPQCollision::simulate(&mut setup, 1000., Some(logger));
}


fn test() {
    const RHO: f64 = 0.88;

    let r = 1e-7;
    let particles: Vec<PhysicsPoint3D> = vec![
        PhysicsPoint3D::new(1.5e-7, 0., 0., 0., 0., 0., radius_to_mass(r, RHO), r),
        PhysicsPoint3D::new(-1.5e-7, 0., 0., 0., 0., 0., radius_to_mass(r, RHO), r)
    ];

    // mass / 2 cause reduced mass

    // 0.0000008655284142600487
    // 2.3e-4
    // 0.00016
    let (b, k) = no_explode::compute::b_and_k(0.00016, radius_to_mass(r, RHO), r);
    println!("k = {:e}, b = {:e}", k, b);

    let k_force = DampedSpring::new(k, b); //(1., 0.1);

    // 0.0001 works with KickStepPQCollision, but not KickStep
    let dt = 0.0001; //0.0001;
    let mut setup = idk::Setup::new(
        LinearWell::new(Vect3::ZERO, 1e-21), 
        k_force, 
        particles, 
        dt);

    let logger = {
        let path = Path::new("pipe");
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        let mut file2 = match File::create("data.out") {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        move |setup: &Setup<_, _>| {
            let p = setup.get_particles();
            for (i, elem) in p.iter().enumerate() {
                if i > 0 {
                    write!(file, "|").expect("");
                    write!(file2, ",").unwrap();
                }

                write!(file, "{},{},{}", elem.pos.x, elem.pos.y, elem.pos.z).expect("");
                write!(file2, "{},{},{}", elem.pos.x, elem.pos.y, elem.pos.z).expect("");
            }
            writeln!(file, "").expect("");
            writeln!(file2, "").expect("");
        }
    };

    // 1000
    //KickStep::simulate(&mut setup, 0.005, Some(logger));
    KickStepPQCollision::simulate(&mut setup, 0.002, Some(logger));

}


