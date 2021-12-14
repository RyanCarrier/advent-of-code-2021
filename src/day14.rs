// #[derive(Clone, Copy)]
struct Day14 {
    elements: Vec<u8>,
    pairs: Vec<Vec<u8>>,
    count: Vec<usize>,
}

impl Day14 {
    fn steps(&mut self, n: u8) {
        for i in 0..self.elements.len() - 1 {
            self.partial_step(n, self.elements[i], self.elements[i + 1])
        }
    }
    fn partial_step(&mut self, n: u8, pre: u8, post: u8) {
        if n <= 0 {
            return;
        }
        let mid = self.pairs[pre as usize][post as usize];
        self.count[mid as usize] += 1;
        self.partial_step(n - 1, pre, mid);
        self.partial_step(n - 1, mid, post);
    }

    fn value(&self) -> usize {
        let mut count_sort = self.count.clone();
        count_sort.sort();
        count_sort.last().unwrap() - count_sort[0]
    }

    fn new(lines: Vec<String>) -> Self {
        let mut map: Vec<char> = vec![];
        let mut lines = lines.into_iter();
        let mut raw_pairs: Vec<[usize; 3]> = vec![];
        let mut elements: Vec<u8> = Vec::new();
        lines.next().unwrap().trim().chars().for_each(|c| {
            elements.push(match map.iter().position(|cm| cm == &c) {
                Some(i) => i as u8,
                None => {
                    map.push(c);
                    (map.len() - 1) as u8
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
        let mut pairs: Vec<Vec<u8>> = vec![vec![0; map.len()]; map.len()];
        raw_pairs
            .into_iter()
            .for_each(|p| pairs[p[0]][p[1]] = p[2] as u8);

        let d = Day14 {
            elements,
            count: vec![0; map.len()],
            pairs,
        };
        d
    }
}
pub fn part1(lines: Vec<String>) -> String {
    let mut data = Day14::new(lines);
    let steps = 28;
    let mut ret = format!("step {}:", steps);
    // data.steps(10);
    // 16:172844
    data.steps(steps);
    //40:2857918149 TOO LOW
    //40:2857918149 (I think it was actually a 30 iter)
    ret.push_str(&data.value().to_string());
    ret
}
pub fn part2(_: Vec<String>) -> String {
    String::from("NA - part 1")
}
