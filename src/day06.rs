use std::collections::HashSet;

pub fn part1(input: String) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let mut answers = group.lines().flat_map(|l| l.chars()).collect::<Vec<_>>();
            answers.sort_unstable();
            answers.dedup();

            answers.len()
        })
        .sum()
}

pub fn part2(input: String) -> usize {
    input.split("\n\n").filter_map(|group| {
        let answers = group.lines().fold(None, |acc, line| {
            let chars = line.chars().collect::<HashSet<_>>();

            if let Some(acc) = acc {
                Some(chars.intersection(&acc).copied().collect())
            } else {
                Some(chars)
            }
        })?;

        Some(answers.len())
    }).sum()


}
