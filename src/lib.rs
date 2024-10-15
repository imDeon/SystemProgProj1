use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub fn process_input_file(folders: Vec<std::path::PathBuf>, output_file: &str) -> String {
    // Open Ouput
    let mut output = File::create(output_file).expect("Couldn't create output file");

    // Processing
    for folder in folders {
        // Path Def.
        let input_path = folder.join("branch_weekly_sales.txt");

        // Open Inputs
        let file = File::open(&input_path).expect("Couldn't open input file");
        let reader = BufReader::new(file);

        let mut total_quantity = 0;
        let mut branch_code = String::new();
        let mut product_code = String::new();

        // Processing Sums
        for line in reader.lines() {
            let line = line.expect("Couldn't read line");
            let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
            if parts.len() == 4 {
                branch_code = parts[0].to_string();
                product_code = parts[1].to_string();
                if let Ok(quantity) = parts[2].parse::<i32>() {
                    total_quantity += quantity;
                }
            }
        }

        // Output Result
        if !branch_code.is_empty() && !product_code.is_empty() {
            let summary_line = format!("{}, {}, {}\n", branch_code, product_code, total_quantity);
            output.write_all(summary_line.as_bytes()).expect("Couldn't write to output file");
        }
    }

    "OK".to_string()
}
