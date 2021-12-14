// #[derive(Clone, Copy)]
struct Day14 {
    elements: Vec<usize>,
    pairs: Vec<Vec<usize>>,
    count: Vec<usize>,
}

impl Day14 {
    fn steps_new_new_new_new_new_new_new(&mut self, n: u8) {
        //You can predict what pairs will come next, so no need to track the entire string??
        self.elements.iter().for_each(|e| self.count[*e] += 1);
        let mut pair_count = vec![vec![0 as usize; self.count.len()]; self.count.len()];
        for i in 0..self.elements.len() - 1 {
            pair_count[self.elements[i]][self.elements[i + 1]] += 1;
        }
        for _ in 0..n {
            let mut pair_count2 = vec![vec![0; self.count.len()]; self.count.len()];
            pair_count.iter().enumerate().for_each(|(pre, post_list)| {
                post_list.iter().enumerate().for_each(|(post, pc)| {
                    pair_count2[pre][self.pairs[pre][post]] += *pc;
                    pair_count2[self.pairs[pre][post]][post] += *pc;
                    self.count[self.pairs[pre][post]] += *pc;
                });
            });
            pair_count = pair_count2;
        }
    }

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
        let mid = self.pairs[pre][post];
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
        let mut pairs: Vec<Vec<usize>> = vec![vec![0; map.len()]; map.len()];
        raw_pairs.into_iter().for_each(|p| pairs[p[0]][p[1]] = p[2]);

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
    let steps = 40;
    let mut ret = format!("step {}:", steps);
    // data.steps(10);
    // 16:172844
    // data.steps(steps);
    data.steps_new_new_new_new_new_new_new(steps);
    //40:2857918149 TOO LOW
    //40:2857918149 (I think it was actually a 30 iter tho (confirmed lol))
    ret.push_str(&data.value().to_string());
    ret
}
pub fn part2(_: Vec<String>) -> String {
    String::from("NA - part 1")
}
