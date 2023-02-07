pub mod eigen;

use eigen::Vector;

fn main() {
    let mut v1: Vector = Vector::from(vec![1.0, 2.0, 3.0]);
    println!("v1: {}", v1);
    println!("v1[0]: {}", v1[0]);

    v1[0] = 10.0;
    println!("v1: {}", v1);
}
