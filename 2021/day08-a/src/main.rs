use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let lines = br.lines();
    let mut input = Vec::new();
    for line in lines {
        let l = line.unwrap().clone();
        let res: Vec<String> = l.split_whitespace().filter(|x| x != &"|").map(|x| x.to_owned()).collect(); //.filter(|x| x != &"|").collect();
        input.push(res);
    }

    let mut res = 0; 
    for entry in &input {
        for pat in &entry[10..=13] {
            match pat.len() {
                2 => { res += 1;},
                3 => { res += 1; }
                4 => { res += 1; },
                7 => { res += 1; },
                _ => {}
            }
        }
    }

    println!("{}", res); {
        
    }
}
