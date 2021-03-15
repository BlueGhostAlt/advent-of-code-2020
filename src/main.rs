#![feature(test)]

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;

pub mod benches;

use std::cmp;
use std::fmt;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use paste::paste;

pub const INPUT: [&str; 9] = [
    include_str!("../input/01.txt"),
    include_str!("../input/02.txt"),
    include_str!("../input/03.txt"),
    include_str!("../input/04.txt"),
    include_str!("../input/05.txt"),
    include_str!("../input/06.txt"),
    include_str!("../input/07.txt"),
    include_str!("../input/08.txt"),
    include_str!("../input/09.txt"),
];

const MILLISECOND: Duration = Duration::from_millis(1);

macro_rules! run_day_on_thread {
    ($day: expr, $ans1: expr, $ans2: expr) => {
        paste! { run_day_on_thread($day, [<day $day>]::part1, [<day $day>]::part2, $ans1, $ans2) }
    };
}

fn compute_time_took<F, R>(fun: F) -> (R, String)
where
    F: FnOnce() -> R,
{
    let now = Instant::now();

    let res = fun();

    let time_elapsed = now.elapsed().as_nanos();
    let time_elapsed = if time_elapsed > MILLISECOND.as_nanos() {
        format!("{:.2}", time_elapsed as f64 / MILLISECOND.as_nanos() as f64)
    } else {
        format!("{:.3}", time_elapsed as f64 / MILLISECOND.as_nanos() as f64)
    };

    (res, time_elapsed)
}

fn run_day_on_thread<A1, A2>(
    day: usize,
    part1: fn(&str) -> A1,
    part2: fn(&str) -> A2,
    ans1: A1,
    ans2: A2,
) -> JoinHandle<()>
where
    A1: cmp::PartialEq + fmt::Debug + Send + Sync + 'static,
    A2: cmp::PartialEq + fmt::Debug + Send + Sync + 'static,
{
    thread::spawn(move || {
        let input = INPUT[day - 1];

        let ((time_took1, time_took2), time_took_total) = compute_time_took(|| {
            let (_, time_took1) = compute_time_took(|| assert_eq!(ans1, part1(input)));

            let (_, time_took2) = compute_time_took(|| assert_eq!(ans2, part2(input)));

            (time_took1, time_took2)
        });

        println!(
            "Day {} ({}ms):\n    Part 1: {:?} ({}ms)\n    Part 2: {:?} ({}ms)",
            day, time_took_total, ans1, time_took1, ans2, time_took2
        );
    })
}

fn main() {
    let mut children = Vec::with_capacity(32);

    children.push(run_day_on_thread!(01, 290784, 177337980));
    children.push(run_day_on_thread!(02, 546, 275));
    children.push(run_day_on_thread!(03, 286, 3638606400));
    children.push(run_day_on_thread!(04, 216, 150));
    children.push(run_day_on_thread!(05, 848, 682));
    children.push(run_day_on_thread!(06, 6596, 3219));
    children.push(run_day_on_thread!(07, 119, 155802));
    children.push(run_day_on_thread!(08, 1134, 1205));
    children.push(run_day_on_thread!(09, 22477624, 2980044));

    let (_, time_took_parallel) = compute_time_took(move || {
        for child in children {
            let _handle = child.join();
        }
    });

    println!("\nTotal (parallel): {}ms", time_took_parallel);
}
