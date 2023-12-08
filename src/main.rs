mod day3;

//use std::fs::File;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn day1_main() {
    //match day1::get_calibration_value(d.as_str()) {
    //     Ok(result) => {
    //         println!("{}", result);
    //         ans += result;
    //     }
    //     Err(result) => {
    //         eprintln!("Error: {}", result);
    //     }
    // }
}

fn day2_main() {
    //use crate::day2::Limits;
    //let limits = Limits {red: 12, green: 13, blue: 14};

    // let game = day2::parse_game(d.as_str());
    // if day2::is_possible(&game, &limits) {
    //     println!("{}", game);
    //     ans += game.id;
    // }
}

fn main() {
    //let data = read_lines("./data.txt");
    let input = read_to_string("./data.txt");
    //"123+....\n........"
    match input {
        Ok(result) => {
            let nums = day3::parse_part_numbers(result.as_str());
            let mut ans = 0;
            for n in nums {
                ans += n;
            }
            println!("{}", ans);
        }
        Err(result) => {
            eprintln!("Error: {}", result);
        }
    } 
    
    //let mut ans = 0;
    //for d in data {
    //}

    //println!("{}", ans);
    
}
