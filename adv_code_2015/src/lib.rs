pub mod ex1;
pub mod ex2;
pub mod ex3;
pub mod ex4;
pub mod ex5;

use std::{io::{self, BufRead}, fs::File, path::Path};


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_current_dir_files() -> String {
    let dir = std::env::current_dir().expect("Current dir not found");
    dir.to_str().unwrap().to_owned() + "/../files"
}