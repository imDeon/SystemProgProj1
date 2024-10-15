use std::fs;
use std::path::Path;
use std::time::Instant;
use version1::process_input_file;

fn main() {
    // Paths
    let data_dir = "../data";
    let output_dir = format!("{}/weekly_summary", data_dir);
    let output_file = format!("{}/weekly_sales_summary.txt", output_dir);

    // Output
    if !Path::new(&output_dir).exists() {
        fs::create_dir(&output_dir).expect("Failed to create output directory");
    }

    // Timer
    let start_time = Instant::now();

    // Inputs
    let branch_folders = fs::read_dir(data_dir)
        .expect("Failed to read data directory")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_dir() && path.file_name().unwrap() != "weekly_summary" {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // Processing
    let result = process_input_file(branch_folders, &output_file);
    println!("{}", result);

    // Timer Results
    let elapsed_time = start_time.elapsed();
    println!("Time taken: {:?}", elapsed_time);

    // End
    println!("Phew! I am done.");
}
