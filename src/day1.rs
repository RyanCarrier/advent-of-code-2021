pub fn part1(lines: Vec<String>) {
    let total = lines
        .windows(2)
        .fold(0, |sum, x| if x[0] < x[1] { sum + 1 } else { sum });

    println!("day1part1: {}", total);
}

fn _day1part1_old(lines: Vec<&str>) {
    let mut total = 0;
    for i in 0..lines.len() - 2 {
        if lines[i] < lines[i + 1] {
            total += 1;
        }
    }
    println!("{}", total);
}
pub fn part2(lines: Vec<String>) {
    let numbers2 = lines.iter();
    let temp: Vec<i32> = numbers2.map(|x| x.parse::<i32>().unwrap()).collect();
    let total = temp
        .windows(4)
        .fold(0, |sum, x| if x[0] < x[3] { sum + 1 } else { sum });
    println!("day1part2: {}", total);
}
