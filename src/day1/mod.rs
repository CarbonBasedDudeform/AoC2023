
pub fn get_calibration_value(input: &str) -> Result<i32, std::num::ParseIntError> {
    let mut result = String::new();
    for c in input.chars() {
        if c.is_digit(10) {
            result.push(c);
            break;
        }
    }

    for d in input.chars().rev() {
        if d.is_digit(10) {
            result.push(d);
            break;
        }
    }

    match result.parse::<i32>() {
        Ok(parsed_value) => Ok(parsed_value),
        Err(parse_error) => Err(parse_error),
    }
}

#[cfg(test)]
mod mod_test;