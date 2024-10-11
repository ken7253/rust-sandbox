fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x); // xの値は{}です
    *&mut x = 6;
    println!("The value of x is: {}", x);
}
