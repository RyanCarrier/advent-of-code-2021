use core::panic;

#[derive(Debug, Clone)]
struct State {
    draws: Vec<u64>,
    n: usize,
    boards: Vec<Vec<Vec<u64>>>,
}

impl State {
    fn get_sum_unmarked(&self, board_win: usize, current_drawn: usize) -> u64 {
        let flat: Vec<u64> = self
            .boards
            .get(board_win)
            .unwrap()
            .into_iter()
            .flat_map(|x| x.into_iter())
            .cloned()
            .collect();
        let draws = self.draws.get(0..current_drawn).unwrap();
        let sum_total: u64 = flat.iter().filter(|x| !draws.contains(x)).sum();
        sum_total
    }
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
            // println!("xwin, board#{}, x:{}", board, x);
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
            // println!("ywin, board#{}, y:{}", board, y);
            return true;
        }
        return false;
    }
}

pub fn part1(lines: Vec<String>) {
    let state = import(lines);

    let mut board_win: usize = 0;
    let mut current_drawn: usize = 9999;
    'drawn: for drawn in 0..state.draws.len() {
        current_drawn = drawn;
        for board in 0..state.boards.len() {
            if state.check_board(drawn, board) {
                board_win = board;
                break 'drawn;
            }
        }
    }
    let sum_total = state.get_sum_unmarked(board_win, current_drawn);
    let last_drawn = state.draws.get(current_drawn - 1).unwrap();
    //sum of unused multiplied by last drawn
    println!(
        "day4part1: Board#{}: {}, sum_total:{}, last_draw:{}",
        board_win,
        sum_total * last_drawn,
        sum_total,
        last_drawn
    )
}

pub fn part2(lines: Vec<String>) {
    let state = import(lines);

    let mut board_win: usize = 0;
    let mut current_drawn: usize = 9999;
    let mut won: Vec<bool> = vec![false; state.boards.clone().len()];
    'drawn: for drawn in 0..state.draws.len() {
        current_drawn = drawn;
        for board in 0..state.boards.len() {
            if won[board] {
                continue;
            }
            if state.check_board(drawn, board) {
                board_win = board;
                won[board] = true;
                if won.iter().all(|x| *x) {
                    break 'drawn;
                }
            }
        }
    }
    let sum_total = state.get_sum_unmarked(board_win, current_drawn);
    let last_drawn = state.draws.get(current_drawn - 1).unwrap();
    //sum of unused multiplied by last drawn
    println!(
        "day4part2: Board#{}: {}, sum_total:{}, last_draw:{}",
        board_win,
        sum_total * last_drawn,
        sum_total,
        last_drawn
    )
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
