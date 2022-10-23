 fn main()  {
 	let p:f64 = 3000.0;
    let r:f64 = 2.0;
    let t:f64 = 1.0;

    // Simple Interest Calculation
    let a = p * ( 1.0 + (r / 100.0)) * t;
    println!("The amount is {}", a);
    let si = a - p;
    println!("Simple Interest is {}", si);
 }