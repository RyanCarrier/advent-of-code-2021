pub fn part1(lines: Vec<String>) -> String {
    let mut data = import(lines);
    data[0][0].to_string()
}

pub fn part2(lines: Vec<String>) -> String {
    let mut data = import(lines);
    data[0][0].to_string()
}

fn import(lines: Vec<String>) -> Vec<Vec<usize>> {
    lines
        .iter()
        .map(|x| x.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect()
}
