use std::io;

fn trapezium(a: f32, b: f32, h:f32) {

    let area = h/2.0 * (a + b);

    println!("The area of the Trapezium is  = {}", area);
}

fn rhombus(a: f32, b: f32) {
    let area = 1.0/2.0 * (a * b);

    println!("The area of the rhombus is = {}", area);
}

fn parallelogram(a: f32, b:f32) {
    let area = a * b ;

    println!("The area of the Parallelogram is = {}", area);
}

fn cube(a: f32) {
    let area = 2.0 * (a * a);

    println!("The area of the Cube is  = {}", area);
}

fn cylinder(a: f32, h: f32) {
    let area = a * a * h * 22.0/7.0;

    println!("The Volume of the Cylinder is  = {}", area);
}
fn main() {

    println!("
        What would you like to calculate today?:
        1 - Area of Trapezium formula
        2 - Area of the rhombus formula
        3 - Area of Parallelogram formula
        4 - Area of Cube formula 
        5 - Volume of Cylinder formula");

    let mut input1 = String::new();
    println!("Pick a number from 1 to 5: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let d:i32 = input1.trim().parse().expect("Invalid input");

    let mut input4 = String::new();
    println!("Enter input for parameter A(base1/diagonal1/altitude/length of the side/radius):");
    io::stdin().read_line(&mut input4).expect("Failed to read input");
    let a:f32 = input4.trim().parse().expect("Invalid input");
    
    let mut input2 = String::new();
    println!("Enter input for parameter B(base2/diagonal2):");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f32 = input2.trim().parse().expect("Invalid input");

    let mut input3 = String::new();
    println!("Enter input for parameter C(height):");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let h:f32 = input3.trim().parse().expect("Invalid input");


    // call add function with arguments
    if d == 1 {trapezium(a, b, h);};
    if d == 2 {rhombus(a, b);};
    if d == 3 {parallelogram(a, b);};
    if d == 4 {cube(a);};
    if d == 5 {cylinder(a, h);};
}