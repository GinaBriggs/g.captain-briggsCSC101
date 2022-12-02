use std::io;

fn FacPub(){
    for x in 0..15{
    println!("Welcome to The Faculty Publication Incentive System");
    let mut input0 = String::new();
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut data: [_; 1] = [""];

    println!("What's your name?:");
        io::stdin().read_line(&mut input0).expect("Not a valid string");
    let c:String = input0.trim().parse().expect("Not a valid string");
        data[0] = &c;

    println!("What's the number of papers you have published?:");
        io::stdin().read_line(&mut input2).expect("Not a valid string");
    let n:i32 = input2.trim().parse().expect("Not a valid string");

    if n >= 3 && n <= 5{
        println!("Name: {}", data[0]);
        println!("Incentive is N500,000");}
    if n > 5 && n < 10{
        println!("Name: {}", data[0]);
        println!("Incentive is N800,000");}
    if n >= 10{
        println!("Name: {}", data[0]);
        println!("Incentive is N1,000,000");}

    if n < 3{
        println!("Name: {}", data[0]);
        println!("Incentive is N100,000");} 
}
}
fn StudentCouncil_VoteX(){
    for x in 0..50{
    let mut input0 = String::new();
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut data: [_; 4] = ["","","",""];

    println!("Welcome to The Student Council Voter System");

    println!("What's your name?:");
        io::stdin().read_line(&mut input0).expect("Not a valid string");
    let c:String = input0.trim().parse().expect("Not a valid string");
    data[0] = &c;

    println!("What's your email?:");
        io::stdin().read_line(&mut input2).expect("Not a valid string");
    let n:String = input2.trim().parse().expect("Not a valid string");
    data[1] = &n;

    println!("What's your department?:");
        io::stdin().read_line(&mut input2).expect("Not a valid string");
    let d:String = input2.trim().parse().expect("Not a valid string");
    data[2] = &d;

    println!("What's your state of Origin?:");
        io::stdin().read_line(&mut input2).expect("Not a valid string");
    let s:String = input2.trim().parse().expect("Not a valid string");
    data[3] = &s;

    println!("Are you a current Class Rep?");
        io::stdin().read_line(&mut input1).expect("Not a valid string");
    let c:String = input1.trim().parse().expect("Type yes or no");

     println!("Are you in 100level?");
        io::stdin().read_line(&mut input5).expect("Not a valid string");
        let xx:String = input5.trim().parse().expect("Type yes or no");
    println!("What is your CGPA?");
    io::stdin().read_line(&mut input6).expect("Not a valid string");
    let zzz:f32 = input6.trim().parse().expect("Not a valid number");

    if zzz > 4.0 && xx == "no"{
        println!("Name: {}", data[0]);
        println!("Email: {}", data[1]);
        println!("Department: {}", data[2]);
        println!("State of Origin: {}", data[3]);

        println!("You can vote");}

        else{
            println!("Name: {}", data[0]);
            println!("Email: {}", data[1]);
            println!("Department: {}", data[2]);
            println!("State of Origin: {}", data[3]);
            println!("Sorry, you are not eligible to vote.");}
        
 }
}

fn main() {

    FacPub();

    StudentCouncil_VoteX(); 


    }