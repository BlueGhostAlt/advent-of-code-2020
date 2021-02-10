fn parse(input: &str) -> impl Iterator<Item = (usize, usize, char, &str)> {
    input.lines().filter_map(|line| {
        let mut components = line.split(' ');
        let range = components.next()?;
        let character = components.next()?;
        let password = components.next()?;

        let mut range = range.split('-');
        let range_first: usize = range.next()?.parse().ok()?;
        let range_second: usize = range.next()?.parse().ok()?;

        let character = character.chars().next()?;

        Some((range_first, range_second, character, password))
    })
}

pub fn part1(input: String) -> usize {
    let input = parse(&input);

    input
        .filter(|(min, max, character, password)| {
            let count = password.chars().filter(|c| c == character).count();

            count >= *min && count <= *max
        })
        .count()
}

pub fn part2(input: String) -> usize {
    let input = parse(&input);

    input
        .filter(|(first, second, character, password)| {
            let first = password.chars().nth(*first - 1).unwrap() == *character;
            let second = password.chars().nth(*second - 1).unwrap() == *character;

            first ^ second
        })
        .count()
}
