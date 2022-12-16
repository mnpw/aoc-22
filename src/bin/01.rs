use std::io::Error;

use aoc_22::*;

fn main() -> Result<(), Error> {
    let input = read_input("input/01")?;
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));

    Ok(())
}

fn part1(input: &str) -> Option<u64> {
    let elves = parse_input(input);

    elves.iter().map(|elf| elf.total_calories()).max()
}

fn part2(input: &str) -> u64 {
    let elves = parse_input(input);
    let mut total_calories: Vec<u64> = elves.iter().map(|elf| elf.total_calories()).collect();
    total_calories.sort();

    total_calories.iter().rev().take(3).sum()
}

fn parse_input(input: &str) -> Vec<Elf> {
    let lines: Vec<&str> = input.lines().into_iter().collect();
    let elves: Vec<Elf> = lines
        .split(|calorie| calorie.is_empty())
        .map(|t| Elf::parse(t.to_vec()))
        .collect();

    elves
}

#[derive(Debug)]
struct Elf {
    calories: Vec<u64>,
}

impl Elf {
    fn parse(input: Vec<&str>) -> Elf {
        let calories = input
            .iter()
            .map(|calorie| calorie.parse().expect("Calorie could not be parsed"))
            .collect();
        Elf { calories }
    }

    fn total_calories(&self) -> u64 {
        self.calories.iter().sum()
    }
}

#[cfg(test)]
mod test {
    use indoc::*;

    use crate::*;

    #[test]
    fn test() {
        let input = indoc! {"
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
        "};

        assert_eq!(part1(input), Some(24000));
        assert_eq!(part2(input), 45000);
    }
}
