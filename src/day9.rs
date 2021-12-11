use std::cmp;

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
    fn adjust(&self, current: [usize; 2], max: [usize; 2]) -> [usize; 2] {
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
}

impl Heightmap {
    fn get_low(&self) -> Vec<[usize; 2]> {
        let mut marisa = vec![];
        for x in 0..self.heights.len() {
            for y in 0..self.heights[0].len() {
                if self.check_low([x, y]) {
                    marisa.push([x, y]);
                }
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

    fn check_low(&self, point: [usize; 2]) -> bool {
        let max = [self.heights.len() - 1, self.heights[0].len() - 1];
        let james = self.heights[point[0]][point[1]];
        DIRECTIONS.iter().fold(true, |b, d| {
            b && (d.adjust(point, max) == point || james < self.get(d.adjust(point, max)))
        })
    }

    fn get(&self, point: [usize; 2]) -> usize {
        match self.heights.get(point[0]) {
            Some(v) => match v.get(point[1]) {
                Some(v2) => *v2,
                None => 10,
            },
            None => 10,
        }
    }

    fn risk(&self) -> usize {
        self.get_low_values().iter().fold(0, |jun, x| jun + x + 1)
    }

    fn find_basins_product(&self) -> usize {
        let max = [self.heights.len() - 1, self.heights[0].len() - 1];
        let low_points = self.get_low();
        let mut basins: Vec<usize> = vec![];
        for l in low_points {
            let mut temp_map = vec![vec![0; self.heights[0].len()]; self.heights.len()];
            let mut to_visit: Vec<[usize; 2]> = vec![l];
            while let Some(point) = to_visit.pop() {
                let point_val = self.get(point);
                temp_map[point[0]][point[1]] = 1;
                for dir in &DIRECTIONS {
                    let point_direction = dir.adjust(point, max);
                    if temp_map[point_direction[0]][point_direction[1]] == 0 {
                        let point_direction_val = self.get(point_direction);
                        if point_direction_val == 9 {
                            temp_map[point_direction[0]][point_direction[1]] = 2;
                            continue;
                        }
                        if point_direction_val > point_val {
                            temp_map[point_direction[0]][point_direction[1]] = 1;
                            to_visit.push(point_direction);
                        }
                    }
                }
            }
            basins.push(temp_map.iter().fold(0, |basin, x| {
                basin + x.iter().fold(0, |basin2, y| basin2 + *y % 2)
            }));
        }
        basins.sort_by(|a, b| b.cmp(a));
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
    Heightmap {
        heights: lines
            .iter()
            .map(|x| {
                x.trim()
                    .chars()
                    .map(|c| c.to_string().parse::<usize>().unwrap())
                    .collect()
            })
            .collect(),
    }
}
