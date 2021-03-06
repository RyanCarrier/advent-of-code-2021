use std::{cmp::Ordering, collections::HashSet};
#[allow(dead_code)]
struct Day13 {
    folds: Vec<Fold>,
    grid: Vec<Vec<bool>>,
}
#[derive(Clone, Copy)]
struct Fold {
    x: bool,
    n: usize,
}

struct Day13Attempt3 {
    points: Vec<[usize; 2]>,
    folds: Vec<Fold>,
}
impl Day13Attempt3 {
    pub fn new(lines: Vec<String>) -> Self {
        let mut points: Vec<[usize; 2]> = vec![];
        let mut lines = lines.iter();
        let mut line = lines.next().unwrap().trim();
        while line.contains(',') {
            let mut int_list = line.split(",");
            points.push([
                int_list.next().unwrap().parse().unwrap(),
                int_list.next().unwrap().parse().unwrap(),
            ]);
            line = lines.next().unwrap().trim();
        }

        let mut folds: Vec<Fold> = lines
            .map(|l| {
                let mut temp = l.split(" ").last().unwrap().split("=");
                Fold {
                    x: temp.next().unwrap() == "x",
                    n: temp.next().unwrap().parse().unwrap(),
                }
            })
            .collect();
        let mut temp = line.split(" ").last().unwrap().split("=");
        folds.insert(
            0,
            Fold {
                x: temp.next().unwrap() == "x",
                n: temp.next().unwrap().parse().unwrap(),
            },
        );
        Day13Attempt3 { points, folds }
    }
    fn fold_first(&mut self) -> usize {
        self.fold();
        self.points.sort_unstable();
        self.points.dedup();
        self.points.len()
    }

    fn fold_all(&mut self) {
        while self.folds.len() > 0 {
            self.fold();
        }
    }

    fn fold(&mut self) {
        let fold = self.folds.remove(0);
        let mutator = fold.n * 2;
        self.points
            .retain(|p| (fold.x && p[0] != fold.n) || (!fold.x && p[1] != fold.n));
        self.points.iter_mut().for_each(|p| {
            if fold.x && p[0] > fold.n {
                p[0] = mutator - p[0]
            } else if !fold.x && p[1] > fold.n {
                p[1] = mutator - p[1]
            }
        });
    }

    #[allow(dead_code)]
    fn to_string_transformed(&mut self) -> String {
        let mut ret = String::new();
        self.points.sort_by(|p1, p2| {
            if p1[1] < p2[1] || (p1[1] == p2[1] && p1[0] < p2[0]) {
                return Ordering::Greater;
            } else if p1[0] == p2[0] && p1[1] == p2[1] {
                return Ordering::Equal;
            }
            Ordering::Less
        });
        self.points.dedup();
        let max_index = self
            .points
            .iter()
            .fold([0, 0], |max, p| [max[0].max(p[0]), max[1].max(p[1])]);
        let mut point = self.points.pop().unwrap();
        for y in 0..=max_index[1] {
            ret.push('\n');
            for x in 0..=max_index[0] {
                if point[0] == x && point[1] == y {
                    ret.push('#');
                    point = match self.points.pop() {
                        Some(p) => p,
                        None => return ret,
                    };
                } else {
                    ret.push(' ');
                }
            }
        }
        return ret;
    }
}
#[allow(dead_code)]
struct Day13Attempt2 {
    points: HashSet<[usize; 2]>,
    folds: Vec<Fold>,
}
#[allow(dead_code)]
impl Day13Attempt2 {
    fn fold_first(&mut self) -> usize {
        self.fold();
        self.points.len()
    }

    fn fold_all(&mut self) {
        while self.folds.len() > 0 {
            self.fold();
        }
    }

    fn fold(&mut self) {
        let fold = self.folds[0];
        self.folds.remove(0);
        let mutator = fold.n * 2;
        if fold.x {
            let (update_set, old_set): (HashSet<[usize; 2]>, HashSet<[usize; 2]>) =
                self.points.iter().partition(|p| p[0] >= fold.n);
            self.points = old_set;
            update_set
                .into_iter()
                .filter(|p| p[0] != fold.n)
                .for_each(|p| {
                    self.points.insert([mutator - p[0], p[1]]);
                });
        } else {
            let (update_set, old_set): (HashSet<[usize; 2]>, HashSet<[usize; 2]>) =
                self.points.iter().partition(|p| p[1] >= fold.n);
            self.points = old_set;
            update_set
                .into_iter()
                .filter(|p| p[1] != fold.n)
                .for_each(|p| {
                    self.points.insert([p[0], mutator - p[1]]);
                });
        }
    }

    #[allow(dead_code)]
    fn to_string_transformed(&self) -> String {
        let max_index = self.points.iter().fold([0, 0], |mut max, p| {
            if p[0] > max[0] {
                max[0] = p[0];
            }
            if p[1] > max[1] {
                max[1] = p[1]
            }
            max
        });

        let mut ret = String::new();
        for y in 0..=max_index[1] {
            ret.push('\n');
            for x in 0..=max_index[0] {
                ret.push(if self.points.get(&[x, y]).is_some() {
                    'x'
                } else {
                    ' '
                });
            }
        }
        return ret;
    }
}
#[allow(dead_code)]
impl Day13 {
    fn fold_first(&mut self) -> usize {
        self.fold(self.folds[0]);
        self.folds.remove(0);
        self.grid
            .iter()
            .flatten()
            .fold(0, |x, b| if *b { x + 1 } else { x })
    }

    fn fold_all(&mut self) {
        for f in self.folds.clone() {
            self.fold(f);
        }
    }

    fn fold(&mut self, fold: Fold) {
        let mut new_grid: Vec<Vec<bool>>;
        if fold.x {
            new_grid = vec![vec![false; self.grid[0].len()]; fold.n];
            'x: for x in 0..self.grid.len() {
                for y in 0..self.grid[0].len() {
                    if x == fold.n {
                        continue 'x;
                    }
                    if self.grid[x][y] {
                        if x < fold.n {
                            new_grid[x][y] = true;
                        }
                        if x > fold.n {
                            new_grid[fold.n * 2 - x][y] = true;
                        }
                    }
                }
            }
        } else {
            new_grid = vec![vec![false; fold.n]; self.grid.len()];
            'y: for y in 0..self.grid[0].len() {
                for x in 0..self.grid.len() {
                    if y == fold.n {
                        continue 'y;
                    }
                    if self.grid[x][y] {
                        if y < fold.n {
                            new_grid[x][y] = true;
                        }
                        if y > fold.n {
                            new_grid[x][fold.n * 2 - y] = true;
                        }
                    }
                }
            }
        }
        self.grid = new_grid;
    }

    fn to_string_transformed(&self) -> String {
        let mut ret = String::new();
        for y in 0..self.grid[0].len() {
            ret.push('\n');
            for x in 0..self.grid.len() {
                ret.push(if self.grid[x][y] { 'x' } else { ' ' });
            }
        }
        return ret;
    }
}
#[allow(dead_code)]
pub fn part1_old_old(lines: Vec<String>) -> String {
    let mut data = import(lines);
    let mut p1 = String::from("part1:");
    p1.push_str(&data.fold_first().to_string());
    p1.push_str("\npart2:\n");
    data.fold_all();
    p1.push_str(&data.to_string_transformed());
    p1.push('\n');
    p1
}
#[allow(dead_code)]
pub fn part1_old(lines: Vec<String>) -> String {
    let mut data = import_attempt2(lines);

    let mut p1 = String::from("part1:");
    p1.push_str(&data.fold_first().to_string());
    p1.push_str("\npart2:\n");
    data.fold_all();
    p1.push_str(&data.to_string_transformed());
    p1.push('\n');
    p1
}
pub fn part1(lines: Vec<String>) -> String {
    let mut data = Day13Attempt3::new(lines);
    let mut p1 = String::from("part1:");
    p1.push_str(&data.fold_first().to_string());
    p1.push_str("\npart2:\n");
    data.fold_all();
    p1.push_str(&data.to_string_transformed());
    p1.push('\n');
    p1
    // String::from("NA - part 1")
}
pub fn part2(_: Vec<String>) -> String {
    String::from("NA - part 1")
}

fn import(lines: Vec<String>) -> Day13 {
    let mut points: Vec<[usize; 2]> = vec![];
    let mut lines = lines.iter();
    let mut line = lines.next().unwrap().trim();
    while line.contains(',') {
        let mut int_list = line.split(",");
        points.push([
            int_list.next().unwrap().parse().unwrap(),
            int_list.next().unwrap().parse().unwrap(),
        ]);

        line = lines.next().unwrap().trim();
    }

    let mut folds: Vec<Fold> = lines
        .map(|l| {
            let mut temp = l.split(" ").last().unwrap().split("=");
            Fold {
                x: temp.next().unwrap() == "x",
                n: temp.next().unwrap().parse().unwrap(),
            }
        })
        .collect();
    let mut temp = line.split(" ").last().unwrap().split("=");
    folds.insert(
        0,
        Fold {
            x: temp.next().unwrap() == "x",
            n: temp.next().unwrap().parse().unwrap(),
        },
    );

    let mut max = points.iter().fold([0, 0], |mut max, p| {
        if p[0] > max[0] {
            max[0] = p[0];
        }
        if p[1] > max[1] {
            max[1] = p[1]
        }
        max
    });
    max = [max[0] + 1, max[1] + 1];
    let mut grid: Vec<Vec<bool>> = vec![vec![false; max[1]]; max[0]];
    points.iter().for_each(|p| grid[p[0]][p[1]] = true);
    Day13 { folds, grid: grid }
}

fn import_attempt2(lines: Vec<String>) -> Day13Attempt2 {
    let mut points: HashSet<[usize; 2]> = HashSet::new();
    let mut lines = lines.iter();
    let mut line = lines.next().unwrap().trim();
    while line.contains(',') {
        let mut int_list = line.split(",");
        points.insert([
            int_list.next().unwrap().parse().unwrap(),
            int_list.next().unwrap().parse().unwrap(),
        ]);
        line = lines.next().unwrap().trim();
    }

    let mut folds: Vec<Fold> = lines
        .map(|l| {
            let mut temp = l.split(" ").last().unwrap().split("=");
            Fold {
                x: temp.next().unwrap() == "x",
                n: temp.next().unwrap().parse().unwrap(),
            }
        })
        .collect();
    let mut temp = line.split(" ").last().unwrap().split("=");
    folds.insert(
        0,
        Fold {
            x: temp.next().unwrap() == "x",
            n: temp.next().unwrap().parse().unwrap(),
        },
    );
    Day13Attempt2 { points, folds }
}
