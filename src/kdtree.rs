use crate::kdpoint::{Dimensions, KDPoint};

const NODE_VALUE_COUNT: usize = 8;//13;

// const DIMS: [Dimensions; 6] = [Dimensions::X, Dimensions::Y, Dimensions::Z, Dimensions::VX, Dimensions::VY, Dimensions::VZ];

pub struct Node<T> {
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    split_direction: Dimensions,
    split_value: f64,
    values: [T; NODE_VALUE_COUNT],
    value_count: i32,
    max_radius: f64,
    com: (f64, f64, f64),
    mass: f64
    // store max radius and com + its loc
}

pub struct Tree<T: KDPoint> {
    pub root: Option<Box<Node<T>>>,
    pub dims: Vec<Dimensions>
}

impl<T: KDPoint> Tree<T> {
    pub fn new(points: &mut [T]) -> Tree<T> {
        let dims = T::all_axis();
        let root = Tree::construct_level(&dims, points);
        Tree { root: root, dims: dims }
    }

    fn construct_level(dims: &Vec<Dimensions>, points: &mut[T]) -> Option<Box<Node<T>>> {
        let len = points.len();
        if len == 0 {
            return None;
        }
        
        

        if len <= NODE_VALUE_COUNT {
            let mut values: [T; NODE_VALUE_COUNT] = [T::ZERO; NODE_VALUE_COUNT];
            let slice = &mut values[..len];
            slice.copy_from_slice(points);
            let max_radius = values.iter().map(|x| x.get_radius()).fold(0., |m, a| if a.get_radius() > m { a } else { m });
            let mass: f64 = values.iter().map(|x| x.get_mass()).sum();
            Some(Box::new(Node { 
                left: None, 
                right: None, 
                split_direction: Dimensions::X, 
                split_value: 0.,
                values: values,
                value_count: len as i32,
                max_radius: max_radius,
                com: KDPoint::compute_com(points),
                mass: mass
            }))
        }
        else {
            let split_direction = *{
                let mut best_split_direction = &dims[0];
                let mut best_value = 0.;
    
                for d in dims {
                    let x = T::spread_in_dim(points, d);
                    if x > best_value {
                        best_value = x;
                        best_split_direction = d;
                    }
                }
    
                best_split_direction
            };

            let values: [T; NODE_VALUE_COUNT] = [T::ZERO; NODE_VALUE_COUNT];
            points.sort_unstable_by(|a, b| a.cmp_on_dim(b, &split_direction));

            let max_radius = points.iter().map(|x| x.get_radius()).fold(0., |m, a| if a.get_radius() > m { a } else { m });
            let mass: f64 = points.iter().map(|x| x.get_mass()).sum();
            
            let median = T::get_value_in_dim(points, len/2, &split_direction);  //points[(points.len()/2)];
            
            let (first, second) = points.split_at_mut(len/2);
            let left = Tree::construct_level(dims, first);
            let right = Tree::construct_level(dims, second);

            Some(Box::new(Node { 
                left: left, 
                right: right, 
                split_direction: split_direction, 
                split_value: median,
                values: values,
                value_count: 0,
                max_radius: max_radius,
                com: KDPoint::compute_com(points),
                mass: mass
            }))
        }
    }

    pub fn printer(&self) -> () {
        Tree::printer_helper(match &self.root {
            Some(e) => e,
            None => panic!("oi")
        }, Some(0));
    }

    fn printer_helper(n: &Node<T>, depth: Option<i32>) -> () {
        let tabs = match depth {
            None => 0,
            Some(e) => e
        };

        if n.value_count == 0 {
            match &n.left {
                None => (),
                Some(x) => Tree::printer_helper(x, Some(tabs+1))
            };
            for _ in 0..tabs {
                print!("  ");
            }
            println!("{:?} = {}", n.split_direction, n.split_value);
            match &n.right {
                None => (),
                Some(x) => Tree::printer_helper(x, Some(tabs+1))
            };
        }
        else {
            for _ in 0..tabs {
                print!("  ");
            }
            print!("points = ");
            let mut i = 0;
            for v in &n.values {
                if i >= n.value_count {
                    break;
                }
                v.print();
                print!(", ");
                i += 1;
            }
            println!("");
        }

    }
}
