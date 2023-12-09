// https://adventofcode.com/2015/day/7
use std::{collections::HashMap, fs::File, io};


fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    super::read_lines(super::get_current_dir_files() + "/signals.txt")
}


#[derive(Debug)]
struct Signals {
    pub first: String,
    pub second: Option<String>,
    pub op: Option<Operation>,
}
#[derive(Debug)]
enum Operation {
    NOT,
    AND,
    OR,
    LSHIFT,
    RSHIFT,
}

impl Signals {
    fn new(first: String, second: Option<String>, op: Option<Operation>) -> Self {
        Signals {
            first: first,
            second: second,
            op: op,
        }
    }
}

pub fn get_wire_a() -> i64 {
    // lf AND lq -> ls => key ls, value lf, lq, AND
    let mut signals: HashMap<String, Signals> = HashMap::new();
    // 0 -> c => key c, value 0
    let mut wires: HashMap<String, i64> = HashMap::new();

    if let Ok(lines) = read_lines() {
        for line in lines.flatten() {
            let mut parts = line.split(' ');
            let first = parts.next().unwrap();
            let res = first.parse::<i64>();
            match res {
                Ok(value) => {
                    //* 0 -> c
                    let key = parts.last().unwrap().to_string();
                    wires.insert(key, value);
                }
                Err(_) => {
                    if first == "NOT" {
                        //* NOT kt -> ku
                        let second = parts.next().unwrap();
                        let key = parts.last().unwrap().to_string();
                        signals.insert(
                            key, 
                            Signals::new(
                                second.to_string(), 
                                None, 
                                Some(Operation::NOT)
                            )
                        );
                    } else {
                        let op = parts.next().unwrap();
                        if op == "->" {
                            //* ge -> gg
                            let key = parts.next().unwrap().to_string();
                            signals.insert(
                                key,
                                 Signals::new(
                                    first.to_string(),
                                    None,
                                    None)
                                );
                        } else {
                            // gf OR ge -> gg
                            let second = parts.next().unwrap();
                            parts.next(); // ->
                            let key = parts.next().unwrap().to_string();
                            //* "AND", "OR", "LSHIFT", "RSHIFT"
                            let mut operation = Operation::AND;
                            if op == "OR" {
                                operation = Operation::OR;
                            }
                            else if op == "LSHIFT" {
                                operation = Operation::LSHIFT;
                            }
                            else if op == "RSHIFT" {
                                operation = Operation::RSHIFT;
                            }
                            signals
                                .insert(
                                    key, 
                                    Signals::new(
                                        first.to_string(), 
                                        Some(second.to_string()), 
                                        Some(operation))
                                );
                        }
                    }
                }
            }
        }
    }

    get_wire(&signals, &mut wires, &String::from("a"))
}

#[allow(unused)]
fn get_wire(signals: &HashMap<String, Signals>, wires: &mut HashMap<String, i64>, key: &String) -> i64 {
    let mut wire_value: i64 = 0;
    match wires.get(key) {
        Some(value) => {
            wire_value = *value;
        },
        None => {
            if let Some(sig) = signals.get(key) {
                if let Some(op) = &sig.op {
                    if let Some(sec) = &sig.second {
                        // gf OR ge -> gg
                        let mut f = 0;
                        if let Ok(nr) = sig.first.parse::<i64>() {
                            f = nr;
                        }
                        else {
                            f = get_wire(signals, wires, &sig.first);
                        }
                        let mut s = 0;
                        if let Ok(nr) = sec.parse::<i64>() {
                            s = nr;
                        }
                        else {
                            s = get_wire(signals, wires, &sec);
                        }
                        // * "AND", "OR", "LSHIFT", "RSHIFT"
                        match op {
                            Operation::NOT => {
                                panic!("Operation Not can not be done here")
                            },
                            Operation::AND => wire_value = f & s,
                            Operation::OR => wire_value = f | s,
                            Operation::LSHIFT => wire_value = f << s,
                            Operation::RSHIFT => wire_value = f >> s,
                        }

                    }
                    else {
                        // * NOT kt -> ku
                        let mut nr = 0;
                        if let Ok(v) = sig.first.parse::<i64>() {
                            nr = v;
                        }
                        else {
                            nr = get_wire(signals, wires, &sig.first);
                        }

                        wire_value = !nr;
                        if wire_value < 0 {
                            wire_value = 65535 - nr;
                        }
                        wires.insert(key.to_string(), wire_value);
                    }
                }
                else {
                    // * ge -> gg
                    if let Ok(nr) = sig.first.parse::<i64>() {
                        wire_value = nr;
                    }
                    else {
                        wire_value = get_wire(signals, wires, &sig.first);
                    }
                    wires.insert(key.to_string(), wire_value);
                }
            }
            else {
                panic!("Must always have signals");
            }
        } 
    }
    wire_value
}


#[test]
fn test() {
    //example()
    let res = get_wire_a();
    // assert_eq!(res, 569999);
    println!("res- {:?}", res); // < 65535
}

fn example() {
    let x = 123;
    let y = 456;
    let d = x & y; // AND
    let e = x | y; // OR
    let f = x << 2; // LSHIFT
    let g = y >> 2; // RSHIFT
    let mut h = !x;
    let mut i = !y;

    if h < 0 {
        h = 65535 - x;
    }
    if i < 0 {
        i = 65535 - y;
    }
    println!("{d}"); // d: 72
    println!("{e}"); // e: 507
    println!("{f}"); // f: 492
    println!("{g}"); // g: 114
    println!("{h}"); // h: 65412
    println!("{i}"); // i: 65079
    println!("{x}"); // x: 123
    println!("{y}"); // y: 456
}
