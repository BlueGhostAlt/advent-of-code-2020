#[derive(Debug, PartialEq)]
enum Square {
    Open,
    Tree,
}

fn count_slope<M>(map: &mut [M], right: usize, down: usize) -> usize
where
    M: Iterator<Item = Square>,
{
    map.into_iter()
        .enumerate()
        .filter_map(|(index, line)| {
            let is_right_y = index % down == 0;
            if !is_right_y {
                return None;
            }

            let iteration = index / down;
            let square = line.nth(right * iteration)?;
            match square {
                Square::Tree => Some(Square::Tree),
                _ => None,
            }
        })
        .count()
}

fn parse(input: &str) -> Vec<impl Iterator<Item = Square> + '_> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .filter_map(|character| match character {
                    '.' => Some(Square::Open),
                    '#' => Some(Square::Tree),
                    _ => None,
                })
                .cycle()
        })
        .collect()
}

pub fn part1(input: String) -> usize {
    count_slope(&mut parse(&input), 3, 1)
}

pub fn part2(input: String) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(right, down)| count_slope(&mut parse(&input), *right, *down))
        .product()
}
