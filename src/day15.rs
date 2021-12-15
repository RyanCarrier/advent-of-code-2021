struct Day15 {}

impl Day15 {
    fn new(lines: Vec<String>) -> Self {
        let mut map: Vec<char> = vec![];
        let mut lines = lines.into_iter();
        let mut raw_pairs: Vec<[usize; 3]> = vec![];
        let mut elements: Vec<usize> = Vec::new();
        lines.next().unwrap().trim().chars().for_each(|c| {
            elements.push(match map.iter().position(|cm| cm == &c) {
                Some(i) => i,
                None => {
                    map.push(c);
                    map.len() - 1
                }
            })
        });
        //empty lines are ommited in data loader
        lines.for_each(|line| {
            let mut temp = line.split(" -> ");
            let mut pairtext = temp.next().unwrap().chars();
            let p1 = pairtext.next().unwrap();
            let p2 = pairtext.next().unwrap();
            let p1v = match map.iter().position(|c| c == &p1) {
                Some(i) => i,
                None => {
                    map.push(p1);
                    map.len() - 1
                }
            };
            let p2v = match map.iter().position(|c| c == &p2) {
                Some(i) => i,
                None => {
                    map.push(p2);
                    map.len() - 1
                }
            };

            let pinsert = temp.next().unwrap().chars().into_iter().next().unwrap();
            let pinsertv = match map.iter().position(|c| c == &pinsert) {
                Some(i) => i,
                None => {
                    map.push(pinsert);
                    map.len() - 1
                }
            };
            raw_pairs.push([p1v, p2v, pinsertv]);
        });
        let n = map.len();
        let mut pairs: Vec<usize> = vec![0; n * n];
        raw_pairs
            .into_iter()
            .for_each(|p| pairs[p[0] * n + p[1]] = p[2]);

        let d = Day15 {};
        d
    }
}
pub fn part1(lines: Vec<String>) -> String {
    let data = Day15::new(lines);
    let ret = format!("p1: {}, p2: ", "");
    ret
}
pub fn part2(_: Vec<String>) -> String {
    String::from("NA - part 1")
}
