use std::collections::HashSet;

pub fn part1(input: String) -> usize {
    let nums: HashSet<usize> = input.lines().map(|x| x.parse().unwrap()).collect();
    for &x in nums.iter() {
        let y = 2020 - x;
        if nums.contains(&y) {
            return x * y;
        }
    }
    0
}

pub fn part2(input: String) -> usize {
    let nums: HashSet<usize> = input.lines().map(|x| x.parse().unwrap()).collect();
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
    0
}
