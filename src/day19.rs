struct Day19 {
    scanner: Vec<Vec<[i64; 3]>>,
}
impl Day19 {
    fn new(lines: Vec<String>) -> Self {
        let mut scanner = 0;
        let mut data = vec![];
        for mut l in lines {
            if l.contains("---") {
                l.drain(0..13);
                l.drain(l.len() - 3..l.len());
                scanner = l.parse::<usize>().unwrap();
                data.push(vec![]);
            }
            let mut split = l.trim().split(",");
            data[scanner].push([
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            ]);
        }
        Day19 { scanner: data }
    }
}

pub fn part1(lines: Vec<String>) -> String {
    let mut state = Day19::new(lines);
    // state.add_all();
    format!("{}", "")
}
pub fn part2(lines: Vec<String>) -> String {
    let mut state = Day19::new(lines);
    format!("{}", "")
}
