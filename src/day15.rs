use std::{cmp::Ordering, collections::BinaryHeap};

struct Day15 {
    verticies: Vec<Vertex>,
}

struct Vertex {
    weight: u32,
    best_value: u32,
    arcs: Vec<u32>,
}

#[derive(Clone, Eq, PartialEq)]
struct SmallVertex {
    id: u32,
    best_value: u32,
}

impl Ord for SmallVertex {
    fn cmp(&self, other: &Self) -> Ordering {
        other.best_value.cmp(&self.best_value)
    }
}

impl PartialOrd for SmallVertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Day15 {
    fn find_path(&mut self) -> u32 {
        let mut to_visit: BinaryHeap<SmallVertex> = BinaryHeap::new();
        let start: usize = 0;
        let fin: usize = self.verticies.len() - 1;
        to_visit.push(SmallVertex {
            id: start as u32,
            best_value: 0,
        });
        while let Some(visitor) = to_visit.pop() {
            if visitor.best_value > self.verticies[visitor.id as usize].best_value {
                continue; //duplicate, move on
            }
            let arcs_len = self.verticies[visitor.id as usize].arcs.len();
            for ai in 0..arcs_len {
                let visitor_arc_id = self.verticies[visitor.id as usize].arcs[ai];
                let mut arc = SmallVertex {
                    id: visitor_arc_id,
                    best_value: self.verticies[visitor_arc_id as usize].best_value,
                };
                if arc.best_value
                    <= self.verticies[visitor.id as usize].best_value
                        + self.verticies[visitor.id as usize].weight
                {
                    continue;
                }
                arc.best_value = visitor.best_value + self.verticies[visitor.id as usize].weight;
                self.verticies[visitor_arc_id as usize].best_value =
                    visitor.best_value + self.verticies[visitor.id as usize].weight;
                if arc.id as usize != fin {
                    to_visit.push(arc);
                }
            }
        }

        self.verticies[fin].best_value - self.verticies[start].weight + self.verticies[fin].weight
    }

    fn new(lines: Vec<String>) -> Self {
        let twodboard: Vec<Vec<Vertex>> = Day15::extract_thicc(lines);
        let n = twodboard.len();
        let mut verticies: Vec<Vertex> = twodboard.into_iter().flatten().collect();
        verticies.iter_mut().for_each(|v| v.weight += 1);
        for x in 0..n {
            for y in 0..n {
                // verticies[x * n + y].id = (x * n + y) as u32;
                if x > 0 {
                    verticies[x * n + y].arcs.push(((x - 1) * n + y) as u32)
                }
                if x < n - 1 {
                    verticies[x * n + y].arcs.push(((x + 1) * n + y) as u32)
                }
                if y > 0 {
                    verticies[x * n + y].arcs.push((x * n + y - 1) as u32)
                }
                if y < n - 1 {
                    verticies[x * n + y].arcs.push((x * n + y + 1) as u32)
                }
            }
        }
        Day15 { verticies }
    }

    fn extract_thicc(lines: Vec<String>) -> Vec<Vec<Vertex>> {
        lines
            .into_iter()
            .map(|s| {
                s.trim()
                    .chars()
                    .map(|c| Vertex {
                        weight: (c.to_digit(10).unwrap() - 1) as u32,
                        best_value: u32::MAX,
                        arcs: vec![],
                        // id: 0,
                    })
                    .collect()
            })
            .collect()
    }

    fn new_pt2(lines: Vec<String>) -> Self {
        let twodboard: Vec<Vec<Vertex>> = Day15::extract_thicc(lines);
        let n = twodboard.len() as u32;
        let verticies: Vec<Vertex> = twodboard.into_iter().flatten().collect();
        let mut verticies_2: Vec<Vertex> = Vec::with_capacity(25 * (n * n) as usize); //(5n)^2
        for x in 0..5 * n {
            for y in 0..5 * n {
                let i = x * 5 * n + y;
                let w =
                    ((verticies[(((x % n) * n) + (y % n)) as usize].weight + (x / n) + (y / n))
                        % 9)
                        + 1;

                verticies_2.push(Vertex {
                    // id: i,
                    weight: w,
                    best_value: u32::MAX,
                    arcs: if x == 0 && y == 0 {
                        vec![5 * n, 1]
                    } else {
                        vec![i - (5 * n), i + (5 * n), i - 1, i + 1]
                    },
                });
                // verticies_2[i as usize].id = i;
                if y == 5 * n - 1 {
                    verticies_2[i as usize].arcs.remove(3);
                }
                if y == 0 {
                    if x != 0 {
                        verticies_2[i as usize].arcs.remove(2);
                    }
                }
                if x == 5 * n - 1 {
                    verticies_2[i as usize].arcs.remove(1);
                }
                if x == 0 {
                    if y != 0 {
                        verticies_2[i as usize].arcs.remove(0);
                    }
                }
            }
        }
        Day15 {
            verticies: verticies_2,
        }
    }
}

pub fn part1(lines: Vec<String>) -> String {
    let mut data = Day15::new(lines);
    format!("p1: {}", data.find_path())
}

pub fn part2(lines: Vec<String>) -> String {
    let mut data2 = Day15::new_pt2(lines);
    format!("p2: {}", data2.find_path())
}
