use std::{cmp, ops::Add};

struct State {
    vents: Vec<Line>,
    diagram: Vec<Vec<u8>>,
}

impl State {
    fn diagram_over_one(&self) -> usize {
        let mut total_over_two = 0;
        self.diagram.iter().for_each(|v| {
            v.iter().for_each(|x| {
                if *x > 1u8 {
                    total_over_two += 1
                }
            })
        });
        total_over_two
    }
    fn fill_diagram_straight(&mut self) {
        for line in self.vents.iter().filter(|line| line.is_straight()) {
            for p in line.range_straight() {
                self.diagram[p[0]][p[1]] += 1;
            }
        }
    }
    fn fill_diagram(&mut self) {
        for line in &self.vents {
            for p in line.range() {
                self.diagram[p[0]][p[1]] += 1;
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
    p1: [usize; 2],
    p2: [usize; 2],
}

impl Line {
    fn is_straight(&self) -> bool {
        self.p1[0] == self.p2[0] || self.p1[1] == self.p2[1]
    }

    fn range_straight(&self) -> Vec<[usize; 2]> {
        if self.p1[0] == self.p2[0] {
            let r = if self.p1[1] < self.p2[1] {
                self.p1[1]..self.p2[1] + 1
            } else {
                self.p2[1]..self.p1[1] + 1
            };
            r.map(|p| [self.p1[0], p]).collect()
        } else {
            let r = if self.p1[0] < self.p2[0] {
                self.p1[0]..self.p2[0] + 1
            } else {
                self.p2[0]..self.p1[0] + 1
            };
            r.map(|p| [p, self.p1[1]]).collect()
        }
    }
    fn range(&self) -> Vec<[usize; 2]> {
        if self.is_straight() {
            return self.range_straight();
        }
        let x_dir: i32 = if self.p1[0] < self.p2[0] { 1 } else { -1 };
        let y_dir: i32 = if self.p1[1] < self.p2[1] { 1 } else { -1 };
        let mut x: i32 = self.p1[0] as i32;
        let mut y: i32 = self.p1[1] as i32;
        let mut range: Vec<[usize; 2]> = vec![];
        while x != self.p2[0] as i32 {
            range.push([x as usize, y as usize]);
            x += x_dir;
            y += y_dir;
        }
        range.push([self.p2[0], self.p2[1]]);
        range
    }
    #[allow(dead_code)]
    fn to_string(&self) -> String {
        let mut total: String = String::new();
        total.push_str("Line{\n");
        total.push_str(&format!("\tis_straight:{},\n", self.is_straight()));
        total.push_str(&format!("\tp1{{ x:{}, y:{}}},", self.p1[0], self.p1[1]));
        total.push_str(&format!("p2{{ x:{}, y:{}}},\n", self.p2[0], self.p2[1]));
        total.push_str(&format!("RANGE:{:?},\n", self.range_straight()));
        total.push_str("}");
        total
    }
}

pub fn part1(lines: Vec<String>) -> String {
    let mut state: State = import(lines);
    state.fill_diagram_straight();
    state.diagram_over_one().to_string()
}

pub fn part2(lines: Vec<String>) -> String {
    let mut state: State = import(lines);
    state.fill_diagram();
    state.diagram_over_one().to_string()
}

fn import(lines: Vec<String>) -> State {
    let vents: Vec<Line> = lines
        .iter()
        .map(|s| {
            import_line(
                s.split("->")
                    .map(|text_point| import_point(text_point))
                    .collect(),
            )
        })
        .collect();
    let n = vents.clone().into_iter().fold(0, |max: usize, vent: Line| {
        cmp::max(
            max,
            cmp::max(
                vent.p2[1],
                cmp::max(vent.p2[0], cmp::max(vent.p1[0], vent.p1[1])),
            ),
        )
    }) as usize;
    return State {
        vents: vents,
        diagram: vec![vec![0; n + 1]; n + 1],
    };
}

fn import_line(points: Vec<[usize; 2]>) -> Line {
    Line {
        p1: points[0],
        p2: points[1],
    }
}

fn import_point(text_point: &str) -> [usize; 2] {
    let mut int_list = text_point.trim().split(",");
    [
        int_list.next().unwrap().parse().unwrap(),
        int_list.next().unwrap().parse().unwrap(),
    ]
}
