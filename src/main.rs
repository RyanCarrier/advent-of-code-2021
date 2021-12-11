use ansi_term::Style;
use std::time::{Duration, Instant};
mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod util;

fn main() {
    let run_benchmark = !true;
    let fns = vec![
        [day1::part1, day1::part2],
        [day2::part1, day2::part2],
        [day3::part1, day3::part2],
        [day4::part1, day4::part2],
        [day5::part1, day5::part2],
        [day6::part1, day6::part2],
        [day7::part1, day7::part2],
        [day8::part1, day8::part2],
        [day9::part1, day9::part2],
        [day10::part1, day10::part2],
        [day11::part1, day11::part2],
        // [day12::part1, day12::part2],
    ];
    if run_benchmark {
        benchmark(fns);
        return;
    }
    let day = fns.len();
    // run_specific(fns, day);
    run_bench(fns, 4, 50);
}

fn run_specific(fns: Vec<[fn(Vec<String>) -> String; 2]>, n: usize) {
    let total_start = Instant::now();
    println!(
        "day{}part{}test:\t{}",
        n,
        1,
        fns[n - 1][0](util::get_test_from_file(n))
    );
    println!(
        "day{}part{}:\t{}",
        n,
        1,
        fns[n - 1][0](util::get_from_file(n))
    );
    println!(
        "day{}part{}:\t{}",
        n,
        2,
        fns[n - 1][1](util::get_from_file(n))
    );
    let total_duration = total_start.elapsed();
    println!("Completed in {}ms", total_duration.as_millis());
}

fn run_bench(fns: Vec<[fn(Vec<String>) -> String; 2]>, n: usize, repeats: usize) {
    let total_start = Instant::now();
    let mut list = vec![0; repeats];
    for i in 0..repeats {
        list[i] = fns[n - 1][0](util::get_from_file(n)).len();
    }
    let duration1 = total_start.elapsed();

    let start2 = Instant::now();
    for i in 0..repeats {
        list[i] = fns[n - 1][1](util::get_from_file(n)).len();
    }
    let duration2 = start2.elapsed();
    let total_duration = total_start.elapsed();
    println!("Part1 in {}ms", duration1.as_millis() / repeats as u128);
    println!("Part2 in {}ms", duration2.as_millis() / repeats as u128);
    println!(
        "Completed in {}ms",
        total_duration.as_millis() / repeats as u128
    );
    println!(
        "p1 result:{}, p2 result:{}",
        fns[n - 1][0](util::get_from_file(n)),
        fns[n - 1][1](util::get_from_file(n))
    );
}

fn benchmark(fns: Vec<[fn(Vec<String>) -> String; 2]>) {
    let mut data = vec![vec![String::new()]; fns.len()];
    for day in 0..fns.len() {
        data[day] = util::get_from_file(day + 1);
    }
    let mut durations = vec![Duration::default(); fns.len()];

    let total_start = Instant::now();
    for day in 0..fns.len() {
        let part1 = data[day].clone();
        let part2 = data[day].clone();
        let start = Instant::now();
        fns[day][0](part1);
        fns[day][1](part2);
        durations[day] = start.elapsed();
    }
    let total_duration = total_start.elapsed();
    println!("{}", Style::new().bold().paint("Day durations"));
    for day in 0..fns.len() {
        println!(
            "Day {}:\t{:.1}ms",
            day + 1,
            durations[day].as_micros() as f64 / 1000 as f64
        );
    }
    println!(
        "{}",
        Style::new().bold().paint(format!(
            "Total:\t{}ms",
            total_duration.as_millis().to_string()
        ))
    );
}
