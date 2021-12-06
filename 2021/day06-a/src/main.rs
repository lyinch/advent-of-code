use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Fish {
    age: u8
}

fn main() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let mut lines = br.lines();
    let mut input: Vec<Fish> = lines.next().unwrap().unwrap().split(',').map(|x| Fish{age: x.parse::<u8>().unwrap() }).collect();

    for _ in 0..80{
        let total_fish = input.len();
        for fish in 0..total_fish {
            if input[fish].age == 0 {
                input.push(Fish {age: 8});
                input[fish].age = 6;
            } else {
                input[fish].age -= 1;
            }
        }
    }

    println!("{}", input.len());
}
