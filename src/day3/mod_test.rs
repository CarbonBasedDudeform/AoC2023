#[cfg(test)]
mod tests {
    use crate::day3;

    #[test]
    fn example_passes() {
        let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        let nums = day3::parse_part_numbers(input);
        let mut ans = 0;
        for n in nums {
            ans += n;
        }
        assert_eq!(ans, 4361);
    }

    #[test]
    fn none() {
        let input = "..........\n";
        let nums = day3::parse_part_numbers(input);
        let mut ans = 0;
        for n in nums {
            ans += n;
        }
        assert_eq!(ans, 0);
    }

    #[test]
    fn left() {
        let input = "+467.....\n..........\n..........";
        let nums = day3::parse_part_numbers(input);
        let mut ans = 0;
        for n in nums {
            ans += n;
        }
        assert_eq!(ans, 467);
    }

    #[test]
    fn right() {
        let input = "467+.....\n..........\n..........";
        let nums = day3::parse_part_numbers(input);
        let mut ans = 0;
        for n in nums {
            ans += n;
        }
        assert_eq!(ans, 467);
    }

    #[test]
    fn above() {
        let input = "..+.......\n.467......\n..........";
        let nums = day3::parse_part_numbers(input);
        let mut ans = 0;
        for n in nums {
            ans += n;
        }
        assert_eq!(ans, 467);
    }

    #[test]
    fn below() {
        let input = ".467......\n...+......\n..........";
        let nums = day3::parse_part_numbers(input);
        let mut ans = 0;
        for n in nums {
            ans += n;
        }
        assert_eq!(ans, 467);
    }

    #[test]
    fn diag() {
        let input = ".467......\n....+.....\n..........";
        let nums = day3::parse_part_numbers(input);
        let mut ans = 0;
        for n in nums {
            ans += n;
        }
        assert_eq!(ans, 467);
    }

    #[test]
    fn back2back() {
        let input = "952*374";
        let nums = day3::parse_part_numbers(input);
        let mut ans = 0;
        for n in nums {
            ans += n;
        }
        assert_eq!(ans, 952+374);
    }

    //negative numbers turn out not to be counted and - is a symbol
    // #[test]
    // fn negative() {
    //     let input = "-952*374";
    //     let nums = day3::parse_part_numbers(input);
    //     let mut ans = 0;
    //     for n in nums {
    //         ans += n;
    //     }
    //     assert_eq!(ans, -952+374);
    // }

    #[test]
    fn reddit() {
        let input = "...\n..1\n..=";
        let nums = day3::parse_part_numbers(input);
        let mut ans = 0;
        for n in nums {
            ans += n;
        }
        assert_eq!(ans, 1);
    }

    #[test]
    fn same_row() {
        let input = "1.1\n...\n...";
        let nums = day3::parse_part_numbers(input);
        let mut ans = 0;
        for n in nums {
            ans += n;
        }
        assert_eq!(ans, 0);
    }

    #[test]
    fn same_row_with_part() {
        let input = "1.1\n.+.\n...";
        let nums = day3::parse_part_numbers(input);
        let mut ans = 0;
        for n in nums {
            ans += n;
        }
        assert_eq!(ans, 2);
    }

    #[test]
    fn same_row_with_part_joined() {
        let input = "1+1\n...\n...";
        let nums = day3::parse_part_numbers(input);
        let mut ans = 0;
        for n in nums {
            ans += n;
        }
        assert_eq!(ans, 2);
    }

    #[test]
    fn same_row_with_part_end() {
        let input = "1.1+\n+...\n....";
        let nums = day3::parse_part_numbers(input);
        let mut ans = 0;
        for n in nums {
            ans += n;
        }
        assert_eq!(ans, 2);
    }
}
