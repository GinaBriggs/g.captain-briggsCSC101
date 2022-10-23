 fn main()  {
    // Assigning Variables for Loan(l), Rate(r) and Number of years(n)
 	let l:f64 = 520_000_000.0;
    let r:f64 = 10.0;
    let n:f64 = 5.0;

    // Using the Compound Interest(CI) formula
    let a = l * ( 1.0 + (r / 100.0)) * n;
    println!("The amount is {}", a);
    let ci = a - l;
    println!("Compound Interest is {}", ci);
 }