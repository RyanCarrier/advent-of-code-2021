use std::collections::HashMap;

struct Navline {
    line: String,
}

impl Navline {
    fn corrupt(&self) -> usize {
        let scores: HashMap<char, usize> =
            HashMap::from([('}', 1197), (']', 57), (')', 3), ('>', 25137)]);
        let convert: HashMap<char, char> =
            HashMap::from([('{', '}'), ('[', ']'), ('(', ')'), ('<', '>')]);
        let mut stack: Vec<char> = vec![];
        for c in self.line.chars() {
            if convert.contains_key(&c) {
                stack.push(*convert.get(&c).unwrap())
            } else if *stack.last().unwrap() == c {
                stack.pop();
                continue;
            } else {
                return *scores.get(&c).unwrap();
            }
        }
        return 0;
    }
    fn autocomplete(&self) -> usize {
        let scores: HashMap<char, usize> = HashMap::from([('}', 3), (']', 2), (')', 1), ('>', 4)]);
        let convert: HashMap<char, char> =
            HashMap::from([('{', '}'), ('[', ']'), ('(', ')'), ('<', '>')]);
        let mut stack: Vec<char> = vec![];
        for c in self.line.chars() {
            if convert.contains_key(&c) {
                stack.push(*convert.get(&c).unwrap())
            } else if *stack.last().unwrap() == c {
                stack.pop();
                continue;
            }
        }
        let mut total = 0;
        while let Some(next) = stack.pop() {
            total *= 5;
            total += scores.get(&next).unwrap()
        }
        total
    }
}
pub fn part1(lines: Vec<String>) {
    let data = import(lines);

    println!(
        "day10part1: {}",
        data.iter().fold(0, |total, nl| total + nl.corrupt())
    )
}

pub fn part2(lines: Vec<String>) {
    let data = import(lines);
    let clean_data: Vec<Navline> = data.into_iter().filter(|x| x.corrupt() == 0).collect();
    let mut scores: Vec<usize> = clean_data.iter().map(|x| x.autocomplete()).collect();
    scores.sort();

    println!("day10part2: {}", scores[scores.len() / 2]);
}

fn import(lines: Vec<String>) -> Vec<Navline> {
    lines
        .iter()
        .map(|x| Navline {
            line: String::from(x),
        })
        .collect()
}
