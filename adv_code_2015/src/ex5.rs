// https://adventofcode.com/2015/day/5
use std::{collections::HashMap, fs::File, io};

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    super::read_lines("/home/tiago/rust/projects/advent_of_code/advent_of_code/files/strings.txt")
}

pub fn nice_strings() -> u32 {
    let mut res = 0;
    if let Ok(lines) = read_lines() {
        for value in lines.flatten() {
            let mut last: u8 = b' ';
            let mut vowels_count: i8 = 0; // at least three vowels - aeiou
            let mut letter_twice = false; // It contains at least one letter that appears twice in a row, like xx, aa
            let mut contains = false; // does not contain the strings ab, cd, pq, or xy
            for &item in value.as_bytes().iter() {
                contains = [b"ab", b"cd", b"pq", b"xy"]
                    .iter()
                    .any(|f| **f == [last, item]);
                if contains {
                    break;
                }

                if vowels_count < 3 && [b'a', b'e', b'i', b'o', b'u'].contains(&item) {
                    vowels_count += 1;
                }

                if !letter_twice && item == last {
                    letter_twice = true;
                }
                last = item;
            }

            if !contains && vowels_count >= 3 && letter_twice {
                res += 1;
            }
        }
    }
    res
}

pub fn nice_strings_part2() -> u32 {
    let mut res = 0;
    if let Ok(lines) = read_lines() {
        for value in lines.flatten() {
            // It contains a pair of any two letters that appears at least twice in the string without overlapping
            // like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
            let mut letter_twice = false;
            // It contains at least one letter which repeats with exactly one letter between them,
            // like xyx, abcdefeghi (efe), or even aaa.
            let mut repeated_letter = false;

            let input = value.as_bytes();
            let len = input.len() - 1;
            let mut c: Vec<[u8; 2]> = Vec::new();

            for (i, item) in input.iter().enumerate() {
                if !repeated_letter && i > 1 {
                    if input[i - 2] == *item {
                        repeated_letter = true;
                    }
                }
                if len == i {
                    break;
                }
                if !letter_twice {
                    let a: [u8; 2] = [*item, input[i + 1]];
                    for (j, b) in input.windows(2).enumerate() {
                        if a == *b && i.abs_diff(j) > 1 {
                            if c.contains(&a) {
                                letter_twice = true;
                                break;
                            } else {
                                c.push(a);
                            }
                        }
                    }
                }

                if letter_twice && repeated_letter {
                    break;
                }
            }

            if letter_twice && repeated_letter {
                res += 1;
            }
        }
    }
    res
}

#[test]
fn test() {
    let res = nice_strings();
    assert_eq!(res, 236);
    let res = nice_strings_part2();
    assert_eq!(res, 51); 
    println!("res- {:?}", res);
}
