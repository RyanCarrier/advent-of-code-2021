use std::ops::Add;

static MININDEX: usize = 0;
static MAXINDEX: usize = 9;

struct Jun {
    octy: Vec<Vec<usize>>,
    ryans: Vec<[usize; 2]>, //get it they are 10s
    zeros: Vec<[usize; 2]>,
    n: usize,
}

impl Jun {
    fn find_all_flash(&mut self) -> usize {
        while self.step() < 100 {}
        self.n
    }

    fn steps(&mut self, n: usize) -> usize {
        let mut total = 0;
        for _ in 0..n {
            total += self.step()
        }
        total
    }

    fn step(&mut self) -> usize {
        self.octy
            .iter_mut()
            .for_each(|v| v.iter_mut().for_each(|power| *power += 1));
        for x in MININDEX..MAXINDEX + 1 {
            for y in MININDEX..MAXINDEX + 1 {
                if self.octy[x][y] > 9 {
                    self.ryans.push([x, y]);
                }
            }
        }
        let mut james = 0;
        while let Some(ryan) = self.ryans.pop() {
            self.increase_around(ryan);
            self.zeros.push(ryan);
            james += 1;
        }
        self.zeros.iter().for_each(|z| self.octy[z[0]][z[1]] = 0);
        self.zeros.clear();
        self.n += 1;
        return james;
    }

    fn increase_around(&mut self, point: [usize; 2]) {
        for dx in -1..=1 {
            for dy in -1..=1 {
                self.increase(point, [dx, dy]);
            }
        }
    }
    fn increase(&mut self, point: [usize; 2], transform: [i32; 2]) {
        if transform == [0, 0] {
            return;
        }
        let x: i32 = point[0] as i32 + transform[0];
        let y: i32 = point[1] as i32 + transform[1];
        if x <= MAXINDEX as i32
            && x >= MININDEX as i32
            && y <= MAXINDEX as i32
            && y >= MININDEX as i32
        {
            self.octy[x as usize][y as usize] += 1;
            if self.octy[x as usize][y as usize] == 10 {
                self.ryans.push([x as usize, y as usize])
            }
        }
    }

    #[allow(dead_code)]
    fn to_string(&self) -> String {
        self.octy.iter().fold(String::new(), |f, line| {
            f.add("\t")
                .add(
                    &line
                        .iter()
                        .fold(String::new(), |g, n| g.add(&n.to_string())),
                )
                .add("\n")
        })
    }
}

pub fn part1(lines: Vec<String>) -> String {
    let mut marisa = import(lines);
    (marisa.steps(100)).to_string()
}

pub fn part2(lines: Vec<String>) -> String {
    (import(lines).find_all_flash()).to_string()
}

fn import(lines: Vec<String>) -> Jun {
    Jun {
        octy: lines
            .iter()
            .map(|x| x.chars().map(|c| c.to_string().parse().unwrap()).collect())
            .collect(),
        ryans: vec![],
        zeros: vec![],
        n: 0,
    }
}
