use std::{cmp, ops::Add};

struct State {
    vents: Vec<Line>,
    diagram: Vec<Vec<u8>>,
}

impl State {
    fn fill_diagram_straight(&mut self) {
        for line in self.vents.iter().filter(|line| line.is_straight()) {
            for p in line.range_straight() {
                self.diagram[p.x as usize][p.y as usize] += 1;
            }
        }
    }
    fn fill_diagram(&mut self) {
        for line in self.vents.clone() {
            for p in line.range() {
                self.diagram[p.x as usize][p.y as usize] += 1;
            }
        }
    }
    #[allow(dead_code)]
    fn to_string(&self) -> String {
        let mut total: String = String::new();
        total.push_str("State{");
        total.push_str("vents:{\n");
        for line in self.vents.clone() {
            total.push_str(&format!("{}\n", line.to_string()));
        }
        total.push_str("},");
        total.push_str("diagram:{\n");
        for line in self.diagram.clone() {
            total.push_str(&line.into_iter().fold(String::new(), |s, x| {
                if x == 0 {
                    s.add(".")
                } else {
                    s.add(&x.to_string())
                }
            }));
            total.push_str("\n");
        }
        total.push_str("}");
        total
    }
}
#[derive(Debug, Clone, Copy)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn is_straight(&self) -> bool {
        self.p1.x == self.p2.x || self.p1.y == self.p2.y
    }

    fn range_straight(&self) -> Vec<Point> {
        if self.p1.x == self.p2.x {
            let r = if self.p1.y < self.p2.y {
                self.p1.y..self.p2.y + 1
            } else {
                self.p2.y..self.p1.y + 1
            };
            r.map(|p| Point { x: self.p1.x, y: p }).collect()
        } else {
            let r = if self.p1.x < self.p2.x {
                self.p1.x..self.p2.x + 1
            } else {
                self.p2.x..self.p1.x + 1
            };
            r.map(|p| Point { x: p, y: self.p1.y }).collect()
        }
    }
    fn range(&self) -> Vec<Point> {
        if self.is_straight() {
            return self.range_straight();
        }
        let x_dir: i32 = if self.p1.x < self.p2.x { 1 } else { -1 };
        let y_dir: i32 = if self.p1.y < self.p2.y { 1 } else { -1 };
        let mut x: i32 = self.p1.x as i32;
        let mut y: i32 = self.p1.y as i32;
        let mut range: Vec<Point> = vec![];
        while x != self.p2.x as i32 {
            range.push(Point {
                x: x as u16,
                y: y as u16,
            });
            x += x_dir;
            y += y_dir;
        }
        range.push(Point {
            x: self.p2.x,
            y: self.p2.y,
        });
        range
    }
    #[allow(dead_code)]
    fn to_string(&self) -> String {
        let mut total: String = String::new();
        total.push_str("Line{\n");
        total.push_str(&format!("\tis_straight:{},\n", self.is_straight()));
        total.push_str(&format!("\tp1{{ x:{}, y:{}}},", self.p1.x, self.p1.y));
        total.push_str(&format!("p2{{ x:{}, y:{}}},\n", self.p2.x, self.p2.y));
        total.push_str(&format!("RANGE:{:?},\n", self.range_straight()));
        total.push_str("}");
        total
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: u16,
    y: u16,
}

pub fn part1(lines: Vec<String>) -> String {
    let mut state: State = import(lines);
    state.fill_diagram_straight();
    let total_over_two: usize = state.diagram.iter().fold(0, |total, x| {
        total
            + x.iter()
                .fold(0, |total2, y| if *y > 1_u8 { total2 + 1 } else { total2 })
    });
    total_over_two.to_string()
}

pub fn part2(lines: Vec<String>) -> String {
    let mut state: State = import(lines);
    state.fill_diagram();
    let total_over_two: usize = state.diagram.iter().fold(0, |total, x| {
        total
            + x.iter()
                .fold(0, |total2, y| if *y > 1_u8 { total2 + 1 } else { total2 })
    });
    total_over_two.to_string()
}

fn import(lines: Vec<String>) -> State {
    let vents: Vec<Line> = lines
        .iter()
        .map(|s| {
            import_line(
                s.split("->")
                    .map(|text_point| import_point(String::from(text_point)))
                    .collect(),
            )
        })
        .collect();
    let n = vents.clone().into_iter().fold(0, |max: u16, vent: Line| {
        cmp::max(
            max,
            cmp::max(
                vent.p2.y,
                cmp::max(vent.p2.x, cmp::max(vent.p1.x, vent.p1.y)),
            ),
        )
    }) as usize;
    return State {
        vents: vents,
        diagram: vec![vec![0; n + 1]; n + 1],
    };
}

fn import_line(points: Vec<Point>) -> Line {
    Line {
        p1: points[0],
        p2: points[1],
    }
}

fn import_point(text_point: String) -> Point {
    let int_list: Vec<u16> = text_point
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();
    Point {
        x: int_list[0],
        y: int_list[1],
    }
}
