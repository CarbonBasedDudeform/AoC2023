#[cfg(test)]
mod tests {
    use crate::day1;

    #[test]
    fn string_with_values_at_start_and_end() {
        let val = "1abc2";
        
        match day1::get_calibration_value(val) {
            Ok(result) => {
                assert_eq!(result, 12);
            }
            Err(result) => {
                eprintln!("Error: {}", result);
            }
        }
    }

    #[test]
    fn string_has_one_pair_internally() {
        let val = "pqr3stu8vwx";
        
        match day1::get_calibration_value(val) {
            Ok(result) => {
                assert_eq!(result, 38);
            }
            Err(result) => {
                eprintln!("Error: {}", result);
            }
        }
    }

    #[test]
    fn string_has_multiple_internal_pairs_selects_outer() {
        let val = "a1b2c3d4e5f";
        
        match day1::get_calibration_value(val) {
            Ok(result) => {
                assert_eq!(result, 15);
            }
            Err(result) => {
                eprintln!("Error: {}", result);
            }
        }
    }

    #[test]
    fn string_has_only_one_number_returns_as_both_start_and_end() {
        let val = "treb7uchet";
        match day1::get_calibration_value(val) {
            Ok(result) => {
                assert_eq!(result, 77);
            }
            Err(result) => {
                eprintln!("Error: {}", result);
            }
        }
    }
}
