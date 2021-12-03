pub fn part1(lines: Vec<String>) {
    let mid_value = lines.len() / 2;
    let size = lines[0].len();
    let frequency: Vec<u32> = lines.iter().fold(vec![0; size], |mut freq, x| {
        for (i, y) in x.chars().enumerate() {
            if y == '1' {
                freq[i] += 1;
            }
        }
        freq
    });
    let mut original = String::new();
    for v in frequency.iter() {
        original += &(v / (mid_value as u32)).to_string();
    }
    let gamma = i64::from_str_radix(&original, 2).unwrap();
    let epsilon = (2_i64).pow(size as u32) - gamma - 1;
    println!(
        "day3part1: gamma:{}, epsilon:{}, total:{}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}
pub fn part2(lines: Vec<String>) {
    let mid_value = lines.len() / 2;
    let size = lines[0].len();
    let frequency: Vec<u32> = lines.clone().iter().fold(vec![0; size], |mut freq, x| {
        for (i, y) in x.chars().enumerate() {
            if y == '1' {
                freq[i] += 1;
            }
        }
        freq
    });
    let line_clone = lines.clone();
    let new_number = line_clone.iter();
    let i: usize = 0;
    while new_number.len() > 1 {
        new_number = new_number
            .filter(|x| {
                x.chars().nth(i).unwrap().to_string()
                    == (frequency[i] / (mid_value as u32)).to_string()
            })
            .collect();
    }
    let oxygen_rating = i64::from_str_radix(new_number.nth(0).unwrap(), 2).unwrap();

    println!("day3part2:");
}
