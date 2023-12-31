// https://adventofcode.com/2015/day/6
use std::{fs::File, io};

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    super::read_lines(super::get_current_dir_files() + "/lighting_configuration.txt")
}

pub fn lights_on_2() -> u32 {
    let mut lights: [[u8; 1000]; 1000] = [[0; 1_000]; 1_000]; // column, row => 999,0
    let mut total: u32 = 0;

    if let Ok(lines) = read_lines() {
        for line in lines.flatten() {
            let values = get_idxs_2(&line);
            let col = values.0 .0.abs_diff(values.1 .0);
            let row = values.0 .1.abs_diff(values.1 .1);
            match values.2 {
                "on" => {
                    for i in 0..=row {
                        let r_i = (values.0 .1 + i) as usize;
                        for j in 0..=col {
                            let c_j = (values.0 .0 + j) as usize;
                            let v = lights[r_i][c_j];
                            if v == 0 {
                                total += 1;
                                lights[r_i][c_j] = 1;
                            } 
                        }
                    }
                },
                "off" => {
                    for i in 0..=row {
                        let r_i = (values.0 .1 + i) as usize;
                        for j in 0..=col {
                            let c_j = (values.0 .0 + j) as usize;
                            let v = lights[r_i][c_j];
                            if v == 1 {
                                total -= 1;
                                lights[r_i][c_j] = 0;
                            } 
                        }
                    }
                },
                "toggle" => {
                    for i in 0..=row {
                        let r_i = (values.0 .1 + i) as usize;
                        for j in 0..=col {
                            let c_j = (values.0 .0 + j) as usize;
                            let v = lights[r_i][c_j];
                            if v == 1 {
                                lights[r_i][c_j] = 0;
                                total -= 1;
                            } else {
                                lights[r_i][c_j] = 1;
                                total += 1;
                            }
                        }
                    }
                },
                _ => {}
            }
        }
    }
    total
}


pub fn lights_on() -> u32 {
    let mut lights: [[u8; 1000]; 1000] = [[0; 1_000]; 1_000]; // column, row => 999,0
    let mut total: u32 = 0;

    if let Ok(lines) = read_lines() {
        for line in lines.flatten() {
            let idxs: ((u16, u16), (u16, u16));
            let col: u16;
            let row: u16;
            if line.contains("on") {
                idxs = get_idxs(&line, "on");
                col = idxs.0 .0.abs_diff(idxs.1 .0);
                row = idxs.0 .1.abs_diff(idxs.1 .1);
                for i in 0..=row {
                    let r_i = (idxs.0 .1 + i) as usize;
                    for j in 0..=col {
                        let c_j = (idxs.0 .0 + j) as usize;
                        let v = lights[r_i][c_j];
                        if v == 0 {
                            total += 1;
                            lights[r_i][c_j] = 1;
                        } 
                    }
                }
            } else if line.contains("off") {
                idxs = get_idxs(&line, "off");
                col = idxs.0 .0.abs_diff(idxs.1 .0);
                row = idxs.0 .1.abs_diff(idxs.1 .1);
                for i in 0..=row {
                    let r_i = (idxs.0 .1 + i) as usize;
                    for j in 0..=col {
                        let c_j = (idxs.0 .0 + j) as usize;
                        let v = lights[r_i][c_j];
                        if v == 1 {
                            total -= 1;
                            lights[r_i][c_j] = 0;
                        } 
                    }
                }
            } else if line.contains("toggle") {
                idxs = get_idxs(&line, "toggle");
                col = idxs.0 .0.abs_diff(idxs.1 .0);
                row = idxs.0 .1.abs_diff(idxs.1 .1);
                for i in 0..=row {
                    let r_i = (idxs.0 .1 + i) as usize;
                    for j in 0..=col {
                        let c_j = (idxs.0 .0 + j) as usize;
                        let v = lights[r_i][c_j];
                        if v == 1 {
                            lights[r_i][c_j] = 0;
                            total -= 1;
                        } else {
                            lights[r_i][c_j] = 1;
                            total += 1;
                        }
                    }
                }
            }
        }
    }

    total
}

pub fn brightness() -> u32 {
    let mut lights: [[u8; 1000]; 1000] = [[0; 1_000]; 1_000]; // column, row => 999,0
    let mut total: u32 = 0;

    if let Ok(lines) = read_lines() {
        for line in lines.flatten() {
            let idxs: ((u16, u16), (u16, u16));
            let col: u16;
            let row: u16;
            if line.contains("on") {
                idxs = get_idxs(&line, "on");
                col = idxs.0 .0.abs_diff(idxs.1 .0);
                row = idxs.0 .1.abs_diff(idxs.1 .1);
                for i in 0..=row {
                    let r_i = (idxs.0 .1 + i) as usize;
                    for j in 0..=col {
                        let c_j = (idxs.0 .0 + j) as usize;
                        let v = lights[r_i][c_j];
                        total += 1;
                        lights[r_i][c_j] = v + 1;
                    }
                }
            } else if line.contains("off") {
                idxs = get_idxs(&line, "off");
                col = idxs.0 .0.abs_diff(idxs.1 .0);
                row = idxs.0 .1.abs_diff(idxs.1 .1);
                for i in 0..=row {
                    let r_i = (idxs.0 .1 + i) as usize;
                    for j in 0..=col {
                        let c_j = (idxs.0 .0 + j) as usize;
                        let v = lights[r_i][c_j];
                        if v > 0 {
                            total -= 1;
                            lights[r_i][c_j] = v - 1;
                        }
                    }
                }
            } else if line.contains("toggle") {
                idxs = get_idxs(&line, "toggle");
                col = idxs.0 .0.abs_diff(idxs.1 .0);
                row = idxs.0 .1.abs_diff(idxs.1 .1);
                for i in 0..=row {
                    let r_i = (idxs.0 .1 + i) as usize;
                    for j in 0..=col {
                        let c_j = (idxs.0 .0 + j) as usize;
                        let v = lights[r_i][c_j];
                        total += 2;
                        lights[r_i][c_j] = v + 2;
                    }
                }
            }
        }
    }

    total
}

#[test]
fn test() {
    //let res = lights_on_2();
    let res = lights_on();
    assert_eq!(res, 569999);
    let res = brightness();
    assert_eq!(res, 17836115);
    //println!("res- {:?}", res);
}

// BENCH MARK GET IDXS FUNCS
fn get_idxs(line: &str, conf: &str) -> ((u16, u16), (u16, u16)) {
    let res = line.split(' ');
    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut next = false;
    let mut first = true;
    for part in res.into_iter() {
        if next {
            let s = part.split(',');
            let mut i = 0;
            if first {
                for item in s {
                    if i == 0 {
                        start.0 = item.parse::<u16>().unwrap();
                        i += 1;
                    } else {
                        start.1 = item.parse::<u16>().unwrap();
                    }
                }
                first = false;
            } else {
                for item in s {
                    if i == 0 {
                        end.0 = item.parse::<u16>().unwrap();
                        i += 1;
                    } else {
                        end.1 = item.parse::<u16>().unwrap();
                    }
                }
                break;
            }
            next = false;
        } else if part == conf || part == "through" {
            next = true;
        }
    }

    (start, end)
}

fn get_idxs_split(line: &str, conf: &str) -> ((u16, u16), (u16, u16)) {
    let res = line.split(conf).nth(1).unwrap();
    let mut first = true;

    let mut start = (0, 0);
    res.split(' ').nth(0).unwrap().split(',').for_each(|v| {
        if first {
            start.0 = v.parse::<u16>().unwrap();
            first = false;
        } else {
            start.1 = v.parse::<u16>().unwrap();
        }
    });
    // through
    first = true;
    let mut end = (0, 0);
    res.split(' ').nth(2).unwrap().split(',').for_each(|v| {
        if first {
            end.0 = v.parse::<u16>().unwrap();
            first = false;
        } else {
            end.1 = v.parse::<u16>().unwrap();
        }
    });

    (start, end)
}

fn get_idxs_2(line: &str) -> ((u16, u16), (u16, u16), &str) {
    let mut parts = line.split(|c| c == ' ' || c == ',');
    let mut opt = parts.next().unwrap();
		if opt == "turn" {
			opt = parts.next().unwrap();
		};

    let mut start = (0, 0);
    start.0 = parts.next().unwrap().parse::<u16>().unwrap();
    start.1 = parts.next().unwrap().parse::<u16>().unwrap();
    // through
    parts.next();
    let mut end = (0, 0);
    end.0 = parts.next().unwrap().parse::<u16>().unwrap();
    end.1 = parts.next().unwrap().parse::<u16>().unwrap();

    (start, end, opt)
}

// BENCH MARK GET IDXS
pub fn lights_on_get_idxs_2() {
    if let Ok(lines) = read_lines() {
        for line in lines.flatten() {
            get_idxs_2(&line);
        }
    }
}

pub fn lights_on_get_idxs_split() {
    if let Ok(lines) = read_lines() {
        for line in lines.flatten() {
            if line.contains("on") {
                get_idxs_split(&line, "on ");
            } else if line.contains("off") {
                get_idxs_split(&line, "off ");
            } else if line.contains("toggle") {
                get_idxs_split(&line, "toggle ");
            }
        }
    }
}

pub fn lights_on_get_idxs() {
    if let Ok(lines) = read_lines() {
        for line in lines.flatten() {
            if line.contains("on") {
                get_idxs(&line, "on");
            } else if line.contains("off") {
                get_idxs(&line, "off");
            } else if line.contains("toggle") {
                get_idxs(&line, "toggle");
            }
        }
    }
}
