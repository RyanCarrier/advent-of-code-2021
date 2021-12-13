struct Day13 {
    folds: Vec<Fold>,
    grid: Vec<Vec<bool>>,
}
#[derive(Clone, Copy)]
struct Fold {
    x: bool,
    n: usize,
}

impl Day13 {
    fn fold_first(&mut self) -> usize {
        self.fold(self.folds[0]);
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
                    if x < fold.n && self.grid[x][y] {
                        new_grid[x][y] = true;
                    }
                    if x > fold.n && self.grid[x][y] {
                        new_grid[fold.n * 2 - x][y] = true;
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
                    if y > fold.n && self.grid[x][y] {
                        new_grid[x][fold.n * 2 - y] = true;
                    }
                    if y < fold.n && self.grid[x][y] {
                        new_grid[x][y] = true;
                    }
                }
            }
        }
        self.grid = new_grid;
    }

    #[allow(dead_code)]
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

pub fn part1(lines: Vec<String>) -> String {
    let mut data = import(lines);
    return data.fold_first().to_string();
}

pub fn part2(lines: Vec<String>) -> String {
    let mut data = import(lines);
    data.fold_all();
    return data.to_string_transformed();
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
