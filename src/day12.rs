#[derive(Clone)]
struct Path {
    path: Vec<usize>,
    small_twice: bool,
}
impl Path {
    fn contains(&self, node: &usize) -> bool {
        self.path.contains(node)
    }
}

struct Vertex {
    is_small: bool,
    arcs: Vec<usize>,
}
struct Graph {
    verticies: Vec<Vertex>,
}

impl Graph {
    fn find_paths(&self, allow_small_twice: bool) -> usize {
        let node = &self.verticies[0];
        let current_path = Path {
            path: vec![0],
            small_twice: !allow_small_twice,
        };
        self.path_continue(node, &current_path)
    }

    fn path_continue(&self, node: &Vertex, path: &Path) -> usize {
        let mut all_paths = 0;
        for arc in &node.arcs {
            if *arc == 1 {
                all_paths += 1;
                continue;
            }
            if self.verticies[*arc].is_small {
                if path.contains(&arc) && path.small_twice {
                    continue;
                }
                let mut arcpath = path.clone();
                if arcpath.contains(&arc) {
                    arcpath.small_twice = true;
                }
                arcpath.path.push(*arc);
                all_paths += self.path_continue(&self.verticies[*arc], &arcpath);
            } else {
                all_paths += self.path_continue(&self.verticies[*arc], &path);
            }
        }
        all_paths
    }
    #[allow(dead_code)]
    fn print(&self) {
        self.verticies
            .iter()
            .enumerate()
            .for_each(|(s, v)| v.arcs.iter().for_each(|a| println!("{}->{}", s, a)))
    }
}

pub fn part1(lines: Vec<String>) -> String {
    let data = import(lines);
    return data.find_paths(false).to_string();
}

pub fn part2(lines: Vec<String>) -> String {
    let data = import(lines);
    //123086
    return data.find_paths(true).to_string();
}

fn import(lines: Vec<String>) -> Graph {
    let mut graph = Graph {
        verticies: vec![
            Vertex {
                is_small: true,
                arcs: vec![],
            },
            Vertex {
                is_small: true,
                arcs: vec![],
            },
        ],
    };
    let mut map: Vec<&str> = vec!["start", "end"];
    lines.iter().for_each(|line| {
        let mut temp = line.trim().split("-");
        let v1 = temp.next().unwrap().trim();
        let v1n = if map.contains(&v1) {
            map.iter().position(|x| *x == v1).unwrap()
        } else {
            graph.verticies.push(Vertex {
                arcs: vec![],
                is_small: &v1 != &v1.to_uppercase(),
            });
            map.push(v1);
            map.len() - 1
        };
        let v2 = temp.next().unwrap().trim();
        let v2n = if map.contains(&v2) {
            map.iter().position(|x| *x == v2).unwrap()
        } else {
            graph.verticies.push(Vertex {
                arcs: vec![],
                is_small: &v2 != &v2.to_uppercase(),
            });
            map.push(v2);
            map.len() - 1
        };
        if v2n != 0 {
            graph.verticies[v1n].arcs.push(v2n);
        }
        if v1n != 0 {
            graph.verticies[v2n].arcs.push(v1n);
        }
    });
    graph
}
