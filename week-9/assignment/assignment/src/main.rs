use std::io;
use std::io::Write;

fn aigbona_juliet(){
    let mut file = std::fs::File::create("Aigbona_juliet.txt").expect("create failed");
    file.write_all("Name: Aigbona Juliet".as_bytes()).expect("write failed");
    file.write_all("\n-----".as_bytes()).expect("write failed");        
    file.write_all("\n\nQualification: B.Sc.".as_bytes()).expect("write failed");
    file.write_all("\n--------------".as_bytes()).expect("write failed"); 
    file.write_all("\n\nCode: 7".as_bytes()).expect("write failed");
    file.write_all("\n-----".as_bytes()).expect("write failed"); 
    file.write_all("\n\nDepartment: Consulting".as_bytes()).expect("write failed");
    file.write_all("\n-----------".as_bytes()).expect("write failed");        
    file.write_all("\n\nConsulting services:".as_bytes()).expect("write failed");
    file.write_all("\n--------------------".as_bytes()).expect("write failed");     
    let consulting = vec!["\nAnalytics consulting services", "\nCustomer experience",
    "\nCybersecurity, strategy, risk, compliance and resilience", "\nDigital transformation", "\nRisk consulting services", 
    "\nSupply chain and operations", "\nTechnology transformation"];
    for i in 0..consulting.len() {
    file.write_all(consulting[i].as_bytes()).expect("write failed"); }
    }

fn ehis_ero(){
    let mut file = std::fs::File::create("Ehis_ero.txt").expect("create failed");
    file.write_all("\nName: Ehis Ero".as_bytes()).expect("write failed");
    file.write_all("\n-----".as_bytes()).expect("write failed");  
    file.write_all("\nQualification: M.Sc.".as_bytes()).expect("write failed");
    file.write_all("\n--------------".as_bytes()).expect("write failed"); 
    file.write_all("\nCode: 9".as_bytes()).expect("write failed");
    file.write_all("\n-----".as_bytes()).expect("write failed"); 
    file.write_all("\nDepartment: Strategy".as_bytes()).expect("write failed");
    file.write_all("\n-----------".as_bytes()).expect("write failed"); 
    file.write_all("\n\nStrategy by EY-Parthenon services:".as_bytes()).expect("write failed");
    file.write_all("\n----------------------------------".as_bytes()).expect("write failed"); 
    let strategy = vec!["\nStrategy consulting", "\nCorporate and growth strategy", "\nTransaction strategy and execution", 
    "\nRestructing and turnaround strategy", "\nIndustry strategy", "\nDigital business building", "\nCommercial Strategy"];
    for i in 0..strategy.len() {
    file.write_all(strategy[i].as_bytes()).expect("write failed");  }
    }

fn adamu_sagamu(){
    let mut file = std::fs::File::create("Adamu_sagamu.txt").expect("create failed");
    file.write_all("\nName: Adamu Sagamu".as_bytes()).expect("write failed");
    file.write_all("\n-----".as_bytes()).expect("write failed");  
    file.write_all("\nQualification: B.Sc.".as_bytes()).expect("write failed");
    file.write_all("\n--------------".as_bytes()).expect("write failed"); 
    file.write_all("\nCode: 8".as_bytes()).expect("write failed");
    file.write_all("\n-----".as_bytes()).expect("write failed"); 
    file.write_all("\nDepartment: Tax".as_bytes()).expect("write failed");
    file.write_all("\n-----------".as_bytes()).expect("write failed"); 
    file.write_all("\n\nTax services:".as_bytes()).expect("write failed");
    file.write_all("\n-------------".as_bytes()).expect("write failed"); 
    let tax = vec!["\nTax planning", "\nTax function operations", "\nTax policy and controversy", "\nGlobal trade", "\nTax accounting", 
    "\nTax compliance", "\nTransaction tax"];
    for i in 0..tax.len() {
    file.write_all(tax[i].as_bytes()).expect("write failed");    }
    }

fn akpevwe_iloka(){
    let mut file = std::fs::File::create("Akpevwe_iloka.txt").expect("create failed");
    file.write_all("\nName: Akpevwe Iloka".as_bytes()).expect("write failed");
    file.write_all("\n-----".as_bytes()).expect("write failed");  
    file.write_all("\nQualification: HND".as_bytes()).expect("write failed");
    file.write_all("\n--------------".as_bytes()).expect("write failed"); 
    file.write_all("\nCode: 7".as_bytes()).expect("write failed");
    file.write_all("\n-----".as_bytes()).expect("write failed"); 
    file.write_all("\nDepartment: Assurance".as_bytes()).expect("write failed");
    file.write_all("\n-----------".as_bytes()).expect("write failed"); 
    file.write_all("\n\nAssurance services:".as_bytes()).expect("write failed");
    file.write_all("\n-------------------".as_bytes()).expect("write failed"); 
    let assurance = vec!["\nAudit services", "\nClimate change and sustainability services", "\nFinancial accounting advisory services", 
    "\nForensic and integrity services", "\nPrivate client audit experience", "\nAccounting Link", "\nAssurance"];
    for i in 0..assurance.len() {
    file.write_all(assurance[i].as_bytes()).expect("write failed");  }
    }

fn maria_akinsola(){
    let mut file = std::fs::File::create("Maria_akinsola.txt").expect("create failed");
        file.write_all("\nName: Maria Akinsola".as_bytes()).expect("write failed");
        file.write_all("\n-----".as_bytes()).expect("write failed");  
        file.write_all("\nQualification: M.Sc.".as_bytes()).expect("write failed");
        file.write_all("\n--------------".as_bytes()).expect("write failed"); 
        file.write_all("\nCode: 9".as_bytes()).expect("write failed");
        file.write_all("\n-----".as_bytes()).expect("write failed"); 
        file.write_all("\nDepartment: Transactions and corporate finance".as_bytes()).expect("write failed");
        file.write_all("\n-----------".as_bytes()).expect("write failed"); 
        file.write_all("\n\nTransactions and corporate finance services:".as_bytes()).expect("write failed");
        file.write_all("\n--------------------------------------------".as_bytes()).expect("write failed");        
        let transactions = vec!["\nCorporate finance", "\nDivestments and carve-outs", "\nSustainability and ESG Services", "\nM&A advisory"
        ,"\nM&A integration", "\nM&A technology and tools", "\nM&A advanced analytics"];
        for i in 0..transactions.len() {
        file.write_all(transactions[i].as_bytes()).expect("write failed");  }
        }

fn gbenga_daniels(){
    let mut file = std::fs::File::create("Gbenga_daniels.txt").expect("create failed");
    file.write_all("\nName: Gbenga Daniels".as_bytes()).expect("write failed");
    file.write_all("\n-----".as_bytes()).expect("write failed");  
    file.write_all("\nQualification: HND".as_bytes()).expect("write failed");
    file.write_all("\n--------------".as_bytes()).expect("write failed"); 
    file.write_all("\nCode: 8".as_bytes()).expect("write failed");
    file.write_all("\n-----".as_bytes()).expect("write failed");
    file.write_all("\nDepartment: People and workforce".as_bytes()).expect("write failed");
    file.write_all("\n-----------".as_bytes()).expect("write failed"); 
    file.write_all("\n\nPeople and workforce services:".as_bytes()).expect("write failed");
    file.write_all("\n------------------------------".as_bytes()).expect("write failed");
    let people = vec!["\nChange management and experience", "\nHR transformation", "\nIntegrated workforce mobility", 
    "\nLearning and development consulting", "\nRecognition and reward advisory", "\nWorkforce analytics", "\nPeople and workforce"];
    for i in 0..people.len() {
    file.write_all(people[i].as_bytes()).expect("write failed");  }
    }

fn main() {
    let mut input0 = String::new();
    println!("Welcome to the HR service of Ernst & Young Global Limited, trade name EY, is a multinational professional services
              partnership headquartered in London, England. EY is one of the largest professional
              services networks in the world. In a world that’s changing faster than ever, our purpose
              acts as the ‘North Star’ guiding more than 300,000 people — providing the context and
              meaning for the work they do every day. EY HR is restructuring their services for the new
              strategic plan of the fiscal year.");
    println!("The following staff data files are available:
              1. Aigbona Juliet
              2. Ehis Ero
              3. Adamu Sagamu
              4. Akpevwe Iloka
              5. Maria Akinsola
              6. Gbenga Daniels");
    println!("Who's staff data file would you like to view?: ");
    io::stdin().read_line(&mut input0).expect("Pick a number from the options above");
    let a:i32 = input0.trim().parse().expect("Not a valid number");

    if a == 1{
        aigbona_juliet();
        println!("\nAigbona Juliet's data has been written to file.");}
    if a == 2{
        ehis_ero();
        println!("\nEhis Ero's data has been written to file.");}
    if a == 3{
        adamu_sagamu();
        println!("\nAdamu Sagamu's data has been written to file.");}
    if a == 4{
        akpevwe_iloka();
        println!("\nAkpevwe Iloka's data has been written to file.");}
    if a == 5{
        maria_akinsola();
        println!("\nMaria Akinsola's data has been written to file.");}
    if a == 6{
        gbenga_daniels();
        println!("\nGbenga Daniels's data has been written to file.");}
    else {
        println!("
            _______________________________________
            That number is not part of the options.
            ---------------------------------------");
    }
}