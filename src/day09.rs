enum Part {
    Part1,
    Part2,
}

fn parse(input: &str) -> Vec<u64> {
    input.lines().filter_map(|s| s.parse().ok()).collect()
}

fn solve(input: &str, part: Part) -> u64 {
    let input = parse(input);

    let part1_result = input
        .windows(26)
        .find_map(|nums| {
            let to_check_and_nums = nums.split_last();

            match to_check_and_nums {
                None => unreachable!(),
                Some((&to_check, nums)) => {
                    for x in nums {
                        for y in nums {
                            if x + y == to_check {
                                return None;
                            }
                        }
                    }

                    Some(to_check)
                }
            }
        })
        .unwrap_or_else(|| panic!("Could not find any weakness!"));

    if matches!(part, Part::Part1) {
        return part1_result;
    }

    for len in 2.. {
        let sections = input.windows(len);

        for section in sections {
            if section.iter().sum::<u64>() == part1_result {
                let min = section
                    .iter()
                    .min()
                    .expect("Could not find a minimum value in the set");
                let max = section
                    .iter()
                    .max()
                    .expect("Could not find a maximum value in the set");

                return min + max;
            }
        }
    }

    unreachable!()
}

pub fn part1(input: &str) -> u64 {
    solve(input, Part::Part1)
}

pub fn part2(input: &str) -> u64 {
    solve(input, Part::Part2)
}
