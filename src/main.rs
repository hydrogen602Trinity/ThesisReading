mod kdtree;

use kdtree::Dimensions;
use kdtree::Tree;

fn main() {
    println!("Hello, world!");
    let mut x: [f64; 15] = [1.,2.,3.,4.,5.,6.,7.,8.,9.,10.,11.,12.,13.,14.,15.];
    let t: Tree = Tree::new(vec![Dimensions::X], &mut x[..]);

    Tree::printer(&t.root.unwrap(), None);
}
