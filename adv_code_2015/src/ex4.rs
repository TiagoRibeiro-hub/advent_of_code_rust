// https://adventofcode.com/2015/day/4
use crypto::md5::Md5;
use crypto::digest::Digest;

pub fn first_five_loop() -> u64 {
    let mut res: u64 = 1;
    for i in 1..std::u64::MAX {
        let output = md5::compute("iwrupvqb".to_string() + &i.to_string()).0;
        let first_five = output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32;
        if first_five == 0 {
            res = i;
            break;
        }
    }
    res
}


pub fn first_five() -> u64 {
    let mut hasher = Md5::new();

    let key = "iwrupvqb".as_bytes();
    let mut res = 0;
    for i in 1..std::u64::MAX {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());
        
        let mut output = [0; 16]; // An MD5 is 16 bytes
        hasher.result(&mut output);

        // * bytes [0, 0] become "00 00"
        // * The byte [1] is represented by 01 in hex.
        // * To split this byte to its halves, we can use bitshifting like this
        // * 0x01 >> 4 gives us 0.
        let first_five = output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32;
        if first_five == 0 {
            res = i;
            break;
        }
        hasher.reset();
    }

    res
}

pub fn first_six() -> u64 {
    let mut hasher = Md5::new();

    let key = "iwrupvqb".as_bytes();
    let mut res = 0;
    for i in 1..std::u64::MAX {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());
        
        let mut output = [0; 16]; // An MD5 is 16 bytes
        hasher.result(&mut output);

        let first_five = output[0] as i32 + output[1] as i32 + output[2] as i32;
        if first_five == 0 {
            res = i;
            break;
        }
        hasher.reset();
    }

    res
}

#[test]
fn test() {
    let res = first_five(); 
    assert_eq!(res, 346386);
    let res = first_six(); 
    assert_eq!(res, 9958218);
    println!("{:?}", res);
}
