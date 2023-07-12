pub fn process_part1(input: &str) -> String {
    input
        .split("\n\n")
        .map(|section| {
            section
                .lines()
                .map(|el| el.parse::<u32>().expect("not a number"))
                .sum::<u32>()
        })
        .max()
        .unwrap()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut loads = input
        .split("\n\n")
        .map(|section| {
            section
                .lines()
                .map(|el| el.parse::<u32>().expect("not a number"))
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    loads.sort_by(|a, b| b.cmp(a));
    loads.iter().take(3).sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "24000");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "45000");
    }
}
