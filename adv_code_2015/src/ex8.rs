use std::{io, fs::File};



fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    //super::read_lines(super::get_current_dir_files() + "/matchsticks.txt")
    super::read_lines("D:\\Projects\\advent_of_code_rust\\files\\matchsticks.txt")
}

pub fn matchsticks() -> i32 {
    let mut total_str_code = 0;
    let mut total_str = 0;

    if let Ok(lines) = read_lines() {
        for mut line in lines.flatten() {
            
            let count_str_code: i32 = line.len() as i32;

            // remove first and last char "
            line.remove(0);
            line.remove(line.len() - 1);
            
            let mut count_str: i32 = 0;

            let line_as_bytes = line.as_bytes();
            let mut jump = 0;

            for (i, &c) in line_as_bytes.iter().enumerate() {
                if jump == 0 {
                        if c == b'\\' {
                            if line_as_bytes[i+1] == b'\\' || line_as_bytes[i+1] == b'"'{
                                count_str = count_str + 1;
                                jump = 1;
                            }
                            else {
                                // \x41 - hexadecimal 
                                count_str = count_str + 1;
                                jump = 3;
                            }
                        }
                        else {
                            count_str = count_str + 1;
                        }
                }
                else {
                    jump = jump - 1;
                }
            }

            total_str_code = total_str_code + count_str_code;
            total_str = total_str + count_str;
        }
    }

    total_str_code - total_str
}

pub fn matchsticks_2() -> i32 {
    let mut total_str_code = 0;
    let mut total_str = 0;

    if let Ok(lines) = read_lines() {
        for mut line in lines.flatten() {
            
            let count_str_code: i32 = line.len() as i32;
            line.remove(line.len() - 1);

            let mut count_str: i32 = 0;

            let mut line_as_bytes = line.as_bytes().iter();
            // remove first "
            line_as_bytes.next();
            
            let mut i = 0;
            while i < line.len() {
                if let Some(c) = line_as_bytes.next() {
                    if *c == b'\\' {
                        if let Some(c_n) = line_as_bytes.next() {
                            if *c_n == b'\\' || *c_n == b'"'{
                                count_str = count_str + 1;
                            }
                            else {
                                // \x41 - hexadecimal 
                                count_str = count_str + 1;
                                let mut count = 2;
                                loop {
                                    line_as_bytes.next();
                                    count = count - 1;
                                    if count == 0 {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    else {
                        count_str = count_str + 1;
                    }
                }
                i = i + 1;
            }

            total_str_code = total_str_code + count_str_code;
            total_str = total_str + count_str;
        }
    }

    total_str_code - total_str
}

pub fn matchsticks_part_2() -> i32 {
    let mut total_str_original = 0;
    let mut total_str = 0;

    if let Ok(lines) = read_lines() {
        for mut line in lines.flatten() {
            
            let count_str_original: i32 = line.len() as i32;
            line.remove(line.len() - 1);

            let mut count_str: i32 = 6;

            let mut line_as_bytes = line.as_bytes().iter();
            // remove first "
            line_as_bytes.next();
            
            let mut i = 0;
            while i < line.len() {
                if let Some(c) = line_as_bytes.next() {
                    if *c == b'\\' {
                        if let Some(c_n) = line_as_bytes.next() {
                            if *c_n == b'\\' || *c_n == b'"' {
                                count_str = count_str + 4;
                            }
                            else {
                                // \x41 - hexadecimal 
                                count_str = count_str + 5;
                                let mut count = 2;
                                loop {
                                    line_as_bytes.next();
                                    count = count - 1;
                                    if count == 0 {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    else {
                        count_str = count_str + 1;
                    }
                }
                i = i + 1;
            }

            total_str_original = total_str_original + count_str_original;
            total_str = total_str + count_str;
        }
    }

    total_str - total_str_original
}


#[test]
fn test() {
    // let res = matchsticks_2();
    // assert_eq!(res, 1342);
    let res = matchsticks_part_2();
    //assert_eq!(res, 1342);
    println!("res- {:?}", res); // > 1332
}