//255 = [
// 254=]
const OPEN: u8 = 255_u8;
const CLOSE: u8 = 254_u8;
struct Day17 {
    result: Vec<u8>,
    hw: Vec<Vec<u8>>,
}
impl Day17 {
    fn new(lines: Vec<String>) -> Self {
        Day17 {
            result: vec![],
            hw: lines
                .into_iter()
                .map(|l| {
                    l.chars()
                        .filter_map(|c| match c {
                            '0'..='9' => Some(c.to_digit(10).unwrap() as u8),
                            '[' => Some(255_u8),
                            ']' => Some(254_u8),
                            _ => None,
                        })
                        .collect()
                })
                .collect(),
        }
    }
    fn find_largest(&mut self) -> usize {
        let mut max = 0;
        for i in 0..self.hw.len() {
            for j in 0..self.hw.len() {
                if i == j {
                    continue;
                }
                self.result = self.hw[i].clone();
                self.add(j, false);
                max = max.max(self.magnitude());
            }
        }
        max
    }
    fn add_all(&mut self) {
        self.result = self.hw.remove(0);
        while self.hw.len() > 0 {
            self.add(0, true);
        }
    }
    fn add(&mut self, i: usize, remove: bool) {
        self.dumb_add(i, remove);
        self.reduce();
    }

    fn dumb_add(&mut self, i: usize, remove: bool) {
        self.result.insert(0, 255);
        if remove {
            self.result.append(&mut self.hw.remove(i));
        } else {
            self.result.append(&mut self.hw[i].clone())
        }
        self.result.push(254);
    }
    fn split(&mut self, i: usize) {
        // println!("SPLIT===={}", format_number(&self.result));
        let split = self.result.remove(i);
        self.result.insert(i, 255);
        self.result.insert(i + 1, split / 2);
        self.result.insert(i + 2, (split / 2) + (split % 2));
        self.result.insert(i + 3, 254);
    }
    fn explode(&mut self, i: usize) {
        // println!("XPLODE==={}", format_number(&self.result));
        let pair: Vec<u8> = self.result.drain(i..i + 4).collect();
        self.result.insert(i, 0);
        let mut j = i;
        while j > 0 {
            if self.result[j - 1] < CLOSE {
                self.result[j - 1] += pair[1];
                break;
            }
            j -= 1;
        }
        let mut j = i + 1;
        while j < self.result.len() {
            if self.result[j] < CLOSE {
                self.result[j] += pair[2];
                break;
            }
            j += 1;
        }
    }
    fn reduce(&mut self) {
        let mut i: usize = 0;
        let mut deepest_open = 0;
        let mut first_over9: usize = 0;
        while i < self.result.len() {
            match self.result[i] {
                OPEN => {
                    deepest_open += 1;
                }
                CLOSE => {
                    if deepest_open > 4 {
                        self.explode(i - 3);
                        return self.reduce();
                    }
                    deepest_open -= 1;
                }
                10..=253 => {
                    if first_over9 == 0 {
                        first_over9 = i;
                    }
                }
                _ => {}
            }
            i += 1;
        }
        if first_over9 != 0 {
            self.split(first_over9);
            return self.reduce();
        }
        return;
    }
    fn magnitude(&self) -> usize {
        let mut data: Vec<usize> = self.result.iter().map(|x| *x as usize).collect();
        for depth in (1..=4).rev() {
            let mut cd = 0;
            let mut i = 0;
            while i < data.len() {
                match data[i] as u8 {
                    OPEN => cd += 1,
                    CLOSE => cd -= 1,
                    _ => {
                        if cd == depth {
                            if data.len() == 3 {
                                return data[1];
                            }
                            let pair: Vec<usize> = data.drain((i - 1)..(i + 3)).collect();
                            data.insert(i - 1, 3 * pair[1] + 2 * pair[2]);
                            i -= 1;
                            cd -= 1;
                        }
                    }
                }
                i += 1;
            }
        }
        data[0]
    }
}

#[allow(dead_code)]
fn format_number(number: &Vec<u8>) -> String {
    number.iter().fold(String::new(), |mut s, x| {
        let c = match *x {
            255 => '[',
            254 => ']',
            0..=9 => x.to_string().chars().nth(0).unwrap(),
            _ => '+',
        };
        if s.len() > 0 {
            let last = s.chars().last().unwrap();
            if !((last == '[')
                || ((last.is_digit(10) || last == '+') && c == ']')
                || (c == ']' && last == ']'))
            {
                s.push(',');
            }
        }
        s.push(c);
        s
    })
}
#[allow(dead_code)]
fn format_number_usize(number: &Vec<usize>) -> String {
    number.iter().fold(String::new(), |mut s, x| {
        let c = match *x {
            255 => '[',
            254 => ']',
            0..=9 => x.to_string().chars().nth(0).unwrap(),
            _ => '+',
        };
        if s.len() > 0 {
            let last = s.chars().last().unwrap();
            if !((last == '[')
                || ((last.is_digit(10) || last == '+') && c == ']')
                || (c == ']' && last == ']'))
            {
                s.push(',');
            }
        }
        s.push(c);
        s
    })
}
pub fn part1(lines: Vec<String>) -> String {
    // let mut state = Day17::new(lines);
    // state.add_all();
    // format!("{}", state.magnitude())
    let mut state = Day17A2::new(lines);
    // state.add_all();
    println!("{}", state.hw[0].to_string());
    format!("{}", "")
}
pub fn part2(lines: Vec<String>) -> String {
    let mut state = Day17::new(lines);
    format!("{}", state.find_largest())
}

struct Value {
    val: Option<u8>,
    l: Option<Box<Value>>,
    r: Option<Box<Value>>,
}
impl Value {
    fn empty() -> Self {
        Value {
            val: None,
            l: None,
            r: None,
        }
    }
    fn push_any(&mut self, v: u8) -> Option<()> {
        // println!("insert:{}", v);
        if self.is_empty() && v < OPEN {
            println!("inserting val:{}", v);
            self.val = Some(v);
            return Some(());
        }
        if self.val.is_some() {
            // println!("returning:{}", v);
            return None;
        }
        match self.l.as_mut() {
            Some(val) => {
                let result = val.push_any(v);
                if result.is_some() {
                    return result;
                }
            }
            None => {
                // println!("PRE NEW LEFT:{}", v);
                if v == OPEN {
                    self.l_update(Value::empty());
                    // println!("NEW LEFT:{}", v);
                    return Some(());
                }
            }
        }

        match self.r.as_mut() {
            Some(val) => {
                let result = val.push_any(v);
                if result.is_some() {
                    return result;
                }
            }
            None => {
                // println!("PRE NEW RIGHT:{}", v);
                if v == OPEN {
                    self.r_update(Value::empty());
                    // println!("NEW RIGHT:{}", v);
                    return Some(());
                }
            }
        }
        return None;
    }
    fn push_branch(&mut self) -> Option<()> {
        self.push_any(OPEN)
    }
    fn push(&mut self, v: u8) -> Option<()> {
        self.push_any(v)
    }
    fn is_empty(&self) -> bool {
        self.l.is_none() && self.r.is_none() && self.val.is_none()
    }
    fn l_update(&mut self, new: Value) {
        self.l = Some(Box::new(new));
    }
    fn r_update(&mut self, new: Value) {
        self.r = Some(Box::new(new));
    }
    fn magnitude(&mut self) -> usize {
        let sv = self.val.clone();
        match sv {
            Some(v) => v as usize,
            None => {
                let mut total = 0;
                match self.l {
                    Some(ref mut v) => total += 3 * v.magnitude(),
                    None => panic!("missing node"),
                }
                match self.r {
                    Some(ref mut v) => total += 2 * v.magnitude(),
                    None => panic!("missing node"),
                }
                total
            }
        }
    }
    fn to_string(&self) -> String {
        if self.val.is_some() {
            return self.val.unwrap().to_string();
        }
        if self.l.is_none() || self.r.is_none() {
            return format!(
                "[{},{}]",
                if self.l.is_some() { "S" } else { "X" },
                if self.r.is_some() { "S" } else { "X" }
            );
        }
        format!(
            "[{},{}]",
            self.l.as_ref().unwrap().to_string(),
            self.r.as_ref().unwrap().to_string()
        )
    }
}
struct Day17A2 {
    result: Value,
    hw: Vec<Value>,
}
impl Day17A2 {
    // fn new(lines: Vec<String>) -> Self {
    //     let mut hw = vec![];
    //     for l in &lines {
    //         let mut root = Value::empty();
    //         let mut stack: Vec<&Value> = vec![&root];
    //         for c in l.chars() {
    //             match c {
    //                 '[' => {
    //                     let new = Value::empty();
    //                     match stack.last_mut() {
    //                         Some(ref mut lm) => {
    //                             stack.push(&new);
    //                             if lm.l.is_none() {
    //                                 lm.l_update(new);
    //                             } else {
    //                                 lm.r_update(new);
    //                             }
    //                         }
    //                         None => {}
    //                     }
    //                 }
    //                 ']' => {
    //                     stack.pop();
    //                 }
    //                 '0'..='9' => {
    //                     let mut lm = stack.last_mut().unwrap();
    //                     lm.val = Some(c.to_digit(10).unwrap() as u8);
    //                     stack.pop();
    //                 }
    //                 _ => {}
    //             }
    //         }
    //         hw.push(root);
    //     }
    fn new(lines: Vec<String>) -> Self {
        let mut hw = vec![];
        for l in &lines {
            let mut root = Value::empty();
            for c in l.chars() {
                match c {
                    '[' => {
                        root.push_branch();
                    }
                    ']' => {}
                    '0'..='9' => {
                        root.push(c.to_digit(10).unwrap() as u8);
                    }
                    _ => {}
                }
            }
            println!("root:{}", root.to_string());
            hw.push(root);
        }

        Day17A2 {
            result: Value::empty(),
            hw: hw,
        }
    }
}
