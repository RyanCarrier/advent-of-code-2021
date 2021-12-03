use std::fs;

pub fn get_from_file(day: i32) -> Vec<String> {
    let filename = format!("./inputs/day{}.txt", day);

    let lines: Vec<String> = fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .filter(|x: &&str| x.trim() != "")
        .map(|x| String::from(x))
        .collect();
    lines
}

pub fn get_test_from_file(day: i32) -> Vec<String> {
    let filename = format!("./inputs/day{}test.txt", day);

    let lines: Vec<String> = fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .filter(|x: &&str| x.trim() != "")
        .map(|x| String::from(x))
        .collect();
    lines
}
