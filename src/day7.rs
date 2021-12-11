use std::cmp;

#[derive(Clone)]
struct CrabMath {
    values: Vec<usize>,
}
impl CrabMath {
    //must initialise with 0,1 otherwise crab math needs another comparison every get
    fn get(&mut self, n: usize) -> usize {
        if self.values.len() > n {
            return self.values[n];
        }
        let prev = self.get(n - 1);
        self.values.push(prev + n);
        self.get(n)
    }
}

#[derive(Clone)]
struct Crabs {
    subs: Vec<i64>,
    branches: [Branch; 3],
    part2: bool,
    math: CrabMath,
}

impl Crabs {
    fn calculate(&mut self, point: i64) -> i64 {
        if self.part2 {
            self.subs.clone().iter().fold(0, |total, x| {
                total + self.math.get((*x - point).abs() as usize) as i64
            })
        } else {
            self.subs
                .clone()
                .iter()
                .fold(0, |total, x| total + (*x - point).abs())
        }
    }
    fn calculate_branch(&mut self, which: usize) {
        self.branches[which].value = self.calculate(self.branches[which].index);
    }
    fn update_branch(&mut self, which: usize, point: i64) {
        self.branches[which].index = point;
        self.branches[which].value = self.calculate(point);
    }
    fn update_midbranch(&mut self) {
        self.update_branch(1, (self.branches[0].index + self.branches[2].index) / 2);
    }
    fn converge(&mut self) {
        while self.branches[0].value != self.branches[2].value {
            // println!(
            //     "{} {} {} -> {} {} {}",
            //     self.branches[0].index,
            //     self.branches[1].index,
            //     self.branches[2].index,
            //     self.branches[0].value,
            //     self.branches[1].value,
            //     self.branches[2].value,
            // );
            if self.branches[0].value < self.branches[2].value {
                self.branches[2] = self.branches[1].clone();
            } else {
                self.branches[0] = self.branches[1].clone();
            }
            self.update_midbranch();
        }
    }
}

#[derive(Clone)]
struct Branch {
    index: i64,
    value: i64,
}

pub fn part1(lines: Vec<String>) -> String {
    let mut crabs = import(lines, false);
    crabs.converge();
    crabs.branches[1].value.to_string()
}

pub fn part2(lines: Vec<String>) -> String {
    let mut crabs = import(lines, true);
    crabs.part2 = true;
    crabs.converge();
    crabs.branches[1].value.to_string()
}

fn import(lines: Vec<String>, part2: bool) -> Crabs {
    let subs: Vec<i64> = lines[0].split(",").map(|x| x.parse().unwrap()).collect();
    let minmax: [i64; 2] = subs.clone().iter().fold([0, 0], |minmax, x| {
        [cmp::min(minmax[0], *x), cmp::max(minmax[1], *x)]
    });
    let mut crabbies = Crabs {
        subs,
        branches: [
            Branch {
                index: minmax[0],
                value: 0,
            },
            Branch {
                index: minmax.iter().sum::<i64>() / 2,
                value: 0,
            },
            Branch {
                index: minmax[1],
                value: 0,
            },
        ],
        math: CrabMath { values: vec![0, 1] },
        part2,
    };
    crabbies.calculate_branch(0);
    crabbies.calculate_branch(1);
    crabbies.calculate_branch(2);
    crabbies
}
