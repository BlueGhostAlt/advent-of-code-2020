use std::collections::HashSet;

fn parse(input: &str) -> HashSet<usize> {
    input.lines().filter_map(|x| x.parse().ok()).collect()
}

pub fn part1(input: &str) -> usize {
    let nums = parse(input);

    for &x in nums.iter() {
        let y = 2020 - x;
        if nums.contains(&y) {
            return x * y;
        }
    }

    unreachable!()
}

pub fn part2(input: &str) -> usize {
    let nums = parse(input);

    for &x in nums.iter() {
        for &y in nums.iter() {
            if x + y > 2020 {
                continue;
            }
            let z = 2020 - x - y;
            if nums.contains(&z) {
                return x * y * z;
            }
        }
    }

    unreachable!()
}
