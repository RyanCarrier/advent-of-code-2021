struct Day14 {
    elements: Vec<usize>,
    pairs: Vec<usize>,
    count: Vec<usize>,
    n: usize,
}

impl Day14 {
    fn steps_new_new_new_new_new_new_new(&mut self, n: u8) {
        //You can predict what pairs will come next, so no need to track the entire string??
        self.elements.iter().for_each(|e| self.count[*e] += 1);
        let cl = self.n;
        let mut pair_count = vec![0 as usize; cl * cl];
        for i in 0..self.elements.len() - 1 {
            pair_count[self.elements[i] * cl + self.elements[i + 1]] += 1;
        }
        let mut pair_count2: Vec<usize>;
        for _ in 0..n {
            pair_count2 = vec![0; cl * cl];
            for pre in 0..cl {
                for post in 0..cl {
                    let pc = pair_count[pre * cl + post];
                    let mid = self.pairs[pre * cl + post];
                    pair_count2[pre * cl + mid] += pc;
                    pair_count2[mid * cl + post] += pc;
                    self.count[mid] += pc;
                }
            }
            pair_count = pair_count2;
        }
    }

    #[allow(dead_code)]
    fn steps(&mut self, n: u8) {
        for i in 0..self.elements.len() - 1 {
            self.partial_step(n, self.elements[i], self.elements[i + 1])
        }
    }
    fn partial_step(&mut self, n: u8, pre: usize, post: usize) {
        //split up massive amounts of processing into tiny chunks so no memory issues? also no copying or inserting slowdown
        if n <= 0 {
            return;
        }
        let mid = self.pairs[pre * self.n + post];
        self.count[mid] += 1;
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

        let d = Day14 {
            elements,
            count: vec![0; n],
            pairs,
            n,
        };
        d
    }
}
pub fn part1(lines: Vec<String>) -> String {
    let mut data = Day14::new(lines);
    data.steps_new_new_new_new_new_new_new(10);
    let mut ret = format!("p1: {}, p2: ", &data.value().to_string());
    data.steps_new_new_new_new_new_new_new(30);
    ret.push_str(&data.value().to_string());
    ret
}
pub fn part2(_: Vec<String>) -> String {
    String::from("NA - part 1")
}
