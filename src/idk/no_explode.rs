
/* 
 * 
 *  d2x/dt2 + beta * dx/dt + omega'^2 * x = 0
 * 
 *  beta = b/m
 *  
 *  omega'^2 = k/m
 */

// for r, v_0 = 1e-7, rho = 0.88
// b = 3.4194745456729856e-20, k = 6.595530918688126e-18
//pub const B: f64 = 3.4194745456729856e-20;
//pub const K: f64 = 6.595530918688126e-18;

pub const PEN_RATIO_DEFAULT: f64 = 0.02;
const COEFF_RES: f64 = 0.3;


pub mod compute {
    use std::f64::consts::PI;
    use super::{COEFF_RES, PEN_RATIO_DEFAULT};

    pub fn beta2(v_0: f64, pen_depth: f64) -> f64 {
        ( - v_0 * 2. * COEFF_RES.ln() * COEFF_RES.sqrt() ) / ( pen_depth * PI )
    }
    
    pub fn omega_0_sq(beta_val: f64) -> f64 {
        let LN_COEFF_RES_SQ = COEFF_RES.ln() * COEFF_RES.ln();
        ( beta_val * beta_val * (LN_COEFF_RES_SQ + PI * PI) ) / ( 4. * LN_COEFF_RES_SQ )
    }

    // returns (b, k)
    pub fn b_and_k2(v_0: f64, m: f64, pen_depth: f64) -> (f64, f64) {
        let beta_val = beta2(v_0, pen_depth);
        let omega_0_sq_val = omega_0_sq(beta_val);

        (beta_val * m, omega_0_sq_val * m)
    }

    // returns (b, k)
    pub fn b_and_k(v_0: f64, m: f64, radius: f64) -> (f64, f64) {
        b_and_k2(v_0, m, radius * PEN_RATIO_DEFAULT)
    }
}


pub mod lewis {
    use std::f64::consts::PI;
    use super::{COEFF_RES, PEN_RATIO_DEFAULT};

    pub fn k(m: f64, v_i: f64, r: f64) -> f64 {
        let delta_r = PEN_RATIO_DEFAULT * r;
        m * v_i * v_i / (delta_r * delta_r)
    }

    pub fn c(k: f64, m: f64) -> f64 {
        2. * (k * m).sqrt() * COEFF_RES.ln() / PI
    }

    pub fn b_and_k(v_0: f64, m: f64, radius: f64) -> (f64, f64) {
        let k = k(m, v_0, radius);
        let c = c(k, m).abs();
        (c, k)
    }
}

pub mod schwartz {
    use std::f64::consts::PI;
    use super::COEFF_RES;

    const CONST_OF_PROP: f64 = 1.; // TODO: what is this?
    // I think its about 1 as it works for the example given
    const MAX_PEN_RATIO: f64 = 0.02;

    pub fn k(m: f64, v_max: f64, x_max: f64) -> f64 {
        let tmp = v_max / x_max;
        m * CONST_OF_PROP * tmp * tmp
    }

    pub fn c(k: f64, m: f64) -> f64 {
        let lne = COEFF_RES.ln();
        -2. * (k * m / (PI * PI + lne * lne)).sqrt() * lne
    }

    pub fn b_and_k(v_max: f64, m: f64, radius: f64) -> (f64, f64) {
        let k = k(m, v_max, radius * MAX_PEN_RATIO).abs();
        let c = c(k, m).abs();
        (c, k)
    }
}
