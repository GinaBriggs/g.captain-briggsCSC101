 fn main()  {
    /* Assigning Variables for Toshiba(t), Mac(m) , HP(h), Dell(d) 
       and Acer(a) 

    */
 	let t:f64 = 450_000.0;
    let m:f64 = 1_500_000.0;
    let h:f64 = 750_000.0;
    let d:f64 = 2_850_000.0;
    let a:f64 = 250_000.0;
    // Assigning the quantity(q)
    let q1:f64 = 1.0;
    let q2:f64 = 2.0;
    let q3:f64 = 3.0;
    // Assigning the number of items(n)
    let n:f64 = 10.0;
    let nt:f64 = t * q2;
    let nm:f64 = m * q1;
    let nh:f64 = h * q3;
    let nd:f64 = d * q3;
    let na:f64 = a * q1;
    // Summing the values(s)
    let s = nt + nm + nh + nd + na;
    println!("The sum of the product sales is {}", s);
    // Getting the average(a)
    let a = s/n;
    println!("The average of the product sales is {}", a);
 }