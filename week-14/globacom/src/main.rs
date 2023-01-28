use std::io::Read;
use std::io;
use std::io::Write;

fn administrator() {
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn manager() {
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn employee() {
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn customer() {
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn vendor() {
    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn main() {
    let mut input0 = String::new();
    println!("Welcome! What type of user are you?
    1. Administrator
    2. Project Manager
    3. Employee
    4. Customer
    5. Vendor
    ");
    println!("Input a number from the options: ");
    io::stdin().read_line(&mut input0).expect("Pick a number from the options above");
    let a:i32 = input0.trim().parse().expect("Not a valid number");

    if a == 1{
        administrator();
        println!("\nDatabase structure has been returned.");}
    if a == 2{
        manager();
        println!("\nThe structure of the project table has been returned.");}
    if a == 3{
        employee();
        println!("\nThe structure of the staff table has been returned.");}
    if a == 4{
        customer();
        println!("\nThe structure of the customer table has been returned.");}
    if a == 5{
        vendor();
        println!("\nThe structure of the dataplan table has been returned.");}
    else {
        println!("
            _______________________________________
            That number is not part of the options.
            ---------------------------------------");}
    }
    
    
    
    
    



