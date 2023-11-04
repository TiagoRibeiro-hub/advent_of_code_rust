// * https://adventofcode.com/2015/day/1

use std::fs;

pub fn read_input_one() -> String {
    fs::read_to_string(super::get_current_dir_files() + "/floor.txt")
        .expect("Should have been able to read the file")
}

pub fn floor_chars(input: &str) -> i32 {
    let mut floor: i32 = 0;
    for c in input.chars() {
        match c {
            '(' => {
                floor += 1;
            }
            ')' => {
                floor -= 1;
            }
            _ => {}
        }
    }
    floor
}

pub fn floor_bytes(input: &str) -> i32 {
    let mut floor: i32 = 0;
    for &item in input.as_bytes().iter() {
        if item == b'(' {
            floor += 1;
        } else if item == b')' {
            floor -= 1;
        }
    }
    floor
}

pub fn floor_basement_same_var(input: &str) -> i32 {
    let mut floor: i32 = 0;
    for (i, &item) in input.as_bytes().iter().enumerate() {
        if item == b'(' {
            floor += 1;
        } else if item == b')' {
            floor -= 1;
        }
        if floor == -1 {
            floor = i as i32 + 1;
            break;
        }
    }
    floor
}

pub fn floor_basement(input: &str) -> usize {
    let mut floor: i32 = 0;
    let mut res: usize = 0;
    for (i, &item) in input.as_bytes().iter().enumerate() {
        if item == b'(' {
            floor += 1;
        } else if item == b')' {
            floor -= 1;
        }
        if floor == -1 {
            res = i + 1;
            break;
        }
    }
    res
}

#[test]
fn test() {
    let input = read_input_one();
    let floor = floor_bytes(&input);
    assert_eq!(floor, 138);
    let floor = floor_basement(&input);
    assert_eq!(floor, 1771);
    println!("{floor}");
}
