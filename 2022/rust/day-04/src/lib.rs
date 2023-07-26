pub fn process_part1(input: &str) -> String {
    input
        .lines()
        .map(|f| f.split(",").collect::<Vec<_>>())
        .map(|f| {
            let part_1 = *f.get(0).unwrap();
            let part_2 = *f.get(1).unwrap();

            let x = part_1
                .split("-")
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let y = part_2
                .split("-")
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            return (x, y);
        })
        .filter(|(x, y)| {
            (x.get(0).le(&y.get(0)) && x.get(1).ge(&y.get(1)))
                || (y.get(0).le(&x.get(0)) && y.get(1).ge(&x.get(1)))
        })
        .count()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    input
        .lines()
        .map(|f| f.split(",").collect::<Vec<_>>())
        .map(|f| {
            let part_1 = *f.get(0).unwrap();
            let part_2 = *f.get(1).unwrap();

            let x = part_1
                .split("-")
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let y = part_2
                .split("-")
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            return (x, y);
        })
        .filter(|(x, y)| {
            (y.get(0).ge(&x.get(0)) && y.get(0).le(&x.get(1)))
                || (x.get(0).ge(&y.get(0)) && x.get(0).le(&y.get(1)))
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        println!("{result}");
        assert_eq!(result, "2");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "4");
    }
}
