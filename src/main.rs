use std::{env, fs};

use advent_of_code_2020::{get_day, noop};

fn main() {
    let args: Vec<_> = env::args().collect();
    let day = args
        .get(1)
        .expect("A day to be solved must be provided")
        .parse::<i32>()
        .expect("A day must be a valid integer");

    let cwd = env::current_dir().expect("There is no current working directory");
    let filename = cwd.join("input").join(format!("{:02}.txt", day));
    let input =
        fs::read_to_string(&filename).expect(&format!("Could not read input file for day {}", day));

    let to_run = get_day(day);

    println!("Day {}:", day);
    if to_run.0 != noop {
        println!("  - Part 1: {:?}", to_run.0(input.clone()));
    }
    if to_run.1 != noop {
        println!("  - Part 2: {:?}", to_run.1(input.clone()))
    }
}
