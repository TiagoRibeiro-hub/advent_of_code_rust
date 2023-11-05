// https://adventofcode.com/2015/day/7
use std::{fs::File, io};

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    super::read_lines(super::get_current_dir_files() + "/signals.txt")
}

pub fn get_wire_a() {
    if let Ok(lines) = read_lines() {
        for line in lines.flatten() {
            todo!()
        }
    }
}

#[test]
fn test() {
    example()
    // let res = lights_on();
    // assert_eq!(res, 569999);
    //println!("res- {:?}", res);
}

fn example() {
    let x = 123;
    let y = 456;
    let d = x & y;
    let e = x | y;
    let f = x << 2;
    let g = y >> 2;
    let mut h = !x;
    let mut i = !y;
    
    if h < 0 {
        h = 65535 - x;
    }
    if i < 0 {
        i = 65535 - y;
    }
    println!("{d}");// d: 72
    println!("{e}");// e: 507
    println!("{f}");// f: 492
    println!("{g}");// g: 114
    println!("{h}");// h: 65412
    println!("{i}");// i: 65079
    println!("{x}");// x: 123
    println!("{y}");// y: 456
    }