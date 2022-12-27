use office::{Excel, Range, DataType};

// opens a new workbook
let mut workbook = Excel::open(path).unwrap();

// Read whole worksheet data and provide some statistics
if let Ok(range) = workbook.worksheet_range("Sheet1") {
    let total_cells = range.get_size().0 * range.get_size().1;
    let non_empty_cells: usize = range.rows().map(|r| {
        r.iter().filter(|cell| cell != &&DataType::Empty).count()
    }).sum();
    println!("Found {} cells in 'Sheet1', including {} non empty cells",
             total_cells, non_empty_cells);
             
}
