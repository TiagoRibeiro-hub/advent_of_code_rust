// https://adventofcode.com/2015/day/6
use std::{collections::HashMap, fs::File, io};

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    super::read_lines(super::get_current_dir_files() + "/lighting_configuration.txt")
}

pub fn lights_on() {
    if let Ok(lines) = read_lines() {
        for value in lines.flatten() {
            
        }
    }
}

#[test]
fn test() {
    let res = lights_on();
    //assert_eq!(res, );
    println!("res- {:?}", res);
}
