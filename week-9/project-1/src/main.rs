use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    println!("Welcome to Nigerian Breweries Plc, 
        the pioneer and largest brewing Company in Nigeria was incorporated in 1946 as Nigerian Brewery Limited.
        Our rich portfolio of high-quality Lager, Stout, Non-alcoholics and Spirit are uniquely outstanding which is why 
        we are Nigeria’s number one choice");

    let mut file = OpenOptions::new().append(true).open("data.txt").expect(
        "cannot open file");
    file.write_all("\nLarger     | Stout      |Non-Alcoholic".as_bytes()).expect("write failed");
    file.write_all("\n______________________________________".as_bytes()).expect("write failed");
    file.write_all("\n33 Export  |Legend      |Maltina".as_bytes()).expect("write failed");
    file.write_all("\nDesperados |Turbo King  |Amstel Malta".as_bytes()).expect("write failed");
    file.write_all("\nGoldberg   |Williams    |Malta Gold".as_bytes()).expect("write failed");
    file.write_all("\nGulder     |            |Favrouz".as_bytes()).expect("write failed");
    file.write_all("\nHeineken   |            |       ".as_bytes()).expect("write failed");
    file.write_all("\nStar       |            |        ".as_bytes()).expect("write failed");
    println!("file append success");

}
