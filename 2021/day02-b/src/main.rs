use std::io::{BufReader, BufRead};
use std::fs::File;

struct SubPosition{
    x: i32,
    y: i32,
    aim: i32,
}

fn main() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let input = br.lines();
    let mut position = SubPosition{x:0, y:0, aim:0};
    for line in input {
        let l = line.unwrap();
        let data: Vec<&str> = l.split_whitespace().collect();
        match data[..] {
            ["forward", n] => {
                position.x += n.parse::<i32>().unwrap();
                position.y += position.aim * n.parse::<i32>().unwrap();
            },
            ["up", n] => {
                position.aim -= n.parse::<i32>().unwrap();
            },
            ["down", n] => {
                position.aim +=  n.parse::<i32>().unwrap();
            },
            _ => {}
        }
    }
   println!("{}", position.x*position.y);
}
