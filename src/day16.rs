struct Day16 {
    chars: Vec<usize>,
    i: usize,
}

struct Packet {
    type_id: usize,
    version_sum: usize,
    value: usize,
}

impl Packet {
    fn new(version: usize, type_id: usize) -> Self {
        Packet {
            type_id,
            version_sum: version,
            value: match type_id {
                1 => 1,
                2 => usize::MAX,
                _ => 0,
            },
        }
    }
    fn include(&mut self, p: Packet) {
        self.version_sum += p.version_sum;
        match self.type_id {
            0 => self.value += p.value,
            1 => self.value *= p.value,
            2 => self.value = self.value.min(p.value),
            3 => self.value = self.value.max(p.value),
            _ => panic!("4 is lit, 5-7 use other include"),
        }
    }
    fn include_comparison(&mut self, p1: Packet, p2: Packet) {
        self.version_sum += p1.version_sum + p2.version_sum;
        match self.type_id {
            5 => self.value = (p1.value > p2.value) as usize,
            6 => self.value = (p1.value < p2.value) as usize,
            7 => self.value = (p1.value == p2.value) as usize,
            _ => panic!("4 is lit, 1-4 manual"),
        }
    }
}

impl Day16 {
    fn new(lines: Vec<String>) -> Self {
        Day16 {
            chars: lines[0].chars().into_iter().fold(vec![], |mut chars, c| {
                let digit = c.to_digit(16).unwrap() as usize;
                for i in (0..4).rev() {
                    chars.push((digit >> i) & 0b_0001);
                }
                chars
            }),
            i: 0,
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!(
            "{}",
            self.chars
                .iter()
                .fold(String::new(), |s, i| s + &i.to_string()),
        );
    }

    fn decode(&mut self) -> Packet {
        let mut packet = Packet::new(self.get_dec(3), self.get_dec(3));
        match packet.type_id {
            4 => {
                //literal [not_last_group,bit1,bit2,bit3]
                packet.value = self.get_literal();
            }
            _ => {
                //other operator
                let length_type_id = self.get_bit();
                if length_type_id {
                    //next 11 bits are a number that reps the number of sub-packets immediately contained by this packet
                    let sub_packets = self.get_dec(11);
                    if packet.type_id > 4 {
                        packet.include_comparison(self.decode(), self.decode());
                    } else {
                        for _ in 0..sub_packets {
                            packet.include(self.decode());
                        }
                    }
                } else {
                    //next 15 bits are a number that reps the total length in bits of the sub-packets contained in this packet
                    let bits_of_subpackets = self.get_dec(15);
                    if packet.type_id > 4 {
                        packet.include_comparison(self.decode(), self.decode());
                    } else {
                        let before = self.i;
                        while self.i - before < bits_of_subpackets {
                            packet.include(self.decode());
                        }
                    }
                }
            }
        }
        packet
    }

    fn get_literal(&mut self) -> usize {
        let mut big_byte = 0;
        while self.i < self.chars.len() - 5 {
            let last_group = !self.get_bit();
            for _ in 0..4 {
                big_byte <<= 1;
                big_byte |= self.get_dec_single();
            }
            if last_group {
                break;
            }
        }
        big_byte
    }

    fn get_dec_single(&mut self) -> usize {
        self.i += 1;
        self.chars[self.i - 1] as usize
    }

    fn get_dec(&mut self, bits: usize) -> usize {
        let mut total = 0;
        for i in 0..bits {
            total += (self.chars[self.i + i] << bits - 1 - i) as usize
        }
        self.i += bits;
        total
    }

    fn get_bit(&mut self) -> bool {
        self.get_dec_single() == 1
    }
}
pub fn part1(lines: Vec<String>) -> String {
    let packet = Day16::new(lines.clone()).decode();
    format!("p1: {}, p2: {}", packet.version_sum, packet.value)
}
pub fn part2(_: Vec<String>) -> String {
    String::from("NA - part 1")
}
