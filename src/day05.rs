fn parse_row(row_str: &str) -> u8 {
    let mut row = 0u8;
    row_str.chars().for_each(|c| {
        row <<= 1;
        if let 'B' = c {
            row += 1
        }
    });

    row
}

fn parse_column(column_str: &str) -> u8 {
    let mut column = 0u8;
    column_str.chars().for_each(|c| {
        column <<= 1;
        if let 'R' = c {
            column += 1
        }
    });

    column
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let row = parse_row(&line[..7]) as usize;
            let column = parse_column(&line[7..]) as usize;

            row * 8 + column
        })
        .max()
        .unwrap_or(0)
}

pub fn part2(input: &str) -> usize {
    let mut seat_ids: Vec<_> = input
        .lines()
        .map(|line| {
            let row = parse_row(&line[..7]) as usize;
            let column = parse_column(&line[7..]) as usize;

            row * 8 + column
        })
        .collect();

    seat_ids.sort_unstable();

    seat_ids
        .windows(2)
        .filter_map(|ids| {
            if let [id1, id2] = ids {
                match id2 - id1 > 1 {
                    true => return Some(id1 + 1),
                    false => return None,
                };
            }

            None
        })
        .next()
        .unwrap_or(0)
}
