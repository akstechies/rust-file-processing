// Import the modules
use std::fs;
use std::io::Write;

fn main() {
    //Read data from input file
    let input_file = "input.txt";
    println!("{}", input_file);

    let contents = match fs::read_to_string(input_file) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            return;
        }
    };
    println!("{}", contents);

    // Process data, convert it to upper case
    let processed_data = contents.to_uppercase();
    println!("{}", processed_data);

    //Write processed data to output file
    let output_filename = "output.txt";
    //mutable
    let mut output_file = match fs::File::create(output_filename) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating output file: {}", e);
            return;
        }
    };

    if let Ok(_) = write!(output_file, "{}", processed_data) {
        println!("Data processed and written to {}", output_filename);
    } else {
        eprintln!("Error writing to output file");
    }
}
