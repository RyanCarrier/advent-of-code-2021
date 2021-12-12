use std::fs;
pub static TRANSFORMS: [[isize; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];
pub fn get_from_file(day: usize) -> Vec<String> {
    get_from_filename(format!("./inputs/day{}.txt", day))
}
#[allow(dead_code)]
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

// pub fn get_around(point: [usize; 2], diagonal: bool, minmax: [usize; 2]) -> Vec<[usize; 2]> {}
