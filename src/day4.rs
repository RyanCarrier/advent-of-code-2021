pub fn part1(lines: Vec<String>) {
    println!("day4part1: ")
}

pub fn part2(lines: Vec<String>) {
    if lines.len() == 0 {
        println!("day4part2: ")
    }
}

fn import(lines: Vec<String>) {
    let draws = import_draws(lines.get(0).unwrap());
    let temp: Vec<_> = lines.get(2).unwrap().trim().split(" ").collect();
    let n: usize = temp.len();
}

fn import_draws(line: &String) -> Vec<u64> {
    line.split(',').map(|x| x.parse().unwrap()).collect()
}

fn import_board(lines: Vec<String>) -> Vec<Vec<u64>> {
    return vec![vec![123]];
}
