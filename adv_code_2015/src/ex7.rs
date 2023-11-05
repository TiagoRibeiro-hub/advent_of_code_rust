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
    // let res = lights_on();
    // assert_eq!(res, 569999);
    //println!("res- {:?}", res);
}
