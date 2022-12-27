use std::fs::OpenOptions;
use std::io::Write;

fn main() {

    let student = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
    let matric = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let department = vec!["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let level = vec!["300", "100", "200", "200", "100"];

    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("    Welcome to the “Student Management Information System” (PAU-SMIS)
    that is used to manage student-related data. This system provides facilities for recording and maintaining 
    personal details of students, maintaining marks scored in
    assessments and computing results of students, keeping track of 
    student attendance, managing many other student-related data\n"
        .as_bytes()).expect("write failed");

    for i in 0..student.len() {
        file.write_all("\n".as_bytes()).expect("write failed");
        file.write_all("\nName:".as_bytes()).expect("write failed");
        file.write_all(student[i].as_bytes()).expect("write failed");
        file.write_all("\nMatric Number:".as_bytes()).expect("write failed");
        file.write_all(matric[i].as_bytes()).expect("write failed");
        file.write_all("\nDepartment:".as_bytes()).expect("write failed");
        file.write_all(department[i].as_bytes()).expect("write failed");
        file.write_all("\nLevel:".as_bytes()).expect("write failed");
        file.write_all(level[i].as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
        
    }
    println!("\nData written to file.");

}

