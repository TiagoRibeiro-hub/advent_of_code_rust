use std::{io, fs::File};

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    super::read_lines(super::get_current_dir_files() + "/all_in_a_single_night.txt")
}


fn shortest_route() {

}





#[test]
fn test() {
    let res = shortest_route();
    //assert_eq!(res, );
    println!("res- {:?}", res); 
}