pub mod eigen;

use eigen::Vector;

fn main() {
    let mut v1: Vector = Vector::from(vec![1.0, 2.0, 3.0]);
    let v2: Vector = Vector::from(vec![4.0, 5.0, 6.0]);
    println!("v1: {}", v1);
    println!("v1[0]: {}", v1[0]);

    v1[0] = 10.0;
    println!("v1: {}", v1);
    v1 = v1 * 2.0;
    println!("v1 * 2.0: {}", v1);
    println!("v1 * v2: {}", v1 * v2);
}
