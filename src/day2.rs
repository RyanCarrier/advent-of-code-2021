enum Direction {
    None,
    Forward,
    Up,
    Down,
}
impl Direction {
    fn from(input: &str) -> Direction {
        match input {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => Direction::None,
        }
    }
}
struct Step {
    direction: Direction,
    size: i64,
}

#[derive(Debug)]
struct State {
    horizontal: i64,
    depth: i64,
    aim: i64,
}

impl Step {
    fn from(input: String) -> Step {
        let split: Vec<&str> = input.split(" ").collect();
        Step {
            direction: Direction::from(split[0].trim()),
            size: split[1].parse().unwrap(),
        }
    }
}

pub fn part1(lines: Vec<String>) {
    let commands: Vec<Step> = lines.iter().map(|x| Step::from(x.to_string())).collect();
    let total = commands.iter().fold(
        State {
            horizontal: 0,
            depth: 0,
            aim: 0,
        },
        |mut state, x| {
            match x.direction {
                Direction::Down => state.depth += x.size,
                Direction::Up => state.depth -= x.size,
                Direction::Forward => state.horizontal += x.size,
                Direction::None => (),
            };
            state
        },
    );

    println!("day1part1:{:?},{}", total, total.depth * total.horizontal);
}

pub fn part2(lines: Vec<String>) {
    let commands: Vec<Step> = lines.iter().map(|x| Step::from(x.to_string())).collect();
    let total = commands.iter().fold(
        State {
            horizontal: 0,
            depth: 0,
            aim: 0,
        },
        |mut state, x| {
            match x.direction {
                Direction::Down => state.aim += x.size,
                Direction::Up => state.aim -= x.size,
                Direction::Forward => {
                    state.horizontal += x.size;
                    state.depth = state.depth + state.aim * x.size;
                }
                Direction::None => (),
            };
            // println!("{:?}", state);
            state
        },
    );

    println!("day1part2:{:?},{}", total, total.depth * total.horizontal);
}
