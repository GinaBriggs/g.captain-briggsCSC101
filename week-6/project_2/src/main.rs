use std::io;

fn main() {
    let mut data: [_; 8] = ["","","","","","","",""];
        let mut  number = String::new();

println!("How many siblings do you have? :");
        io::stdin().read_line(&mut number).expect("Not a valid string");


    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input8 = String::new();
    let mut input9 = String::new();
    let mut input10 = String::new();
    let mut input = String::new();
       println!("Enter your sibling's first name :");
        io::stdin().read_line(&mut input).expect("Not a valid string");
       let mut name:String = input.trim().parse().expect("Not a valid number");
   
   data[0] = &input;
   //println!("Number of siblings is: {}", data[0]);

       println!("Enter your sibling's age:");
        io::stdin().read_line(&mut input2).expect("Not a valid string");
        let age:i32 = input2.trim().parse().expect("Not a valid number");
            data[1] = &input2;
    
            if age > 18{

                let mut input3 = String::new();
                println!("Are you (1)married or (2)single? 
                       Pick a number:");
              io::stdin().read_line(&mut input3).expect("Not a valid string");
                let z:i32 = input3.trim().parse().expect("Not a valid number");
                data[2] = &input3;
            
                    if z == 2{ let mut input1 = String::new();

        println!("Are you a (1)student or a (2)worker?
              Pick a number");
        io::stdin().read_line(&mut input8).expect("Not a valid string");
    let b:i32 = input8.trim().parse().expect("Not a valid number");
    data[7] = &input8;

    if b == 1{println!("What university do you attend?");
        io::stdin().read_line(&mut input9).expect("Not a valid string");
    let university:String = input9.trim().parse().expect("Not a valid number");
    data[8] = &input9;

    println!("What is your course of study?");
        io::stdin().read_line(&mut input10).expect("Not a valid string");
    let course:String = input10.trim().parse().expect("Not a valid number");
                            data[9] = &input10;}}
                    if z == 1{
                 let mut input4 = String::new();

                    println!("Do you have any offspring? Yes or no?");
                   io::stdin().read_line(&mut input4).expect("Not a valid string");
                   let offspring:String = input4.trim().parse().expect("Not a valid number");
                   data[3] = &offspring;

                 let mut input5 = String::new();
                 println!("What city does your family live in?");
                    io::stdin().read_line(&mut input5).expect("Not a valid string");
                    let city:String = input5.trim().parse().expect("Not a valid number");
                    data[4] = &city;

                        }
                }
            else {
                let mut input6 = String::new();
                 println!("What secondary school do you attend? :");
                    io::stdin().read_line(&mut input6).expect("Not a valid string");
                 let waec:String = input6.trim().parse().expect("Not a valid number");
                 data[5] = &waec;

                let mut input7 = String::new();
                println!("What's your current class level? :");
                io::stdin().read_line(&mut input7).expect("Not a valid string");
                let class:String = input7.trim().parse().expect("Not a valid number");
                data[6] = &class;

       } 
     println!("Number of siblings is: {}", data[0]);
     println!("Sibling's first name is: {}", data[1]);
     println!("Age: {}", data[2]);
     println!("Marital status: {}", data[3]);
     println!("Children: {}", data[4]);
     println!("City of residence: {}", data[5]);
     println!("Secondary school: {}", data[6]);
     println!("Current class level: {}", data[7]);
     println!("Occupation: {}", data[8]);
     println!("University: {}", data[9]);
     println!("Course: {}", data[10]);
    }