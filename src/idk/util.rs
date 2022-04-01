// use std::ops::Deref;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Time(f64);

impl Deref for Time {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<f64> for Time {
    fn from(f: f64) -> Self {
        assert!(f.is_finite(), "time should be real");
        Self(f)
    }
}

impl From<Time> for f64 {
    fn from(t: Time) -> Self {
        t.0
    }
}

impl From<Reverse<Time>> for Time {
    fn from(f: Reverse<Time>) -> Self {
        f.0
    }
}

impl From<Time> for Reverse<Time> {
    fn from(t: Time) -> Self {
        Reverse(t)
    }
}

impl Eq for Time {}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        PartialOrd::partial_cmp(self, other).expect("What???")
    }
}



#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Index(usize);

impl Deref for Index {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<usize> for Index {
    fn from(i: usize) -> Self {
        Self(i)
    }
}

impl From<Index> for usize {
    fn from(i: Index) -> usize {
        i.0
    }
}

// impl Deref for Index {
//     type Target = usize;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

use std::{cmp::Reverse, ops::Deref};
