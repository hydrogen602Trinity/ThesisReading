
pub fn testf() {
    println!("yeet");
}

pub enum Dimensions {
    X,
    Y,
    Z,
    VX,
    VY,
    VZ
}

// const DIMS: [Dimensions; 6] = [Dimensions::X, Dimensions::Y, Dimensions::Z, Dimensions::VX, Dimensions::VY, Dimensions::VZ];

pub struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    split_direction: Dimensions,
    split_value: f64,
    values: [f64; 8],
    value_count: i32
}

pub struct Tree {
    pub root: Option<Box<Node>>,
    pub dims: Vec<Dimensions> //&[Dimensions]
}

impl Tree {
    pub fn new(dims: Vec<Dimensions>, points: &mut[f64]) -> Tree {
        let root = Tree::construct_level(&dims, points);
        Tree { root: root, dims: dims }
    }

    fn construct_level(dims: &Vec<Dimensions>, points: &mut[f64]) -> Option<Box<Node>> {
        let len = points.len();
        if len == 0 {
            return None;
        }
        
        let split_direction = Dimensions::X;
        
        if len <= 8 {
            let mut values: [f64; 8] = [0.; 8];
            let slice = &mut values[..len];
            slice.copy_from_slice(points);
            Some(Box::new(Node { 
                left: None, 
                right: None, 
                split_direction: split_direction, 
                split_value: 0.,
                values: values,
                value_count: len as i32
            }))
        }
        else {
            let values: [f64; 8] = [0.; 8];
            points.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        
            let left = Tree::construct_level(dims, &mut points[..(len/2)]);

            let right = Tree::construct_level(dims, &mut points[(len/2)..]);

            let median = points[(points.len()/2)];

            Some(Box::new(Node { 
                left: left, 
                right: right, 
                split_direction: split_direction, 
                split_value: median,
                values: values,
                value_count: 0
            }))
        }
    }

    pub fn printer(n: &Node, depth: Option<i32>) -> () {
        let tabs = match depth {
            None => 0,
            Some(e) => e
        };

        for _ in 0..tabs {
            print!("  ");
        }
        if n.value_count == 0 {
            println!("{}", n.split_value);
            match &n.left {
                None => (),
                Some(x) => Tree::printer(x, Some(tabs+1))
            };
            match &n.right {
                None => (),
                Some(x) => Tree::printer(x, Some(tabs+1))
            };
        }
        else {
            print!("points = ");
            let mut i = 0;
            for v in &n.values {
                if i >= n.value_count {
                    break;
                }
                if v < &10. {
                    print!("0");
                }
                print!("{:0.2}, ", v);
                i += 1;
            }
            println!("");
        }

    }
}
