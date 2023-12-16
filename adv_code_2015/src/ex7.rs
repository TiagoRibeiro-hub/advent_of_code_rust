// https://adventofcode.com/2015/day/7
use std::{collections::HashMap, fs::File, io};

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    super::read_lines(super::get_current_dir_files() + "/signals.txt")
}

#[derive(Debug)]
enum Operation {
    Not,
    And,
    Or,
    Lshift,
    Rshift,
}
#[derive(Debug)]
struct Signals {
    pub first: String,
    pub second: Option<String>,
    pub op: Option<Operation>,
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

fn get_value_string(value: &str) -> String {
    let sec_string: String;
    if let Ok(nr) = value.parse::<i64>() {
        sec_string = nr.to_string();
    }
    else {
        sec_string = value.to_string();
    }
    sec_string
}

fn get_operation(op: &str) -> Operation {
    //* "AND", "OR", "LSHIFT", "RSHIFT"
    let mut operation = Operation::And;
    if op == "OR" {
        operation = Operation::Or;
    }
    else if op == "LSHIFT" {
        operation = Operation::Lshift;
    }
    else if op == "RSHIFT" {
        operation = Operation::Rshift;
    }
    else if op == "NOT" {
        panic!("Operation Not can not be set here")
    }
    operation
}

pub fn get_signal_a() -> i64 {
    // lf AND lq -> ls => key ls, value lf, lq, AND
    let mut wires: HashMap<String, Signals> = HashMap::new();
    // 0 -> c => key c, value 0
    let mut signals: HashMap<String, i64> = HashMap::new();

    if let Ok(lines) = read_lines() {
        for line in lines.flatten() {
            //* cm AND co -> cp
            let mut parts = line.split(' ');
            let first = parts.next().unwrap();
            let res = first.parse::<i64>();
            match res {
                Ok(value) => {
                    let op = parts.next().unwrap();
                    if op == "->" {
                        // ! 0 -> cu
                        let key = parts.last().unwrap().to_string();
                        signals.insert(key, value);
                    }else {
                        // ! 1 AND lu -> lv || 1 AND 2 -> lv
                        let second = parts.next().unwrap();
                        parts.next(); // ->
                        wires
                        .insert(
                            parts.next().unwrap().to_string(), 
                            Signals::new(
                                first.to_string(), 
                                Some(get_value_string(second)), 
                                Some(get_operation(op)))
                        );
                    }
                }
                Err(_) => {
                    if first == "NOT" {
                        // ! NOT kt -> ku || NOT 1 -> ku
                        let second = parts.next().unwrap();
                        wires.insert(
                            parts.last().unwrap().to_string(), 
                            Signals::new(
                                get_value_string(second), 
                                None, 
                                Some(Operation::Not)
                            )
                        );
                    } else {
                        let op = parts.next().unwrap();
                        if op == "->" {
                            // ! ge -> gg || 1 -> gg
                            wires.insert(
                                parts.next().unwrap().to_string(),
                                 Signals::new(
                                    get_value_string(first),
                                    None,
                                    None)
                                );
                        } else {
                            // gf OR ge -> gg || 1 OR ge -> gg || 1 OR 2 -> gg || gf OR 1 -> gg
                            let second = parts.next().unwrap();
                            parts.next(); // ->
                            wires
                                .insert(
                                    parts.next().unwrap().to_string(), 
                                    Signals::new(
                                        get_value_string(first), 
                                        Some(get_value_string(second)), 
                                        Some(get_operation(op)))
                                );
                        }
                    }
                }
            }
        }
    }

    get_signal(&wires, &mut signals, &String::from("a"))
}

fn get_signal(wires: &HashMap<String, Signals>, signals: &mut HashMap<String, i64>, key: &String) -> i64 {
    let mut wire_value;
    match signals.get(key) {
        Some(value) => {
            wire_value = *value;
            return wire_value;
        },
        None => {
            if let Some(sig) = wires.get(key) {
                if let Some(op) = &sig.op {
                    if let Some(sec) = &sig.second {
                        let f;
                        // ! 1 OR ge -> gg
                        if let Ok(v) = sig.first.parse::<i64>() {
                            f = v;
                        }
                        else {
                            // ! gf OR ge -> gg
                            f = get_signal(wires, signals, &sig.first);
                        }
                        let s;
                        // ! f OR 2 -> gg
                        if let Ok(v) = sec.parse::<i64>() {
                            s = v;
                        }
                        else {
                            // ! f OR ge -> gg
                            s = get_signal(wires, signals, &sec);
                        }
                        //* "AND", "OR", "LSHIFT", "RSHIFT"
                        match op {
                            Operation::Not => {
                                panic!("Operation Not can not be done here")
                            },
                            Operation::And => wire_value = f & s,
                            Operation::Or => wire_value = f | s,
                            Operation::Lshift => wire_value = f << s,
                            Operation::Rshift => wire_value = f >> s,
                        }
                    }
                    else {
                        let nr;
                        // ! NOT 1 -> ku
                        if let Ok(v) = sig.first.parse::<i64>() {
                            nr = v;
                        }
                        else {
                            // ! NOT kt -> ku
                            nr = get_signal(wires, signals, &sig.first);
                        }
                        wire_value = !nr;
                        if wire_value < 0 {
                            wire_value = 65535 - nr;
                        }
                    }
                }
                else {
                    // ! 1 -> gg
                    if let Ok(nr) = sig.first.parse::<i64>() {
                        wire_value = nr;
                    }
                    else {
                        // ! ge -> gg
                        wire_value = get_signal(wires, signals, &sig.first);
                    }     
                }
            }
            else {
                panic!("Must always have signals");
            }
        } 
    }
    if wire_value > 65535 {
        panic!("Bigger than 65535")
    }
    signals.insert(key.to_string(), wire_value);
    wire_value
}


#[test]
fn test() {
    //example()
    let res = get_signal_a();
    assert_eq!(res, 16076);
    println!("res- {:?}", res);
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
