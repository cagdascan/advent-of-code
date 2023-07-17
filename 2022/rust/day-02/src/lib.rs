pub fn process_part1(input: &str) -> String {
    let mut choice_score: u32 = 0;
    let mut game_score: u32 = 0;

    input.split("\n").for_each(|game| {
        println!("{:?} vs {:?}", game.chars().nth(0), game.chars().nth(2));
        let player_1 = game.chars().nth(0).unwrap();
        let player_2 = game.chars().nth(2).unwrap();

        match player_2 {
            'X' => {
                choice_score += 1;
            }
            'Y' => {
                choice_score += 2;
            }
            'Z' => {
                choice_score += 3;
            }
            _ => {}
        }

        match (player_1, player_2) {
            ('A', 'X') => game_score += 3,
            ('A', 'Y') => game_score += 6,
            ('A', 'Z') => game_score += 0,
            ('B', 'X') => game_score += 0,
            ('B', 'Y') => game_score += 3,
            ('B', 'Z') => game_score += 6,
            ('C', 'X') => game_score += 6,
            ('C', 'Y') => game_score += 0,
            ('C', 'Z') => game_score += 3,
            _ => {}
        }
    });

    return (choice_score + game_score).to_string();
}

pub fn process_part2(input: &str) -> String {
    let mut game_score: u32 = 0;

    input.lines().for_each(|game| {
        let player_1 = game.chars().nth(0).unwrap();
        let player_2 = game.chars().nth(2).unwrap();

        match (player_1, player_2) {
            ('A', 'X') => game_score += 3,
            ('A', 'Y') => game_score += 4,
            ('A', 'Z') => game_score += 8,

            ('B', 'X') => game_score += 1,
            ('B', 'Y') => game_score += 5,
            ('B', 'Z') => game_score += 9,

            ('C', 'X') => game_score += 2,
            ('C', 'Y') => game_score += 6,
            ('C', 'Z') => game_score += 7,
            _ => {}
        }
    });

    return game_score.to_string();
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
