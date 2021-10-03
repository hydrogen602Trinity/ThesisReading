
#[derive(Debug, PartialEq, Copy, Clone)]
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

    fn all_axis() -> Vec<Dimensions>;
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
        print!("{:0>5.2}", self);
    }

    const ZERO: Self = 0.;
    fn all_axis() -> Vec<Dimensions> { vec!(Dimensions::X) }
}

impl KDPoint for (f64, f64) {
    fn spread_in_dim(data: &[Self], dim: &Dimensions) -> f64 {
        if dim == &Dimensions::X {
            let min = data.iter().fold(f64::INFINITY, |a, &(b, _)| if a < b { a } else { b });
            let max = data.iter().fold(f64::NEG_INFINITY, |a, &(b, _)| if a > b { a } else { b });
            max - min
        }
        else if dim == &Dimensions::Y {
            let min = data.iter().fold(f64::INFINITY, |a, &(_, b)| if a < b { a } else { b });
            let max = data.iter().fold(f64::NEG_INFINITY, |a, &(_, b)| if a > b { a } else { b });
            max - min
        }
        else {
            panic!("This tree is 2D");
        }
    }

    fn get_value_in_dim(data: &[Self], index: usize, dim: &Dimensions) -> f64 {
        if dim == &Dimensions::X {
            let (x, _) = data[index];
            x
        }
        else if dim == &Dimensions::Y {
            let (_, y) = data[index];
            y
        }
        else {
            panic!("This tree is 2D");
        }
    }

    fn cmp_on_dim(&self, other: &Self, dim: &Dimensions) -> std::cmp::Ordering {
        let (self_val, other_val) = if dim == &Dimensions::X {
            let (s, _) = self;
            let (o, _) = other;
            (s, o)
        }
        else if dim == &Dimensions::Y {
            let (_, s) = self;
            let (_, o) = other;
            (s, o)
        }
        else {
            panic!("This tree is 2D");
        };
        
        if self_val < other_val {
            std::cmp::Ordering::Less
        }
        else if self_val > other_val {
            std::cmp::Ordering::Greater
        }
        else {
            std::cmp::Ordering::Equal
        }
    }

    fn print(&self) {
        print!("<{:0>5.2} {:0>5.2}>", self.0, self.1);
    }

    const ZERO: Self = (0., 0.);
    fn all_axis() -> Vec<Dimensions> { vec!(Dimensions::X, Dimensions::Y) }
}
