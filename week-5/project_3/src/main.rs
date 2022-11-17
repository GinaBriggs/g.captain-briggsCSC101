
use std::io;

fn main() {

    println!("
        Menu                         Price
        Poundo Yam/Edinkaiko Soup   -N3,200
        Fried Rice & Chicken        -N3,000
        Amala & Ewedu Soup          -N2,500
        Eba & Egusi Soup            -N2,000
        White Rice & Stew           -N2,500");

     let mut input1 = String::new();
     let mut input2 = String::new();
     let mut input3 = String::new();
     let mut input4 = String::new();
     let mut input5 = String::new();

    println!("How much Poundo Yam/Edinkaiko soup would you like?:");
        io::stdin()
        .read_line(&mut input1)
        .expect("Not a valid string");
    let input1:f32 = input1.trim().parse().expect("Not a valid response");
        
    println!("How much Fried Rice & Chicken would you like?:");
        io::stdin()
        .read_line(&mut input2)
        .expect("Not a valid string");
    let input2:f32 = input2.trim().parse().expect("Not a valid response");
        
    println!("How much Amala & Ewedu Soup would you like?:");
        io::stdin()
        .read_line(&mut input3)
        .expect("Not a valid string");
    let input3:f32 = input3.trim().parse().expect("Not a valid response");
        
    println!("How much Eba & Egusi Soup soup would you like?:");
        io::stdin()
        .read_line(&mut input4)
        .expect("Not a valid string");
    let input4:f32 = input4.trim().parse().expect("Not a valid response");
        
    println!("How much White Rice & Stew would you like?:");
        io::stdin()
        .read_line(&mut input5)
        .expect("Not a valid string");
    let input5:f32 = input5.trim().parse().expect("Not a valid response");

    let total:f32 = (input1 * 3200.0) + (input2 * 3000.0) + (input3 * 2500.0) + (input4 * 2000.0) + (input5 * 2500.0);
    println!("Your total price is N{}", total);

    
        let discount = total * (0.05);
        let price = total - discount;

        if total > 10000.0{
        println!("Congratulations! You have a discount of 5%
            Your new total price is N{}", price);
         }
}
        
