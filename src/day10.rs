
pub fn part1(lines: Vec<String>) {
    let data = import(lines);
    println!("day9part1: {}", data.len())
}

pub fn part2(lines: Vec<String>) {
    let data = import(lines);
    println!("day9part2: {}", data.len());
}

fn import(lines: Vec<String>) -> Vec<Vec<usize>> {
    lines.iter().map(|x| x.split(" ").map(|y| y.parse::<usize>().unwrap()).collect()).collect()
}
