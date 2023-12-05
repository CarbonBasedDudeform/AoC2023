#[cfg(test)]
mod tests {
    use crate::day2;
    use crate::day2::Limits;

    #[test]
    fn perfect_game_is_possible() {
        let input = "Game 1: 4 red, 5 blue, 4 green;";
        let game = day2::parse_game(input);
        let limits = Limits{red: 100, green: 100, blue: 100};

        assert_eq!(day2::is_possible(game, limits), true);
    }

    #[test]
    fn perfect_impossible_game_is_impossible() {
        let input = "Game 1: 4 red, 5 blue, 4 green;";
        let game = day2::parse_game(input);
        let limits = Limits{red: 1, green: 1, blue: 1};

        assert_eq!(day2::is_possible(game, limits), false);
    }

    #[test]
    fn multi_game_is_possible() {
        let input = "Game 1: 4 red, 5 blue, 4 green; 4 red, 5 blue, 4 green;";
        let game = day2::parse_game(input);
        let limits = Limits{red: 100, green: 100, blue: 100};

        assert_eq!(day2::is_possible(game, limits), true);
    }

    #[test]
    fn impossible_multi_game_is_impossible() {
        let input = "Game 1: 4 red, 5 blue, 4 green; 101 red, 5 blue, 4 green;";
        let game = day2::parse_game(input);
        let limits = Limits{red: 100, green: 100, blue: 100};

        assert_eq!(day2::is_possible(game, limits), false);
    }
}
