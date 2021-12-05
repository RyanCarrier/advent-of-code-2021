use core::panic;

#[derive(Debug, Clone)]
struct State {
    draws: Vec<u64>,
    n: usize,
    boards: Vec<Vec<Vec<u64>>>,
}

impl State {
    fn check_board(&self, draw_depth: usize, board: usize) -> bool {
        let draws = self.draws.get(0..draw_depth).unwrap();
        'x: for x in 0..self.n {
            for y in 0..self.n {
                let board_item = self
                    .boards
                    .get(board)
                    .unwrap()
                    .get(x)
                    .unwrap()
                    .get(y)
                    .unwrap();
                if !draws.contains(board_item) {
                    continue 'x;
                }
            }
            println!("xwin");
            return true;
        }
        'y: for y in 0..self.n {
            for x in 0..self.n {
                let board_item = self
                    .boards
                    .get(board)
                    .unwrap()
                    .get(x)
                    .unwrap()
                    .get(y)
                    .unwrap();
                if !draws.contains(board_item) {
                    continue 'y;
                }
            }
            println!("ywin");
            return true;
        }
        return false;
    }
}

pub fn part1(lines: Vec<String>) {
    let state = import(lines);
    println!("\tdraws:{:?}", state.draws);
    println!("\tn:{:?}", state.n);
    println!("\tboard:{:?}", state.boards);
    'drawn: for drawn in 0..state.draws.len() {
        for board in 0..state.boards.len() - 1 {
            if state.check_board(drawn, board) {
                println!(
                    "board #{:?} won, draws:{:?}",
                    board,
                    state.clone().draws.get(0..drawn).unwrap(),
                );
                break 'drawn;
            }
        }
    }
    println!("day4part1: ")
}

pub fn part2(lines: Vec<String>) {
    if lines.len() == 0 {
        println!("day4part2: ")
    }
}

fn import<'a>(lines: Vec<String>) -> State {
    let draws = import_draws(lines.get(0).unwrap());
    let temp: Vec<_> = lines
        .get(2)
        .unwrap()
        .trim()
        .split(" ")
        .filter(|x| x.len() > 0)
        .collect();
    let n: usize = temp.len();
    let mut i = 1;
    let mut boards: Vec<Vec<Vec<u64>>> = vec![];
    loop {
        match lines.get(i) {
            None => {
                return State {
                    draws,
                    n,
                    boards: boards,
                }
            }
            Some(x) => {
                if x.len() == 0 {
                    panic!("err len should not be 0");
                }
                let got_lines = lines.get(i..i + n).unwrap().to_vec();
                let temp = import_board(got_lines);
                boards.push(temp);
            }
        }
        i += n;
    }
}

fn import_draws(line: &String) -> Vec<u64> {
    line.split(',').map(|x| x.parse().unwrap()).collect()
}

fn import_board(lines: Vec<String>) -> Vec<Vec<u64>> {
    lines
        .into_iter()
        .map(|x| {
            x.trim()
                .split(" ")
                .filter(|y| y.len() > 0)
                .map(|y| y.parse().unwrap())
                .collect()
        })
        .collect()
}
