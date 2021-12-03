use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let input = br.lines();
    let mut total_lines = 0; 
    let mut counter = vec![0,0,0,0,0,0,0,0,0,0,0,0];

    for line in input {
        total_lines += 1;
        let l = line.unwrap();
        for (pos,entry) in l.chars().enumerate() {
            match entry {
                '0' => {},
                '1' => {counter[pos] += 1;},
                _ => {}
            }
        }
    }

    let mut gamma = Vec::new();
    let mut epsilon= Vec::new();
    for n in counter {
        if n > total_lines/2 {
           gamma.push(1);
           epsilon.push(0);
        } else {
            gamma.push(0);
            epsilon.push(1);
        }
    }
    let mut gamma_dec = 0;
    let mut epsilon_dec = 0;
    for (position, n) in gamma.iter().enumerate() {
       gamma_dec += n*(2_i32.pow(11-(position as u32)));
    }

    for (position, n) in epsilon.iter().enumerate() {
        epsilon_dec += n*(2_i32.pow(11-(position as u32)));
     }
     
    println!("{}", gamma_dec*epsilon_dec);
}
