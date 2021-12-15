use std::{cmp::Ordering, collections::BinaryHeap, ops::RangeBounds};

struct Day15 {
    verticies: Vec<Vertex>,
}

#[derive(Clone, Eq, PartialEq)]
struct Vertex {
    weight: usize,
    best: usize,
    bestv: usize,
    arcs: Vec<usize>,
    i: usize,
}
impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.best.cmp(&self.best)
        // .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Day15 {
    fn find_path(&mut self) -> usize {
        let mut to_visit: BinaryHeap<&Vertex> = BinaryHeap::new();
        let start: usize = 0;
        let fin: usize = self.verticies.len();
        to_visit.push(&self.verticies[start]);
        while let Some(v) = to_visit.pop() {
            for a in &v.arcs {
                if self.verticies[*a].best > v.best + v.weight {
                    self.verticies[*a].best = v.best + v.weight;
                    self.verticies[*a].bestv = v.i;
                }
            }
        }
        0
    }

    fn new(lines: Vec<String>) -> Self {
        let n = lines.len();
        let twodboard: Vec<Vec<Vertex>> = lines
            .into_iter()
            .map(|s| {
                s.trim()
                    .split(" ")
                    .map(|ss| Vertex {
                        weight: ss.parse().unwrap(),
                        best: 0,
                        bestv: 0,
                        arcs: vec![],
                        i: 0,
                    })
                    .collect()
            })
            .collect();
        let mut verticies: Vec<Vertex> = twodboard.into_iter().flatten().collect();
        for x in 0..n {
            for y in 0..n {
                verticies[x * n + y].i = x * n + y;
                if x > 0 {
                    verticies[x * n + y].arcs.push((x - 1) * n + y)
                }
                if x < n - 1 {
                    verticies[x * n + y].arcs.push((x + 1) * n + y)
                }
                if y > 0 {
                    verticies[x * n + y].arcs.push(x * n + y - 1)
                }
                if y < n - 1 {
                    verticies[x * n + y].arcs.push(x * n + y + 1)
                }
            }
        }

        Day15 {
            verticies: twodboard.into_iter().flatten().collect(),
        }
    }
}
pub fn part1(lines: Vec<String>) -> String {
    let data = Day15::new(lines);
    let ret = format!("p1: {}, p2: ", "");
    ret
}
pub fn part2(_: Vec<String>) -> String {
    String::from("NA - part 1")
}
