use std::collections::{HashMap, HashSet};

fn count_valid<P>(input: String, predicate: P) -> usize
where
    P: Fn(HashMap<&str, &str>) -> bool,
{
    input
        .split("\n\n")
        .filter(|passport| {
            let batch: HashMap<_, _> = passport
                .split_whitespace()
                .filter_map(|key_value| {
                    let mut key_value = key_value.split(':');
                    let key = key_value.next()?;
                    let value = key_value.next()?;
                    Some((key, value))
                })
                .collect();

            predicate(batch)
        })
        .count()
}

fn in_range<T>(x: T, low: T, high: T) -> bool
where
    T: PartialOrd,
{
    x >= low && x <= high
}

pub fn part1(input: String) -> usize {
    count_valid(input, |batch| {
        let all_keys: HashSet<_> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]
            .iter()
            .collect();

        let keys = batch.keys().collect();
        let diff: Vec<_> = all_keys.difference(&keys).collect();
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
            .into_iter()
            .filter(|&(key, value)| match key {
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
                        _ => return false,
                    }
                }
                "hcl" => {
                    let mut chars = value.chars();

                    chars.next().unwrap_or('\0') == '#'
                        && chars.filter(|c| c.is_digit(16)).count() == 6
                }
                "ecl" => match value {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                    _ => false,
                },
                "pid" => value.len() == 9 && value.chars().all(|c| c.is_digit(10)),
                _ => false,
            })
            .count()
            == 7
    })
}
