mod day1;

//use std::fs::File;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn main() {
    let data = read_lines("./data.txt");
    let mut ans = 0;
    for d in data {
        match day1::get_calibration_value(d.as_str()) {
            Ok(result) => {
                println!("{}", result);
                ans += result;
            }
            Err(result) => {
                eprintln!("Error: {}", result);
            }
        }
    }

    println!("{}", ans);
    
}