#[derive(Clone)]
struct Path {
    path: Vec<bool>,
    small_twice: bool,
}

struct Graph {
    arc_list: Vec<Vec<u8>>,
    small: Vec<bool>,
}

impl Graph {
    fn find_paths(&self, allow_small_twice: bool) -> u32 {
        let node = &self.arc_list[0];
        let mut current_path = Path {
            path: vec![false; self.arc_list.len()],
            small_twice: !allow_small_twice,
        };
        current_path.path[0] = true;
        let mut total = 0;
        self.path_continue(node, &current_path, &mut total);
        total
    }

    fn path_continue(&self, node_arcs: &Vec<u8>, path: &Path, total: &mut u32) {
        for arc in node_arcs {
            if *arc == 1 {
                *total += 1;
                continue;
            }
            if self.small[*arc as usize] {
                if path.small_twice && path.path[*arc as usize] {
                    continue;
                }
                let mut arcpath = path.clone();
                if path.path[*arc as usize] {
                    arcpath.small_twice = true;
                } else {
                    arcpath.path[*arc as usize] = true
                }
                self.path_continue(&self.arc_list[*arc as usize], &arcpath, total);
            } else {
                self.path_continue(&self.arc_list[*arc as usize], &path, total);
            }
        }
    }
}

pub fn part1(lines: Vec<String>) -> String {
    let data = import(lines);
    return data.find_paths(false).to_string();
}

pub fn part2(lines: Vec<String>) -> String {
    let data = import(lines);
    return data.find_paths(true).to_string();
}

fn import(lines: Vec<String>) -> Graph {
    let mut graph = Graph {
        arc_list: vec![vec![], vec![]],
        small: vec![true, true],
    };
    let mut map: Vec<&str> = vec!["start", "end"];
    lines.iter().for_each(|line| {
        let mut temp = line.trim().split("-");
        let v1 = temp.next().unwrap().trim();
        let v1n = if map.contains(&v1) {
            map.iter().position(|x| *x == v1).unwrap()
        } else {
            graph.arc_list.push(vec![]);
            graph.small.push(&v1 != &v1.to_uppercase());
            map.push(v1);
            map.len() - 1
        };
        let v2 = temp.next().unwrap().trim();
        let v2n = if map.contains(&v2) {
            map.iter().position(|x| *x == v2).unwrap()
        } else {
            graph.arc_list.push(vec![]);
            graph.small.push(&v2 != &v2.to_uppercase());
            map.push(v2);
            map.len() - 1
        };
        if v2n != 0 {
            graph.arc_list[v1n].push(v2n as u8);
        }
        if v1n != 0 {
            graph.arc_list[v2n].push(v1n as u8);
        }
    });
    graph
}
