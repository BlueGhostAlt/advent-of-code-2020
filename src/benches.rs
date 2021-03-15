extern crate test;

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    use crate::INPUT;

    use crate::day01;
    use crate::day02;
    use crate::day03;
    use crate::day04;
    use crate::day05;
    use crate::day06;
    use crate::day07;
    use crate::day08;
    use crate::day09;

    #[bench]
    fn bench_day01_part1(b: &mut Bencher) {
        b.iter(|| day01::part1(INPUT[0]));
    }
    #[bench]
    fn bench_day01_part2(b: &mut Bencher) {
        b.iter(|| day01::part2(INPUT[0]));
    }

    #[bench]
    fn bench_day02_part1(b: &mut Bencher) {
        b.iter(|| day02::part1(INPUT[1]));
    }
    #[bench]
    fn bench_day02_part2(b: &mut Bencher) {
        b.iter(|| day02::part2(INPUT[1]));
    }

    #[bench]
    fn bench_day03_part1(b: &mut Bencher) {
        b.iter(|| day03::part1(INPUT[2]));
    }
    #[bench]
    fn bench_day03_part2(b: &mut Bencher) {
        b.iter(|| day03::part2(INPUT[2]));
    }

    #[bench]
    fn bench_day04_part1(b: &mut Bencher) {
        b.iter(|| day04::part1(INPUT[3]));
    }
    #[bench]
    fn bench_day04_part2(b: &mut Bencher) {
        b.iter(|| day04::part2(INPUT[3]));
    }

    #[bench]
    fn bench_day05_part1(b: &mut Bencher) {
        b.iter(|| day05::part1(INPUT[4]));
    }
    #[bench]
    fn bench_day05_part2(b: &mut Bencher) {
        b.iter(|| day05::part2(INPUT[4]));
    }

    #[bench]
    fn bench_day06_part1(b: &mut Bencher) {
        b.iter(|| day06::part1(INPUT[5]));
    }
    #[bench]
    fn bench_day06_part2(b: &mut Bencher) {
        b.iter(|| day06::part2(INPUT[5]));
    }

    #[bench]
    fn bench_day07_part1(b: &mut Bencher) {
        b.iter(|| day07::part1(INPUT[6]));
    }
    #[bench]
    fn bench_day07_part2(b: &mut Bencher) {
        b.iter(|| day07::part2(INPUT[6]));
    }

    #[bench]
    fn bench_day08_part1(b: &mut Bencher) {
        b.iter(|| day08::part1(INPUT[7]));
    }
    #[bench]
    fn bench_day08_part2(b: &mut Bencher) {
        b.iter(|| day08::part2(INPUT[7]));
    }

    #[bench]
    fn bench_day09_part1(b: &mut Bencher) {
        b.iter(|| day09::part1(INPUT[8]));
    }
    #[bench]
    fn bench_day09_part2(b: &mut Bencher) {
        b.iter(|| day09::part2(INPUT[8]));
    }
}
