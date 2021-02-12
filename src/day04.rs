use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> impl Iterator<Item = HashMap<&str, &str>> {
    input.split("\n\n").map(|passport| {
        passport
            .split_whitespace()
            .filter_map(|key_value| {
                let mut key_value = key_value.split(':');
                let key = key_value.next()?;
                let value = key_value.next()?;

                Some((key, value))
            })
            .collect::<HashMap<_, _>>()
    })
}

fn count_valid<P>(input: String, predicate: P) -> usize
where
    P: Fn(&HashMap<&str, &str>) -> bool,
{
    parse(&input).filter(|batch| predicate(batch)).count()
}

fn in_range<T>(x: T, low: T, high: T) -> bool
where
    T: PartialOrd,
{
    x >= low && x <= high
}

pub fn part1(input: String) -> usize {
    count_valid(input, |batch| {
        let all_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]
            .iter()
            .collect::<HashSet<_>>();

        let keys = batch.keys().collect();
        let diff = all_keys.difference(&keys).collect::<Vec<_>>();
        if diff.len() > 1 {
            return false;
        }

        match diff.get(0) {
            Some(&&&diff) => diff == "cid",
            None => true,
        }
    })
}

pub fn part2(input: String) -> usize {
    count_valid(input, |batch| {
        batch
            .iter()
            .filter(|&(&key, &value)| match key {
                "byr" => match value.parse() {
                    Ok(x) => in_range(x, 1920, 2002),
                    Err(_) => false,
                },
                "iyr" => match value.parse() {
                    Ok(x) => in_range(x, 2010, 2020),
                    Err(_) => false,
                },
                "eyr" => match value.parse() {
                    Ok(x) => in_range(x, 2020, 2030),
                    Err(_) => false,
                },
                "hgt" => {
                    let (height, unit) = value.split_at(value.len() - 2);
                    let height = match height.parse::<i32>() {
                        Ok(x) => x,
                        Err(_) => return false,
                    };
                    match unit {
                        "cm" => in_range(height, 150, 193),
                        "in" => in_range(height, 59, 76),
                        _ => false,
                    }
                }
                "hcl" => {
                    let mut chars = value.chars();

                    chars.next().unwrap_or('\0') == '#'
                        && chars.filter(|c| c.is_digit(16)).count() == 6
                }
                "ecl" => matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"),
                "pid" => value.len() == 9 && value.chars().all(|c| c.is_digit(10)),
                _ => false,
            })
            .count()
            == 7
    })
}
