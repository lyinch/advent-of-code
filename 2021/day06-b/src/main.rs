use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let mut lines = br.lines();
    let mut input: Vec<u8> = lines.next().unwrap().unwrap().split(',').map(|x|  x.parse::<u8>().unwrap()).collect();
    let mut fish_state = [0i64; 9];
    for fish in input {
        fish_state[fish as usize] += 1;
    }
    
    for _ in 0..256{
        let mut buffer = [0; 9];
        for i in 0..9 {
            if i == 0 {
                buffer[8] = fish_state[0];
                buffer[6] = fish_state[0];
            } else {
                buffer[i-1] += fish_state[i];
            }
        }  
        fish_state = buffer;

    }

    let result: i64 = fish_state.iter().sum(); 
    println!("{}", result);
}
