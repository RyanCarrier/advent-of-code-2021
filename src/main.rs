use ansi_term::Style;
use std::{
    env,
    time::{Duration, Instant},
};
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
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
        [day14::part1, day14::part2],
    ];
    process_args(fns);
}

fn process_args(fns: Vec<[fn(Vec<String>) -> String; 2]>) {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        benchmark(fns);
        return;
    }
    let day: usize;
    match args[1].as_str() {
        "bench" => return benchmark(fns),
        _ => {
            day = match args[1].parse::<usize>() {
                Ok(day) => day,
                Err(_) => panic!("incorrect syntax"),
            };
        }
    }
    if args.len() < 3 {
        return run_specific(fns, day);
    }
    match args[2].as_str() {
        "bench" => {
            if args.len() < 4 {
                panic!("need repeats")
            }
            let repeats = match args[3].parse::<usize>() {
                Ok(repeats) => repeats,
                Err(_) => panic!("incorrect syntax"),
            };
            return bench_specific(fns, day, repeats);
        }
        _ => {
            match &args[2].parse::<usize>() {
                Ok(part) => {
                    if [1, 2].contains(part) {
                        return run_single(fns, day, *part - 1);
                    }
                    panic!("incorrect syntax");
                }
                Err(_) => panic!("incorrect syntax"),
            };
        }
    }
}

fn run_specific(fns: Vec<[fn(Vec<String>) -> String; 2]>, day: usize) {
    let input_test = util::get_test_from_file(day);
    let start = Instant::now();
    println!("day{}part{}test:\t{}", day, 1, fns[day - 1][0](input_test));
    let test_duration = start.elapsed();
    let input = util::get_from_file(day);
    let inputp2 = input.clone();
    let part1_start = Instant::now();
    println!("day{}part{}:\t{}", day, 1, fns[day - 1][0](input));
    let part1_duration = part1_start.elapsed();
    let part2_start = Instant::now();
    println!("day{}part{}:\t{}", day, 2, fns[day - 1][1](inputp2));
    let part2_duration = part2_start.elapsed();
    println!(
        "Completed in {}\t(p1t:{}, p1:{}, p2:{})",
        util::format_duration(test_duration + part1_duration + part2_duration),
        util::format_duration(test_duration),
        util::format_duration(part1_duration),
        util::format_duration(part2_duration)
    );
}

fn run_single(fns: Vec<[fn(Vec<String>) -> String; 2]>, day: usize, part: usize) {
    let input = util::get_from_file(day);
    let start = Instant::now();
    let result = fns[day - 1][part](input);
    let duration = start.elapsed();
    println!(
        "day{}part{}:\t{}\nCompleted in {}",
        day,
        part + 1,
        result,
        util::format_duration(duration),
    );
}

fn bench_specific(fns: Vec<[fn(Vec<String>) -> String; 2]>, day: usize, repeats: usize) {
    let input = util::get_from_file(day);
    let mut inputs: Vec<Vec<String>> = (0..repeats).map(|_| input.clone()).collect();
    let total_start = Instant::now();
    for input in inputs {
        fns[day - 1][0](input);
    }
    let duration1 = total_start.elapsed();

    inputs = (0..repeats).map(|_| input.clone()).collect();
    let start2 = Instant::now();
    for input in inputs {
        fns[day - 1][1](input);
    }
    let duration2 = start2.elapsed();
    let total_duration = duration1 + duration2;
    println!(
        "Day {} Part1 in {}",
        day,
        util::format_duration(duration1.div_f64(repeats as f64))
    );
    println!(
        "Day {} Part2 in {}",
        day,
        util::format_duration(duration2.div_f64(repeats as f64))
    );
    println!(
        "Completed day {} in {}",
        day,
        util::format_duration(total_duration.div_f64(repeats as f64))
    );
    println!(
        "Day {} p1 result:{}, p2 result:{}",
        day,
        fns[day - 1][0](input.clone()),
        fns[day - 1][1](input.clone())
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
