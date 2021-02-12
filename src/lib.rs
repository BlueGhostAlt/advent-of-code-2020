#![feature(str_split_once)]

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

pub fn noop(_input: String) -> Box<dyn std::fmt::Debug> {
    Box::new(())
}

pub type DayFn = fn(String) -> Box<dyn std::fmt::Debug>;

macro_rules! aoc {
    ($day: expr, $ans1: expr, $ans2: expr) => {
        (
            |input| {
                paste::item! { let ans = [<day $day>]::part1(input); }
                assert_eq!(ans, $ans1);
                Box::new(ans)
            },
            |input| {
                paste::item! { let ans = [<day $day>]::part2(input); }
                assert_eq!(ans, $ans2);
                Box::new(ans)
            },
        )
    };
}

pub fn get_day(day: i32) -> (DayFn, DayFn) {
    return match day {
        1 => aoc!(01, 290784, 177337980),
        2 => aoc!(02, 546, 275),
        3 => aoc!(03, 286, 3638606400),
        4 => aoc!(04, 216, 150),
        5 => aoc!(05, 848, 682),
        6 => aoc!(06, 6596, 3219),
        7 => aoc!(07, 119, 155802),
        _ => {
            eprintln!("Unknown day: {}", day);
            return (noop, noop);
        }
    };
}
