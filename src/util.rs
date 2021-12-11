use std::fs;

pub fn get_from_file(day: usize) -> Vec<String> {
    get_from_filename(format!("./inputs/day{}.txt", day))
}

pub fn get_test_from_file(day: usize) -> Vec<String> {
    get_from_filename(format!("./inputs/day{}test.txt", day))
}

fn get_from_filename(filename: String) -> Vec<String> {
    let lines: Vec<String> = fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .filter(|x: &&str| x.trim() != "")
        .map(|x| String::from(x.trim()))
        .collect();
    lines
}
