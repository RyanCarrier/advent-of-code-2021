struct State {
    days: u64,
    fishies: [u64; 9],
}

impl State {
    fn age(&mut self) {
        self.days += 1;
        self.fishies.rotate_left(1);
        self.fishies[6] += self.fishies[8];
    }

    fn school(&self) -> u64 {
        self.fishies.into_iter().sum()
    }
    fn to_string(&self) -> String {
        format!("After {} days:\t{:?}", self.days, self.fishies)
    }
}

pub fn part1(lines: Vec<String>) {
    let mut state = import_fish(lines);
    for _ in 0..80 {
        state.age();
    }
    println!("day6part1: {}", state.school())
}
pub fn part1_test(lines: Vec<String>) {
    let mut state = import_fish(lines);
    for _ in 0..18 {
        state.age();
    }
    println!("day6part1_test: {}", state.school())
}

pub fn part2(lines: Vec<String>) {
    if lines.len() == 0 {
        println!("day6part2: ")
    }
}

fn import_fish(lines: Vec<String>) -> State {
    let all_fish: Vec<u8> = lines[0].split(",").map(|x| x.parse().unwrap()).collect();

    let fishies: [u64; 9] = all_fish
        .iter()
        .fold([0u64; 9], |mut fishies: [u64; 9], x: &u8| {
            fishies[*x as usize] += 1;
            fishies
        });
    return State {
        days: 0,
        fishies: fishies,
    };
}
