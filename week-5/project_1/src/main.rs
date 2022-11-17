// Rust program to output roots of a quadratic

use std::io;

fn main()  {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Using ax^2 + bx + c, enter your value of a:");
        io::stdin()
        .read_line(&mut input1)
        .expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter your value of b:");
        io::stdin()
        .read_line(&mut input2)
        .expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter your value of c:");
        io::stdin()
        .read_line(&mut input3)
        .expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

 
    let _d:f32 = (b * b) - (4.0 * a * c);

        if _d == 0.0
     { println!("There is exactly one real root");}
    if _d > 0.0
     { println!("There are two distinct roots");}
    else {
        unimplemented!("There are no real roots");
    }
    
       let f = _d.sqrt();
       let first:f32 = ((-1.0 * b) - f ) / (2.0 * a);
       let second:f32 = ((-1.0 * b) + f ) / (2.0 * a);

    println!("The roots of the equation are: x = {} and x = {}", first, second);

}
