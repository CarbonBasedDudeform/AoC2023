use std::collections::HashSet;

struct Dims {
    x: usize,
    y: usize,
}

struct Map {
    dims: Dims,
    data: Vec<Vec<char>>,
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Point {
    x: usize,
    y: usize,
}

fn get_dims(input: &str) -> Dims {
    let mut dims = Dims{x: 0, y: 0};
    let length: Vec<&str> = input.lines().collect();

    dims.x = length.get(0).expect("no lines").len();
    dims.y = input.lines().count();

    return dims;
}

fn get_map(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.lines().collect();

    // Create a 2D array of characters
    let mut char_2d_array: Vec<Vec<char>> = Vec::new();

    // Populate the 2D array
    for line in lines {
        let char_vec: Vec<char> = line.chars().collect();
        char_2d_array.push(char_vec);
    }

    return char_2d_array;
}

fn get_horizontal_adjacent_numbers(map: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<Point> {
    let mut adjacent_numbers = Vec::new();
    
    if y >= map.len() {
        return adjacent_numbers; // Invalid y-coordinate
    }
    
    let row = &map[y];
    let width = row.len();

    if x >= width {
        return adjacent_numbers; // Invalid x-coordinate
    }

    // Skip the first character at the current position
    let mut i = x + 1;

    // Check cells to the right until the end of the row or a non-number character is encountered
    while i < width {
        let current_char = row[i];
        if current_char.is_digit(10) {
            adjacent_numbers.push(Point{x: i,  y: y});
            i += 1;
        } else {
            break; // Stop if a non-number character is encountered
        }
    }

    adjacent_numbers
}

fn is_valid_neighbor(map: &Vec<Vec<char>>, p: &Point) -> bool {
    let x = p.x;
    let y = p.y;

    let max_y = map.len() as isize;
    if y >= map.len() {
    
        return false; // Invalid coordinates
    }   

    
    let max_x = map[y].len() as isize;
    
    if x >= map[y].len() {
        
        return false;
    }

    
    let neighbors = [
        (-1, 0), // Left
        (1, 0),  // Right
        (0, -1), // Up
        (0, 1),  // Down
        (-1, -1), //Left-Up
        (1, 1), //Right-Down
        (-1, 1), // Left-Down
        (1, -1), //Right-Up
    ];

    let mut any_valid = false;
    let mut current_x = x;

    if map.len() < 1 {
        return false;
    }

    while current_x < map.get(0).expect("somehow wasn't a row?!?").len() && map[y][current_x].is_digit(10) {
    for (dx, dy) in &neighbors {
        let nx = current_x as isize + dx;
        let ny = y as isize + dy;

        // Check if the neighbor is out of bounds
        if nx >= 0 && ny >= 0 && nx < max_x && ny < max_y {
            let nx = nx as usize;
            let ny = ny as usize;

            let neighbor_char = map[ny][nx];
            if neighbor_char.is_digit(10) {
                continue;
            }

            // Check if the neighbor is not a number or "."
            if neighbor_char != '.' {
                any_valid = true;
                //println!("not valid");
            }
        }
    }

    current_x += 1;
    }

    any_valid
}

fn extract_number_from_map(map: &Vec<Vec<char>>, p: &Point) -> Option<i32> {
    let x = p.x;
    let y = p.y;

    if y >= map.len() || x >= map[y].len() {
        return None; // Invalid coordinates
    }

    let mut result = String::new();
    let mut current_x = x;
    let mut current_y = y;

    while current_y < map.len() {
        let row = &map[current_y];
        
        if current_x >= row.len() {
            break; // Reached the end of the row
        }

        let current_char = row[current_x];
        if current_char.is_digit(10) {
            result.push(current_char);
        } else {
            break; // Stop if any non-number character is encountered
        }

        current_x += 1;
    }

    if !result.is_empty() {
        match result.parse::<i32>() {
            Ok(parsed_number) => Some(parsed_number),
            Err(_) => None, // Invalid number format
        }
    } else {
        None // No valid number found
    }
}


pub fn parse_part_numbers(input: &str) -> Vec<i32> {
    let mut part_numbers = Vec::<i32>::new();
    let dims = get_dims(input);
    let map = get_map(input);

    let roots = Vec::<Point>::new();
    let mut excluded = HashSet::<Point>::new();

    for (y, row) in map.iter().enumerate() {
        for (x, character) in row.iter().enumerate() {
            if character.is_digit(10) {
                let p = Point{x: x, y: y};
                if excluded.contains(&p) == false {
                    let neighbours = get_horizontal_adjacent_numbers(&map, x, y);
                    for n in &neighbours {
                        excluded.insert(n.clone());
                    }

                    let valid = is_valid_neighbor(&map, &p);
                    if valid {
                        match extract_number_from_map(&map, &p) {
                            Some(result) => {
                                println!("{}", result);
                                part_numbers.push(result);
                            }
                            None => {
                                eprintln!("error");
                            }  
                        }
                    }
                }
            }
        }
    }


    return part_numbers;
}

#[cfg(test)]
mod mod_test;