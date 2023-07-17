pub fn process_part1(input: &str) -> String {
    input
        .lines()
        .map(|game| {
            let player_1_move = game.chars().nth(0).unwrap();
            let player_2_move = game.chars().nth(2).unwrap();
            let moves = (player_1_move, player_2_move);

            match moves {
                ('A', 'X') => 4,
                ('A', 'Y') => 8,
                ('A', 'Z') => 3,
                ('B', 'X') => 1,
                ('B', 'Y') => 5,
                ('B', 'Z') => 9,
                ('C', 'X') => 7,
                ('C', 'Y') => 2,
                ('C', 'Z') => 6,
                _ => 0,
            }
        })
        .sum::<u32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    input
        .lines()
        .map(|game| {
            let player_1_move = game.chars().nth(0).unwrap();
            let player_2_move = game.chars().nth(2).unwrap();

            match (player_1_move, player_2_move) {
                ('A', 'X') => 3,
                ('A', 'Y') => 4,
                ('A', 'Z') => 8,
                ('B', 'X') => 1,
                ('B', 'Y') => 5,
                ('B', 'Z') => 9,
                ('C', 'X') => 2,
                ('C', 'Y') => 6,
                ('C', 'Z') => 7,
                _ => 0,
            }
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "15");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "12");
    }
}
