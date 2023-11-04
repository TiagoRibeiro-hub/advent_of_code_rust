use std::collections::{HashMap, HashSet};

// https://adventofcode.com/2015/day/3
// Moves are always exactly one house to the north (^), south (v), east (>), or west (<)

pub fn read_input_three() -> String {
    std::fs::read_to_string(
        super::get_current_dir_files() + "/grid_of_houses.txt",
    )
    .expect("Should have been able to read the file")
}

pub fn delivers_presents(input: &str) -> i32 {
    const NORTH: u8 = 94; // ^
    const SOUTH: u8 = 118; // v
    const EAST: u8 = 60; // <
    const WEST: u8 = 62; // >

    let mut sum: i32 = 1;
    let mut last_y = 0;
    let mut last_x = 0;

    let mut points: HashMap<i32, Vec<i32>> = HashMap::new();
    points.insert(last_y, vec![last_x]);

    for &item in input.as_bytes().iter() {
        if item == NORTH {
            last_y += 1;
            y_func(&mut points, last_y, last_x, &mut sum);
        } else if item == SOUTH {
            last_y -= 1;
            y_func(&mut points, last_y, last_x, &mut sum);
        } else if item == EAST {
            last_x -= 1;
            x_func(&mut points, last_y, last_x, &mut sum);
        } else if item == WEST {
            last_x += 1;
            x_func(&mut points, last_y, last_x, &mut sum);
        }
    }
    sum
}

fn x_func(points: &mut HashMap<i32, Vec<i32>>, last_y: i32, last_x: i32, sum: &mut i32) {
    if let Some(value) = points.get_mut(&last_y) {
        if !value.contains(&last_x) {
            value.push(last_x);
            *sum += 1;
        }
    };
}

fn y_func(points: &mut HashMap<i32, Vec<i32>>, last_y: i32, last_x: i32, sum: &mut i32) {
    if !points.contains_key(&last_y) {
        points.insert(last_y, vec![last_x]);
        *sum += 1;
    } else if let Some(value) = points.get_mut(&last_y) {
        if !value.contains(&last_x) {
            value.push(last_x);
            *sum += 1;
        }
    }
}

pub fn delivers_presents_hash_set(input: &str) -> i32 {
    const NORTH: u8 = 94; // ^
    const SOUTH: u8 = 118; // v
    const EAST: u8 = 60; // <
    const WEST: u8 = 62; // >

    let mut sum: i32 = 1;
    let mut last_y = 0;
    let mut last_x = 0;

    let mut points: HashMap<i32, HashSet<i32>> = HashMap::new();
    points.insert(last_y, HashSet::from([last_x]));

    for &item in input.as_bytes().iter() {
        if item == NORTH {
            last_y += 1;
            y_func_hash_set(&mut points, last_y, last_x, &mut sum);
        } else if item == SOUTH {
            last_y -= 1;
            y_func_hash_set(&mut points, last_y, last_x, &mut sum);
        } else if item == EAST {
            last_x -= 1;
            x_func_hash_set(&mut points, last_y, last_x, &mut sum);
        } else if item == WEST {
            last_x += 1;
            x_func_hash_set(&mut points, last_y, last_x, &mut sum);
        }
    }
    sum
}

fn x_func_hash_set(
    points: &mut HashMap<i32, HashSet<i32>>,
    last_y: i32,
    last_x: i32,
    sum: &mut i32,
) {
    if let Some(value) = points.get_mut(&last_y) {
        if !value.contains(&last_x) {
            value.insert(last_x);
            *sum += 1;
        }
    };
}

fn y_func_hash_set(
    points: &mut HashMap<i32, HashSet<i32>>,
    last_y: i32,
    last_x: i32,
    sum: &mut i32,
) {
    if !points.contains_key(&last_y) {
        points.insert(last_y, HashSet::from([last_x]));
        *sum += 1;
    } else if let Some(value) = points.get_mut(&last_y) {
        if !value.contains(&last_x) {
            value.insert(last_x);
            *sum += 1;
        }
    }
}

pub fn delivers_presents_robot(input: &str) -> i32 {
    const NORTH: u8 = 94; // ^
    const SOUTH: u8 = 118; // v
    const EAST: u8 = 60; // <
    const WEST: u8 = 62; // >

    let mut sum: i32 = 1;
    let mut last_y = 0;
    let mut last_x = 0;
    let mut last_y_robot = 0;
    let mut last_x_robot = 0;

    let mut points: HashMap<i32, Vec<i32>> = HashMap::new();
    points.insert(last_y, vec![last_x]);

    for (i, &item) in input.as_bytes().iter().enumerate() {
        if i % 2 == 0 {
            if item == NORTH {
                last_y += 1;
                y_func(&mut points, last_y, last_x, &mut sum);
            } else if item == SOUTH {
                last_y -= 1;
                y_func(&mut points, last_y, last_x, &mut sum);
            } else if item == EAST {
                last_x -= 1;
                x_func(&mut points, last_y, last_x, &mut sum);
            } else if item == WEST {
                last_x += 1;
                x_func(&mut points, last_y, last_x, &mut sum);
            }
        } else if item == NORTH {
            last_y_robot += 1;
            y_func(&mut points, last_y_robot, last_x_robot, &mut sum);
        } else if item == SOUTH {
            last_y_robot -= 1;
            y_func(&mut points, last_y_robot, last_x_robot, &mut sum);
        } else if item == EAST {
            last_x_robot -= 1;
            x_func(&mut points, last_y_robot, last_x_robot, &mut sum);
        } else if item == WEST {
            last_x_robot += 1;
            x_func(&mut points, last_y_robot, last_x_robot, &mut sum);
        }
    }
    sum
}

#[test]
fn test() {
    let input = read_input_three();
    //let nr_presents = delivers_presents_robot("v>v<vvv<<vv^v<v>vv>v<<<^^^^^<<^<vv>^>v^>^>^>^>^>"); // 42
    let nr_presents = delivers_presents(&input);
    assert_eq!(nr_presents, 2572);
    let nr_presents = delivers_presents_robot(&input);
    assert_eq!(nr_presents, 2631);
    println!("{nr_presents}");
}
