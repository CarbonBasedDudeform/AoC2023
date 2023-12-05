use regex::Regex;
use std::fmt;

pub struct Match {
    red: i32,
    green: i32,
    blue: i32
}

pub struct Game {
    pub id: i32,
    matches: Vec<Match>
}

pub struct Limits {
    pub red: i32,
    pub green: i32,
    pub blue: i32
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "id: {}\n", self.id);
        for m in &self.matches {
            write!(f, "red: {}, green: {}, blue: {}\n", m.red, m.green, m.blue);
        }
        write!(f, "===end===\n")
    }
}

pub fn is_possible(game: &Game, limits: &Limits) -> bool {
    for m in &game.matches {
        if (m.red <= limits.red && m.blue <= limits.blue && m.green <= limits.green) == false {
            return false;
        }
    }
    
    true
}

fn extract(input: &str, pattern: &str) -> i32 {
    let re = Regex::new(pattern).unwrap();

    // Search for the pattern in the input string
    if let Some(captures) = re.captures(input) {
        // Extract the captured number
        if let Some(game_number) = captures.get(1) {
            let game_number_str = game_number.as_str();
            // Convert the extracted number to an integer
            if let Ok(game_number) = game_number_str.parse::<i32>() {
                return game_number;
            }
        }
    }

    0
}

fn get_matches(input: &str) -> Vec<&str> {
    let remaining_str = &input["Game ".len()..];

        // Split the remaining string by ';'
        let game_data: Vec<&str> = remaining_str.split(';').collect();

        // Trim leading/trailing whitespaces from each element
        let trimmed_game_data: Vec<&str> = game_data
            .iter()
            .map(|s| s.trim())
            .collect();
            
        return trimmed_game_data;
}

pub fn parse_game(input: &str) -> Game {
    //Game 1: 4 red, 5 blue, 4 green; 7 red, 8 blue, 2 green; 9 blue, 6 red; 1 green, 3 red, 7 blue; 3 green, 7 red
    let mut game = Game{id: -1, matches: Vec::<Match>::new()};
    game.id = extract(input, r"Game (\d+):");

    
    let extracted_matches = get_matches(input);
    //println!("{}", input);
    for g in extracted_matches {
        let mut m = Match{red: 0, green: 0, blue: 0};
        m.red = extract(g, r"(\d+) red");
        m.green = extract(g, r"(\d+) green");
        m.blue = extract(g, r"(\d+) blue");
        game.matches.push(m);
    }

    game
}

#[cfg(test)]
mod mod_test;