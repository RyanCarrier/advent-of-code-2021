use std::ops::Add;

static SECTORS: [usize; 10] = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];

static UNIQUESECTORS: [usize; 4] = [2, 4, 3, 7];
static DEFAULTA: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
// static SUSSFREQUENCIES: [usize; 3] = [6, 4, 9];
//no need for suss frequencies when we figure them out on the fly with style
struct Digit {
    sectors: [Vec<char>; 7],
}

impl Digit {
    #[allow(dead_code)]
    fn print(&self) {
        println!("Digit state;");
        for i in 0..self.sectors.len() {
            println!("\tSector {}:{:?}", i, self.sectors[i]);
        }
    }

    fn get_sectors(&self, a: String) -> Vec<usize> {
        self.sectors
            .iter()
            .enumerate()
            .fold(vec![], |mut result, (i, char_list)| {
                if a.contains(*char_list.get(0).unwrap()) {
                    result.push(i);
                }
                result
            })
    }
}

pub fn part1(lines: Vec<String>) -> String {
    let source = import(lines);
    let total = source.iter().fold(0, |total, line| {
        total
            + line
                .get(1)
                .unwrap()
                .iter()
                .filter(|digit| UNIQUESECTORS.contains(&digit.trim().len()))
                .count()
    });

    total.to_string()
}

pub fn part2(lines: Vec<String>) -> String {
    let source = import(lines);
    let sectors_used: &[Vec<usize>; 10] = &[
        vec![0, 1, 2, 4, 5, 6],
        vec![2, 5],
        vec![0, 2, 3, 4, 6],
        vec![0, 2, 3, 5, 6],
        vec![1, 2, 3, 5],
        vec![0, 1, 3, 5, 6],
        vec![0, 1, 3, 4, 5, 6],
        vec![0, 2, 5],
        vec![0, 1, 2, 3, 4, 5, 6],
        vec![0, 1, 2, 3, 5, 6],
    ];

    let freq = sectors_used.clone().iter().fold([0; 10], |mut f, x| {
        x.iter().for_each(|j| f[*j] += 1);
        f
    });
    let mut sum_total: usize = 0;
    for line in &source {
        // let sectors = [0; 7].map(|_| DEFAULTA.clone().to_vec()); //lol gross
        let mut digit = Digit {
            sectors: [vec![], vec![], vec![], vec![], vec![], vec![], vec![]], //less gross??? Lol?
        };

        // println!("FREQ:{:?}", freq);
        // digit.print(); //i'm empty daddy

        //FREQUENCY FILTERING (or adding I guess lul)
        let concat = line.clone()[0].iter().fold(String::new(), |s, x| s.add(x));
        for a in DEFAULTA.clone() {
            let char_occurance = concat.chars().filter(|x| *x == a).count();
            freq.clone().iter().enumerate().for_each(|(i, f)| {
                if *f == char_occurance {
                    let mut temp = digit.sectors[i].clone();
                    temp.push(a);
                    digit.sectors[i] = temp;
                }
            });
        }

        // digit.print();
        //COLLECT DISTINCT DIGITS
        let uniques = line
            .get(0)
            .unwrap()
            .iter()
            .filter(|x| UNIQUESECTORS.contains(&x.len()));
        uniques.clone().for_each(|x| {
            let index = SECTORS.clone().iter().position(|y| *y == x.len()).unwrap();
            // println!("index {}", index);
            //FILTER DOWN ON SEGMENTS IT COULD BE
            //note this section does nothing on test case, but might be necessary in the big case? (all work done on the 'not' side)
            for sector in sectors_used.get(index).unwrap() {
                // println!("adjusting {}, keep {}", *sector, x);
                let temp = digit.sectors[*sector].clone();
                digit.sectors[*sector] = temp.into_iter().filter(|y| x.contains(*y)).collect();
            }
            // digit.print();
            //FILTER DOWN SEGMENTS IT COULD NOT BE
            for i in 0..digit.sectors.len() {
                if sectors_used.get(index).unwrap().contains(&i) {
                    continue;
                }
                let temp = digit.sectors[i].clone();
                digit.sectors[i] = temp.into_iter().filter(|j| !(x).contains(*j)).collect();
            }
        });
        let mut number = String::new();
        for word in line[1].clone() {
            let mut sectors = digit.get_sectors(word);
            sectors.sort();
            number = number.add(
                &sectors_used
                    .iter()
                    .position(|x| *x == sectors)
                    .unwrap()
                    .to_string(),
            );
        }

        // digit.print();
        // println!("{:?} : {}", line[1], number);
        sum_total += number.parse::<usize>().unwrap();
    }
    sum_total.to_string()
}

fn import(lines: Vec<String>) -> Vec<Vec<Vec<String>>> {
    lines
        .iter()
        .map(|x| {
            x.split("|")
                .map(|y| {
                    String::from(y)
                        .trim()
                        .split(" ")
                        .map(|j| String::from(j))
                        .collect()
                })
                .collect()
        })
        .collect()
}
