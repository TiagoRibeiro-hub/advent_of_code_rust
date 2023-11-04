// https://adventofcode.com/2015/day/6
use std::{collections::HashMap, fs::File, io};

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    super::read_lines(super::get_current_dir_files() + "/lighting_configuration.txt")
}

fn get_idxs(line: &String, conf: &str) -> ((u16, u16), (u16, u16)) {
    let res = line.split(" ");
    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut next = false;
    let mut first = true;
    for part in res.into_iter(){
        if next {
            let s = part.split(",").into_iter();
            let mut i = 0;
            if first {
                for item in s {
                    if i == 0 {
                        start.0 = item.parse::<u16>().unwrap();
                        i += 1;
                    }
                    else {
                        start.1 = item.parse::<u16>().unwrap();
                    }
                }
                first = false;
            }
            else {
                for item in s {
                    if i == 0 {
                        end.0 = item.parse::<u16>().unwrap();
                        i += 1;
                    }
                    else {
                        end.1 = item.parse::<u16>().unwrap();
                    }
                }
                break;
            }
            next = false;
        }
        else if part == conf || part == "through"{
            next = true;
        }
    }

    (start, end)
}

#[allow(unused)]
pub fn lights_on() -> u16 {
    let mut lights_on: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut lights_off: HashMap<i32, Vec<i32>> = HashMap::new();


    if let Ok(lines) = read_lines() {
        for line in lines.flatten() {
            let mut idxs = ((0,0), (0,0));
            if line.contains("on") {
                idxs = get_idxs(&line, "on");
            } else if line.contains("off") {
                idxs = get_idxs(&line, "off");
            } else if line.contains("toggle") {
                idxs = get_idxs(&line, "toggle");
            }

        }
    }

    0
}



#[test]
fn test() {
    let res = lights_on();
    //assert_eq!(res, );
    println!("res- {:?}", res);
}


// BENCH MARK GET IDXS
fn get_idxs_split(line: &String, conf: &str) -> ((u16, u16), (u16, u16)) {
    let res = line.split(conf).nth(1).unwrap();
    let mut first = true;

    let mut start = (0, 0);
    res.split(" ")
        .nth(0)
        .unwrap()
        .split(",")
        .for_each(|v| {
            if first {
                start.0 = v.parse::<u16>().unwrap();
                first = false;
            }
            else {
                start.1 = v.parse::<u16>().unwrap();
            }
        });
    // through 
    first = true;
    let mut end = (0, 0);
    res.split(" ")
        .nth(2)
        .unwrap()
        .split(",")
        .for_each(|v| {
            if first {
                end.0 = v.parse::<u16>().unwrap();
                first = false;
            }
            else {
                end.1 = v.parse::<u16>().unwrap();
            }
        });

    (start, end)
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