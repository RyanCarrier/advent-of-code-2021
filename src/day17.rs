struct Day17 {
    x: Vec<i64>,
    y: Vec<i64>,
}

impl Day17 {
    fn new(lines: Vec<String>) -> Self {
        let split: Vec<&str> = lines[0].split(", ").collect();
        let xsplit: Vec<&str> = split[0].split("..").collect();
        let ysplit: Vec<&str> = split[1].split("..").collect();
        Day17 {
            x: (xsplit[0].split("=").collect::<Vec<&str>>()[1]
                .parse::<i64>()
                .unwrap()..xsplit[1].parse::<i64>().unwrap() + 1)
                .collect(),
            y: (ysplit[0].split("=").collect::<Vec<&str>>()[1]
                .parse::<i64>()
                .unwrap()..ysplit[1].parse::<i64>().unwrap() + 1)
                .collect(),
        }
    }

    fn x_options(&self) -> Vec<(i64, i64)> {
        // at x[-1],
        let mut x_options: Vec<(i64, i64)> = vec![];
        //t should be usize not i64, but it will ALWAYS need to be casted for comparison....
        let x_too_far = self.x.last().unwrap() + 1;
        //could start 0 but.... s=(u_x*(u_x+1))/2, 2s=u_x(u_x+1), 2s<(u_x+1)2, sqrt(2s)-1<u_x
        //any starting vel lower and it will never reach
        let mut u_x = (((self.x[0] * 2) as f64).sqrt() - 1.0).ceil() as i64;
        while u_x < x_too_far {
            let mut t = 0;
            while t <= u_x {
                let s = u_x * t - (t * (t - 1)) / 2;
                if s >= x_too_far {
                    break;
                }
                if s >= self.x[0] {
                    x_options.push((u_x, t))
                }
                t += 1
            }
            u_x += 1;
        }
        // x_options.sort_unstable();
        x_options
    }
    fn y_options(&self) -> Vec<(i64, i64)> {
        let mut u_y = self.y[0];
        let mut y_options: Vec<(i64, i64)> = vec![];
        let last = *self.y.last().unwrap();
        while u_y >= self.y[0] && u_y <= 0 {
            let mut t = 1;
            loop {
                let s = u_y * t - (t * (t - 1)) / 2;
                if s < self.y[0] {
                    break;
                }
                if s <= last {
                    y_options.push((u_y, t));
                    y_options.push((-u_y - 1, 2 * (-u_y) - 1 + t));
                }
                t += 1;
            }
            u_y += 1;
        }
        // y_options.sort_unstable();
        y_options
    }

    fn options(&self) -> (i64, Vec<[i64; 2]>) {
        let x_options: Vec<(i64, i64)> = self.x_options();
        let y_options: Vec<(i64, i64)> = self.y_options();
        let mut options: Vec<[i64; 2]> = vec![];
        let mut best_uy = i64::MIN;
        for (u_x, t_x) in x_options {
            if u_x == t_x {
                //terminal vel
                y_options
                    .iter()
                    .filter(|(_, t)| *t >= t_x)
                    .for_each(|(u_y, _)| {
                        options.push([u_x, *u_y]);
                        if u_y > &best_uy {
                            best_uy = best_uy.max(*u_y);
                        }
                    });
            } else {
                y_options
                    .iter()
                    .filter(|(_, t)| *t == t_x)
                    .for_each(|(u_y, _)| {
                        options.push([u_x, *u_y]);
                        if u_y > &best_uy {
                            best_uy = best_uy.max(*u_y);
                        }
                    });
            }
        }
        ((best_uy * (best_uy + 1)) / 2, options)
    }

    #[allow(dead_code)]
    fn step(&self, mut position: [i64; 2], mut velocity: [i64; 2]) -> ([i64; 2], [i64; 2]) {
        position[0] += velocity[0];
        position[1] += velocity[1];
        if self.x.contains(&position[0]) && self.y.contains(&position[1]) {
            println!("in range");
        }
        velocity[0] = 0.max(velocity[0] - 1);
        velocity[1] -= 1;
        (position, velocity)
    }
}
pub fn part1(lines: Vec<String>) -> String {
    let state = Day17::new(lines.clone());
    let (best_height, mut options) = state.options();
    // println!("{:?}", options);
    options.sort_unstable();
    // println!("{:?}", options);
    options.dedup();
    // println!("{:?}", options);

    format!("p1: {}, p2: {}", best_height, options.len())
}
pub fn part2(_: Vec<String>) -> String {
    String::from("NA - part 1")
}

/*
GRAVEYARD


                //u_x is terminal (go long)
                //my amazing quadratic...
                // let mut u_y = 0;
                // while u_y < self.y[0].abs() {
                //     for s_y in &self.y {
                //         if u_x == 6 && u_y == 9 {
                //             println!(
                //                 "ux:{}, uy:{}, uyuy:{}, 2sy:{}",
                //                 u_x,
                //                 u_y,
                //                 u_y * u_y,
                //                 2 * s_y
                //             );
                //         }
                //         if u_y * u_y < 2 * s_y {
                //             continue;
                //         }
                //         let t_4ac = ((u_y * u_y - 2 * s_y) as f64).sqrt();
                //         if u_x == 6 && u_y == 9 {
                //             println!("ux:{}, uy:{}, t_4ac:{}", u_x, u_y, t_4ac);
                //         }
                //         if t_4ac.fract() > 1e-10 {
                //             continue;
                //         }
                //         let t_4ac_i = t_4ac as i64;
                //         if (t_4ac_i + u_y) < t_x {
                //             continue;
                //         }
                //         //two t options here, but we don't care
                //         best_height = best_height.max((u_y * u_y) / 2);
                //         options.push([u_x, u_y]);
                //     }
                //     u_y += 1;
                // }


                fn x_options(&self) -> Vec<(i64, i64)> {
        // at x[-1],
        let mut x_options: Vec<(i64, i64)> = vec![];
        //t should be usize not i64, but it will ALWAYS need to be casted for comparison....
        let x_too_far = self.x.last().unwrap() + 1;
        //could start 0 but.... s=(u_x*(u_x+1))/2, 2s=u_x(u_x+1), 2s<(u_x+1)2, sqrt(2s)-1<u_x
        let mut u_x = ((self.x[0] * 2) as f64).sqrt().ceil() as i64 - 1;
        // println!("x_0:{}, x_1:{}", self.x[0], self.x.last().unwrap());
        // println!("starting u_x:{}", u_x);
        //any starting vel lower and it will never reach
        while u_x < x_too_far {
            //this while filter only works for lobbed shots, which works fine for height, will not work for shots directly towards the endzone, though highest shot counts so this should work
            //filter takes v=0, when x drag has full taken over velocity, and so would assume to be falling straight down, if this is not in the zone then it can't be correct

            //this can be used as u>sqrt(u2-rs) == u2>u2-2s, s is always positive so sqrt always positive
            //this is the check for if they are accurate without reaching terminal velocity; this doesn't work for terminal as a becomes 0
            self.x.iter().for_each(|x| {
                if u_x * u_x > 2 * x {
                    let t_4ac = ((u_x * u_x - 2 * x) as f64).sqrt();
                    if t_4ac.fract() < 1e-10 {
                        let t_4ac_i = t_4ac as i64;
                        // println!("t_4ac_i:{}", t_4ac_i);
                        // x_options.push((u_x, (u_x + t_4ac_i) / 2));
                        if t_4ac_i < u_x {
                            let t = u_x - t_4ac_i;
                            // println!(
                            //     "pushing at s_x:{} -> ({},{}), u_x*t+at2:{}",
                            //     x,
                            //     u_x,
                            //     t,
                            //     u_x * t - ((t * t) / 2)
                            // );
                            x_options.push((u_x, t));
                        }
                    }
                }
            });

            // if (u_x * (u_x + 1)) / 2 >= self.x[0] && (u_x * (u_x + 1)) / 2 < x_too_far {
            //terminal velocity and will be in range from time t
            // x_options.push((u_x, u_x))

            // let t = u_x;
            // let s = u_x * t - (t * t);
            // if s >= self.x[0] && s <= x_too_far {
            //     x_options.push((u_x, u_x))
            // }

            // }
            u_x += 1;
        }
        x_options.sort_unstable();
        x_options
    }
                */
