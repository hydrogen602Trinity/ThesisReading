use crate::kdpoint::{Dimensions3, KDPoint};
use crate::util::Vect3;

const NODE_VALUE_COUNT: usize = 8;//13;
const OPENING_ANGLE_RATIO: f64 = 0.01; // about tan(0.5 degree)
// OPENING_ANGLE_RATIO = size / distance

// const DIMS: [Dimensions3; 6] = [Dimensions3::X, Dimensions3::Y, Dimensions3::Z, Dimensions3::VX, Dimensions3::VY, Dimensions3::VZ];

pub struct Node<'a, T> {
    pub left: Option<Box<Node<'a, T>>>,
    pub right: Option<Box<Node<'a, T>>>,
    pub split_direction: Dimensions3,
    pub split_value: f64,
    pub values: Option<&'a [T]>, // has the number of values we actually have
    pub value_count: i32,
    pub max_radius: f64,
    pub com: Vect3,
    pub mass: f64,
    pub width: f64
    // store width, max spread
    // opening angle size/distance

}

pub struct Tree<'a, T: KDPoint> {
    pub root: Option<Box<Node<'a, T>>>,
    pub dims: Vec<Dimensions3>
}

// struct TreeIterLocation<'a, T: KDPoint> {
//     tree: &'a Tree<T>,
//     // current: Option<&'a Node<T>>,
//     index: usize,
//     stack: Vec<&'a Node<T>>
// }

impl<'a, T: KDPoint> Tree<'a, T> {
    pub fn new(points: &mut [T]) -> Tree<T> {
        let dims = T::all_axis();
        let root = Tree::construct_level(&dims, points);
        Tree { root: root, dims: dims }
    }

    fn get_max_dim_spread(dims: &Vec<Dimensions3>, points: &[T]) -> (Dimensions3, f64) {
        let mut best_split_direction = &dims[0];
                let mut best_value = 0.;
    
                for d in dims {
                    let x = T::spread_in_dim(points, d);
                    if x > best_value {
                        best_value = x;
                        best_split_direction = d;
                    }
                }
    
                (*best_split_direction, best_value)
    }

    fn construct_level(dims: &Vec<Dimensions3>, points: &'a mut[T]) -> Option<Box<Node<'a, T>>> {
        let len = points.len();
        if len == 0 {
            return None;
        }

        if len <= NODE_VALUE_COUNT {
            let points_fixed: &[T] = points;

            let max_radius = points_fixed.iter().map(|x| x.get_radius()).fold(0., |m, a| if a > m { a } else { m });
            let mass: f64 = points_fixed.iter().map(|x| x.get_mass()).sum();

            let (_, width) = Tree::get_max_dim_spread(dims, points_fixed);
            Some(Box::new(Node { 
                left: None, 
                right: None, 
                split_direction: Dimensions3::X, 
                split_value: 0.,
                values: Some(points_fixed),
                value_count: len as i32,
                max_radius: max_radius,
                com: KDPoint::compute_com(points_fixed),
                mass: mass,
                width: width
            }))
        }
        else {
            let (split_direction, split_spread) = Tree::get_max_dim_spread(dims, points);

            points.sort_unstable_by(|a, b| a.cmp_on_dim(b, &split_direction));

            let max_radius = points.iter().map(|x| x.get_radius()).fold(0., |m, a| if a > m { a } else { m });
            let mass: f64 = points.iter().map(|x| x.get_mass()).sum();
            
            let median = T::get_value_in_dim(points, len/2, &split_direction);  //points[(points.len()/2)];
            
            let com = KDPoint::compute_com(points);
            let (first, second) = points.split_at_mut(len/2);
            let left = Tree::construct_level(dims, first);
            let right = Tree::construct_level(dims, second);

            Some(Box::new(Node { 
                left: left, 
                right: right, 
                split_direction: split_direction, 
                split_value: median,
                values: None,
                value_count: 0,
                max_radius: max_radius,
                com: com,
                mass: mass,
                width: split_spread
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
            // if this is called, n.value_count != 0
            // if n.values is None, then something got messed up
            for v in n.values.unwrap() {
                v.print();
                print!(", ");
            }
            println!("");
        }

    }

    fn recursive_helper(n: &Option<Box<Node<T>>>, point: &T) -> Vect3 {
        // println!("Recursion!");
        match n {
            None => Vect3::ZERO,
            Some(r) => {
                let node = r.as_ref();
                let dis = (point.get_point() - node.com).mag();
                let open_angle = node.width / dis;
                if open_angle < OPENING_ANGLE_RATIO {
                    point.compute_acceleration_from_node(node)
                }
                else {
                    let left = Tree::recursive_helper(&node.left, point);
                    let right = Tree::recursive_helper(&node.right, point);
                    let points_acc = if r.as_ref().value_count > 0 {
                        let mut curr = Vect3::ZERO;
    
                        for pt in node.values.unwrap_or(&[]) {
                            curr = curr + point.compute_acceleration_from(pt);
                        }
    
                        curr
                    } else { Vect3::ZERO };
    
                    left + right + points_acc
                }

            }
        }
    }

    pub fn compute_acceleration(&self, point: &T) -> Vect3 {
        Tree::recursive_helper(&self.root, point)
    }

    // pub fn iter(&self) -> TreeIterLocation<T> {
    //     // let t = TreeIterLocation{ tree: self, stack: Vec::new(), index: 0 }
    //     // t.go_left();
    //     // return 
    //     TreeIterLocation::new(self)
    // }
}

// impl<T> Node<T> {

//     pub fn go_left(&self, stack: &Vec<&Node<T>>) -> &Self {
//         match self.left {
//             Some(e) => {
//                 let mut node = e;
//                 while false {
//                     match node.left {
//                         Some(e) => { node = e; }
//                         None => { break; }
//                     }
//                 }
//                 node.as_ref()
//             },
//             None => self
//         }
//     }
// }

// impl<'a, T: KDPoint> TreeIterLocation<'a, T> {

//     pub fn new(tree: &Tree<T>) -> TreeIterLocation<T> {
//         let mut stack = Vec::new();
//         // self.root.map(|e| e.as_ref().go_left())
//         let mut node = tree.root.map(|e| e.as_ref());
//         while match node {
//                 Some(e) => {
//                     stack.push(e);
//                     node = e.left.map(|e| e.as_ref());
//                     true 
//                 }
//                 None => false
//         } {}
        
//         TreeIterLocation{ tree: tree, stack: stack, index: 0 }
//     }
// }

// impl<'a, T: KDPoint> Iterator for TreeIterLocation<'a, T> {
//     // We can refer to this type using Self::Item
//     type Item = &'a T;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.stack.pop() {
//             Some(e) => {
//                 match e.get_values().get(self.index) {
//                     Some(v) => {
//                         self.index += 1;
//                         Some(v)
//                     },
//                     None => {
//                         // nothing left here, go up
//                         self.index = 0;

//                     }
//                 }
//             },
//             None => None
//         }
//     }
// }

