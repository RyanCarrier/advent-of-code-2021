pub fn part1(lines: Vec<String>) -> String {
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
    format!(
        "gamma:{}, epsilon:{}, total:{}",
        gamma,
        epsilon,
        gamma * epsilon,
    )
}

fn cmp<'a>(a: u32, midpoint: f64) -> &'a str {
    if (a as f64) < midpoint {
        return "0";
    }
    return "1";
}

pub fn part2(lines: Vec<String>) -> String {
    let size = lines[0].len();

    let mut mid_value: f64;
    let mut frequency: Vec<u32>;
    let mut frequencylist: Vec<&str>;

    // println!("Freq:{:?}, midpoint:{}", frequency, mid_value);
    let line_clone = lines.clone();
    let mut oxygen_lines: Vec<String> = line_clone;
    let mut i: usize = 0;
    while oxygen_lines.len() > 1 && i < size {
        mid_value = oxygen_lines.len() as f64 / 2 as f64;
        frequency = oxygen_lines
            .clone()
            .iter()
            .fold(vec![0; size], |mut freq, x| {
                for (i, y) in x.chars().enumerate() {
                    if y == '1' {
                        freq[i] += 1;
                    }
                }
                freq
            });
        frequencylist = frequency.iter().map(|x| cmp(*x, mid_value)).collect();
        // println!("Freq:{:?}, midpoint:{}", frequency, mid_value);
        // println!("-------:{:?}", frequencylist);

        oxygen_lines = oxygen_lines
            .into_iter()
            .filter(|x| {
                x.chars().nth(i).unwrap().to_string() == frequencylist[i]
                //maybe issue here with midvalue being even/uneven
            })
            .collect();

        // println!("\t{:?}", oxygen_lines);
        i += 1
    }
    // println!("{}", oxygen_lines.get(0).unwrap());
    let oxygen_rating = i64::from_str_radix(oxygen_lines.get(0).unwrap(), 2).unwrap();
    //////////////////////////////////////////////////////////////////////////////
    let co2_lines_clone = lines.clone();
    let mut co2_lines = co2_lines_clone;
    let mut i: usize = 0;
    while co2_lines.len() > 1 && i < size {
        mid_value = co2_lines.len() as f64 / 2 as f64;
        frequency = co2_lines.clone().iter().fold(vec![0; size], |mut freq, x| {
            for (i, y) in x.chars().enumerate() {
                if y == '1' {
                    freq[i] += 1;
                }
            }
            freq
        });
        frequencylist = frequency.iter().map(|x| cmp(*x, mid_value)).collect();
        // println!("Freq:{:?}, midpoint:{}", frequency, mid_value);
        // println!("-------:{:?}", frequencylist);

        co2_lines = co2_lines
            .into_iter()
            .filter(|x| {
                x.chars().nth(i).unwrap().to_string() != frequencylist[i]
                //maybe issue here with midvalue being even/uneven
            })
            .collect();
        // println!("\t{:?}", co2_lines);
        i += 1;
    }
    // println!("{}", co2_lines.get(0).unwrap());
    let co2_rating = i64::from_str_radix(co2_lines.get(0).unwrap(), 2).unwrap();

    format!(
        "oxygen:{}, co2:{}, o*x:{}",
        oxygen_rating,
        co2_rating,
        oxygen_rating * &co2_rating
    )
}
