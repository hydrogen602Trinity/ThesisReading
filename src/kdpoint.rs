
#[derive(Debug, PartialEq)]
pub enum Dimensions {
    X,
    Y,
    Z,
    VX,
    VY,
    VZ
}

pub trait KDPoint: Copy {
    fn spread_in_dim(data: &[Self], dim: &Dimensions) -> f64;
    // fn compute_median_in_dim(&mut self, dim: Dimensions) -> f64;
    // fn split_on_dim(data: &mut [Self], dim: Dimensions) -> (&mut [Self], &mut [Self], f64);

    fn cmp_on_dim(&self, other: &Self, dim: &Dimensions) -> std::cmp::Ordering;

    fn get_value_in_dim(data: &[Self], index: usize, dim: &Dimensions) -> f64;

    fn print(&self);

    const ZERO: Self;
}

impl KDPoint for f64 {
    fn spread_in_dim(data: &[Self], dim: &Dimensions) -> f64 {
        if dim != &Dimensions::X {
            panic!("This tree is 1D");
        }
        else {
            let min = data.iter().fold(f64::INFINITY, |a, &b| if a < b { a } else { b });
            let max = data.iter().fold(f64::NEG_INFINITY, |a, &b| if a > b { a } else { b });
            max - min
        }
    }

    fn get_value_in_dim(data: &[Self], index: usize, dim: &Dimensions) -> f64 {
        if dim != &Dimensions::X {
            panic!("This tree is 1D");
        }
        else {
            data[index]
        }
    }

    fn cmp_on_dim(&self, other: &Self, dim: &Dimensions) -> std::cmp::Ordering {
        if dim != &Dimensions::X {
            panic!("This tree is 1D");
        }
        else if self < other {
            std::cmp::Ordering::Less
        }
        else if self > other {
            std::cmp::Ordering::Greater
        }
        else {
            std::cmp::Ordering::Equal
        }
    }

    fn print(&self) {
        if self < &10. {
            print!("0");
        }
        print!("{:0.2}, ", self);
    }

    const ZERO: Self = 0.;
}
