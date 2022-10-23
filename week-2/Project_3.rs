 fn main()  {
    /* Assigning Variables for Principal(p), 
      Rate(r) and Number of years(n)
      */
 	 let p:f64 = 210_000.0;
    let r:f64 = 5.0;
    let n:f64 = 3.0;

    // Using the Compound Interest(CI) formula for depreciation
    let a = 1.0 - (r / 100.0);
    let b =f64::powf(a, n);
    let ci = p * b;
    println!("Compound Interest is {}", ci);
 }