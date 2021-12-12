enum Direction {
    Left,
    Up,
    Right,
    Down,
}
static DIRECTIONS: [Direction; 4] = [
    Direction::Left,
    Direction::Up,
    Direction::Right,
    Direction::Down,
];

impl Direction {
    fn adjust(&self, current: [usize; 2], max: &[usize; 2]) -> [usize; 2] {
        match self {
            Direction::Left => [if current[0] == 0 { 0 } else { current[0] - 1 }, current[1]],
            Direction::Up => [current[0], (current[1] + 1).min(max[1])],
            Direction::Right => [(current[0] + 1).min(max[0]), current[1]],
            Direction::Down => [current[0], if current[1] == 0 { 0 } else { current[1] - 1 }],
        }
    }
}

struct Heightmap {
    heights: Vec<Vec<usize>>,
    max: [usize; 2],
}

impl Heightmap {
    fn get_adjust(&self, p: [usize; 2], d: Direction) -> usize {
        match d {
            Direction::Left => {
                if p[0] == 0 {
                    return 9;
                } else {
                    return self.heights[p[0] - 1][p[1]];
                }
            }
            Direction::Up => {
                if p[1] == self.max[1] {
                    return 9;
                } else {
                    return self.heights[p[0]][p[1] + 1];
                }
            }

            Direction::Right => {
                if p[0] == self.max[0] {
                    return 9;
                } else {
                    return self.heights[p[0] + 1][p[1]];
                }
            }
            Direction::Down => {
                if p[1] == 0 {
                    return 9;
                } else {
                    return self.heights[p[0]][p[1] - 1];
                }
            }
        }
    }
    fn get_low(&self) -> Vec<[usize; 2]> {
        let mut marisa = vec![];
        for x in 0..=self.max[0] {
            for y in 0..=self.max[1] {
                if self.heights[x][y] == 9 {
                    continue;
                }
                let pv = self.heights[x][y];
                if self.get_adjust([x, y], Direction::Left) <= pv
                    || self.get_adjust([x, y], Direction::Up) <= pv
                    || self.get_adjust([x, y], Direction::Right) <= pv
                    || self.get_adjust([x, y], Direction::Down) <= pv
                {
                    continue;
                }
                marisa.push([x, y]);
            }
        }
        marisa
    }

    fn get_low_values(&self) -> Vec<usize> {
        self.get_low()
            .iter()
            .map(|m| self.heights[m[0]][m[1]])
            .collect()
    }

    fn risk(&self) -> usize {
        self.get_low_values().iter().fold(0, |jun, x| jun + x + 1)
    }

    fn find_basins_product(&self) -> usize {
        let mut basins: Vec<usize> = vec![];
        let mut temp_map = vec![vec![false; self.heights[0].len()]; self.heights.len()];
        let mut to_visit: Vec<[usize; 2]> = vec![];
        for l in self.get_low() {
            let mut temp_total = 1;
            to_visit.push(l);
            while let Some(point) = to_visit.pop() {
                let point_val = self.heights[point[0]][point[1]];
                temp_map[point[0]][point[1]] = true;
                // for t in self.get_valid_direction(point, max) {
                for d in &DIRECTIONS {
                    let t = d.adjust(point, &self.max);
                    if temp_map[t[0]][t[1]] || t == point {
                        continue;
                    }
                    let point_direction_val = self.heights[t[0]][t[1]]; //self.get(t);
                    if point_direction_val == 9 {
                        temp_map[t[0]][t[1]] = true;
                        continue;
                    }
                    if point_direction_val > point_val {
                        temp_map[t[0]][t[1]] = true;
                        temp_total += 1;
                        to_visit.push(t);
                    }
                }
            }
            basins.push(temp_total);
        }
        basins.sort_by(|a, b| b.cmp(a));
        // println!("{:?}", basins.get(0..3));
        return basins[0] * basins[1] * basins[2]; //top3.iter().product();
    }
}

pub fn part1(lines: Vec<String>) -> String {
    let data = import(lines);
    data.risk().to_string()
}

pub fn part2(lines: Vec<String>) -> String {
    let hmap = import(lines);
    hmap.find_basins_product().to_string()
}

fn import(lines: Vec<String>) -> Heightmap {
    let mut hm = Heightmap {
        heights: lines
            .iter()
            .map(|x| {
                x.trim()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect(),
        max: [0, 0],
    };
    hm.max = [hm.heights.len() - 1, hm.heights[0].len() - 1];
    hm
}
