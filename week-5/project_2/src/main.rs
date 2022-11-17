// Rust program to output name and age

use std::io;

fn main()  {
    print!("Let's determine your annual incentive!");
    let mut experience = String::new();
    let mut age = String::new();

    println!("Are you experienced? Input 1 for yes and 0 for no:");
        io::stdin()
        .read_line(&mut experience)
        .expect("Not a valid string");
    let a:i32 = experience.trim().parse().expect("Not a valid response");

    println!("What's your age?");
        io::stdin()
        .read_line(&mut age)
        .expect("Not a valid string");
    let b:i32 = age.trim().parse().expect("Not a valid number");

    if a == 1 && b >= 40 { println!("Your incentive is N1,560,000.");}
    if a == 1 && b >= 30 && b < 40 { println!("Your incentive is N1,480,000.");}
    if a == 1 && b < 28 { println!("Your incentive is N1,300,000 per month.");}
    if a == 0 {
        println!("Your incentive is N100,000.");
    }

}
