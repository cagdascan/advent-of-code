fn char_to_priority(c: char) -> Option<u32> {
    match c {
        'a'..='z' => Some(c as u32 - 'a' as u32 + 1),
        'A'..='Z' => Some(c as u32 - 'A' as u32 + 27),
        _ => None,
    }
}

pub fn process_part1(input: &str) -> String {
    input
        .lines()
        .map(|rucksack| {
            let len = rucksack.len();
            let (part_1, part_2) = rucksack.split_at(len / 2);
            let found = part_1.chars().find(|c| part_2.contains(*c)).unwrap();
            return char_to_priority(found).unwrap();
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        println!("{result}");
        assert_eq!(result, "157");
    }
}
