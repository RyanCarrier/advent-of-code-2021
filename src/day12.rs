pub fn part1(lines: Vec<String>) {
    let mut data = import(lines);
    println!("day12part1: {}", 0);
}

pub fn part2(lines: Vec<String>) {
    let mut data = import(lines);
    println!("day12part2: {}", 0);
}

fn import(lines: Vec<String>) -> Vec<Vec<usize>> {
    lines
        .iter()
        .map(|x| x.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect()
}
