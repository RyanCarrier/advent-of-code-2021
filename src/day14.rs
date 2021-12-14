// #[derive(Clone, Copy)]
struct Day14 {
    elements: Vec<u8>,
    pairs: Vec<Pair>,
}

struct Pair {
    at: [u8; 2],
    insert: u8,
}

impl Day14 {
    fn to_string(&self) -> String {
        let freq = self
            .elements
            .iter()
            .fold([0; 28], |mut freq: [usize; 28], e| {
                freq[*e as usize] += 1;
                freq
            });
        format!("{:?}", freq)
    }
    fn steps(&mut self, n: usize) {
        (0..n).for_each(|_| self.step());
    }
    fn steps_new(&mut self, n: usize) {
        let mut new: Vec<u8> = vec![];
        let mut i = 0;
        new.push(self.elements[0]);
        while i < self.elements.len() - 1 {
            new.push(self.elements[i + 1]);
            'n: for _ in 0..n {
                for p in &self.pairs {
                    if p.at[0] == new[i] && p.at[1] == new[i + 1] {
                        new.insert(i + 1, p.insert);
                        continue 'n;
                    }
                }
            }
            i += 1;
        }
        new.push(*self.elements.last().unwrap());

        self.elements = new;
    }
    #[allow(dead_code)]
    fn step(&mut self) {
        let mut i = 0;
        'e: while i < self.elements.len() - 1 {
            for p in &self.pairs {
                if p.at[0] == self.elements[i] && p.at[1] == self.elements[i + 1] {
                    self.elements.insert(i + 1, p.insert);
                    i += 2;
                    continue 'e;
                }
            }
            i += 1;
        }
    }

    #[allow(dead_code)]
    fn step_older(&mut self) {
        let mut new: Vec<u8> = vec![];
        for i in 0..self.elements.len() - 1 {
            new.push(self.elements[i]);
            for p in &self.pairs {
                if p.at[0] == self.elements[i] && p.at[1] == self.elements[i + 1] {
                    new.push(p.insert);
                }
            }
        }
        new.push(*self.elements.last().unwrap());

        self.elements = new;
    }
    fn value(&self) -> usize {
        //max 28 characters used, as we use alphabet
        let mut freq = self
            .elements
            .iter()
            .fold([0; 28], |mut freq: [usize; 28], e| {
                freq[*e as usize] += 1;
                freq
            });
        freq.sort();
        let freq_sorted: Vec<usize> = freq.into_iter().filter(|x| *x > 0).collect();
        freq_sorted.last().unwrap() - freq_sorted[0]
    }

    fn new(lines: Vec<String>) -> Self {
        let mut map: Vec<char> = vec![];
        let mut lines = lines.into_iter();
        let mut pairs: Vec<Pair> = vec![];
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
            pairs.push(Pair {
                at: [p1v as u8, p2v as u8],
                insert: pinsertv as u8,
            });
        });
        Day14 { elements, pairs }
    }
}
pub fn part1(lines: Vec<String>) -> String {
    let mut data = Day14::new(lines);
    // let mut ret = String::new();
    // ret.push('\n');
    // for i in 0..=20 {
    //     println!("step {}\t{}\n", i, data.to_string());
    //     data.step();
    // }
    // return ret;

    let mut ret = String::from("10 step:");
    data.steps_new(16);
    //16:172844
    ret.push_str(&data.value().to_string());
    ret.push('\n');
    ret
    // data.steps(30);
    // ret.push_str(", 40 step:");

    // ret.push_str(&data.value().to_string());
    // ret
}
pub fn part2(_: Vec<String>) -> String {
    String::from("NA - part 1")
}
