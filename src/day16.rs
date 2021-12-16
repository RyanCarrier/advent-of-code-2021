use std::{collections::HashMap, ops::Range};

struct Day16 {
    chars: Vec<u8>,
}

enum TypeId {
    Literal,
}

struct Packet {
    version: u8,
    type_id: u8,
}

impl Day16 {
    fn new(lines: Vec<String>) -> Self {
        Day16 {
            chars: lines[0].chars().into_iter().fold(vec![], |mut chars, c| {
                let digit = c.to_digit(16).unwrap() as u8;
                for i in (0..4).rev() {
                    chars.push((digit >> i) & 0b_0001);
                }
                chars
            }),
        }
    }

    fn print(&self) {
        println!(
            "{}",
            self.chars
                .iter()
                .fold(String::new(), |s, i| s + &i.to_string()),
        );
    }

    fn decode(&mut self) -> String {
        //id's
        /*
        0 = 15 bits are a number that rep the total length in bits of sub packets
        1 = 11 bits are a number that rep the number of subpackets immediately contained by this packet


        */
        // id_map.insert(4, TypeId::Literal);
        let mut i = 0;

        let version = self.get_dec(i..i + 3);
        i += 3;
        let id = self.get_dec(i..i + 3);
        match id {
            4 => { //literal just keep going?
            }
            _ => {
                //other operator
            }
        }
        let mut big_byte: u64 = 0;
        while i < self.chars.len() - 5 {
            let last_group = self.get_dec(i..i + 1) == 0;
            i += 1;
            for _ in 0..4 {
                big_byte <<= 1;
                big_byte |= self.get_dec(i..i + 1) as u64;
                i += 1;
            }
            if last_group {
                break;
            }
        }
        println!("ver:{}, id:{}", version, id);
        String::new()
    }

    fn get_dec(&self, range: Range<usize>) -> u8 {
        let mut total = 0;
        for (i, r) in range.rev().enumerate() {
            total += self.chars[r] << i
        }
        total
    }
}
pub fn part1(lines: Vec<String>) -> String {
    let mut data = Day16::new(lines.clone());
    data.print();
    let ret = format!("p1: {}, p2: {}", data.decode(), "");
    ret
}
pub fn part2(_: Vec<String>) -> String {
    String::from("NA - part 1")
}
