use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let lines = br.lines();
    let mut input: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        let l = line.unwrap();
        let tmp: Vec<u32> = l
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .map(|x| x.to_digit(10).unwrap())
            .collect();
        input.push(tmp);
    }
    let mut sum = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if y == 0 && x == 0 {
                let n = input[y][x];
                if n < input[y + 1][x] && n < input[y][x + 1] {
                    sum += n+1;
                }
            } else if y == 0 && x == input[0].len() - 1 {
                let n = input[y][x];
                if n < input[y + 1][x] && n < input[y][x - 1] {
                     sum += n+1;
                }
            } else if x == 0 && y == input.len()-1 {
                let n = input[y][x];
                if n < input[y - 1][x] && n < input[y][x + 1] {
                     sum += n+1;
                }
            } else if y == input.len() - 1 && x == input[0].len() - 1 {
                let n = input[y][x];
                if n < input[y][x - 1] && n < input[y - 1][x] {
                     sum += n+1;
                }
            }
            else if y == 0 {
                let n = input[y][x];
                if n < input[y][x - 1] && n < input[y + 1][x] && n < input[y][x + 1] {
                     sum += n+1;
                }
            } 
            else if x == 0 {
                let n = input[y][x];
                if n < input[y - 1][x] && n < input[y + 1][x] && n < input[y][x + 1] {
                     sum += n+1;
                }
            }  else if y == input.len() - 1 {
                let n = input[y][x];
                if n < input[y][x - 1] && n < input[y - 1][x] && n < input[y][x + 1] {
                     sum += n+1;
                }
            } else if x == input[0].len() - 1 {
                let n = input[y][x];
                if n < input[y][x - 1] && n < input[y - 1][x] && n < input[y + 1][x] {
                     sum += n+1;
                }
            } else {
                let n = input[y][x];
                if n < input[y][x - 1]
                    && n < input[y - 1][x]
                    && n < input[y + 1][x]
                    && n < input[y][x + 1]
                {
                     sum += n+1;
                }
            }
        }
    }
    
    println!("{}", sum);
}
