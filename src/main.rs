use ansi_term::Style;
use std::time::{Duration, Instant};
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
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
        [day12::part1, day12::part2],
        [day13::part1, day13::part2],
    ];
    if run_benchmark {
        benchmark(fns);
        return;
    }
    let day = fns.len();
    // run_specific(fns, day);
    run_bench(fns, 12, 2000);
}

fn run_specific(fns: Vec<[fn(Vec<String>) -> String; 2]>, n: usize) {
    let start = Instant::now();
    println!(
        "day{}part{}test:\t{}",
        n,
        1,
        fns[n - 1][0](util::get_test_from_file(n))
    );
    let test_duration = start.elapsed();
    println!(
        "day{}part{}:\t{}",
        n,
        1,
        fns[n - 1][0](util::get_from_file(n))
    );
    let part1_duration = start.elapsed() - test_duration;
    println!(
        "day{}part{}:\t{}",
        n,
        2,
        fns[n - 1][1](util::get_from_file(n))
    );
    let total_duration = start.elapsed();
    println!(
        "Completed in {}ms\t(p1t:{}μs,p1:{}μs,p2:{}μs)",
        total_duration.as_millis(),
        test_duration.as_micros(),
        part1_duration.as_micros(),
        (total_duration - test_duration - part1_duration).as_micros()
    );
}

#[allow(dead_code)]
fn run_bench(fns: Vec<[fn(Vec<String>) -> String; 2]>, n: usize, repeats: usize) {
    let input = util::get_from_file(n);
    let inputs: Vec<Vec<String>> = (0..repeats).map(|_| input.clone()).collect();
    let total_start = Instant::now();
    for input in inputs {
        let _ = fns[n - 1][0](input);
    }
    let duration1 = total_start.elapsed();

    let inputs: Vec<Vec<String>> = (0..repeats).map(|_| input.clone()).collect();
    let start2 = Instant::now();
    for input in inputs {
        let _ = fns[n - 1][1](input);
    }
    let duration2 = start2.elapsed();
    let total_duration = duration1 + duration2;
    println!("Part1 in {}μs", duration1.as_micros() / repeats as u128);
    println!("Part2 in {}μs", duration2.as_micros() / repeats as u128);
    println!(
        "Completed in {}μs",
        total_duration.as_micros() / repeats as u128
    );
    println!(
        "p1 result:{}, p2 result:{}",
        fns[n - 1][0](input.clone()),
        fns[n - 1][1](input.clone())
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
