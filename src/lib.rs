pub mod day01;

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
                paste::item! { let ans =[<day $day>]::part2(input); }
                assert_eq!(ans, $ans2);
                Box::new(ans)
            },
        )
    };
}

pub fn get_day(day: i32) -> (DayFn, DayFn) {
    return match day {
        1 => aoc!(01, 290784, 177337980),
        _ => {
            eprintln!("Unknown day: {}", day);
            return (noop, noop);
        }
    };
}
