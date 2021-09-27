mod kdtree;

use kdtree::Dimensions;
use kdtree::Tree;

fn main() {
    println!("Hello, world!");
    let mut x: [f64; 50] = [26.21, 9.78, 26.18, 27.38, 8.59, 16.97, 0.31, 18.57, 21.27, 28.89, 28.43, 25.99, 6.74, 18.69, 0.3, 2.72, 14.48, 12.52, 9.49, 7.1, 15.57, 22.23, 17.85, 23.63, 26.12, 10.17, 0.81, 2.42, 0.98, 24.66, 28.93, 11.32, 3.19, 25.29, 6.91, 28.45, 4.41, 12.77, 24.46, 3.34, 4.54, 11.03, 27.08, 2.67, 22.7, 11.12, 14.25, 27.21, 18.1, 17.26];
    let t: Tree = Tree::new(vec![Dimensions::X], &mut x[..]);

    Tree::printer(&t.root.unwrap(), None);
}
