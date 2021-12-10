use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let lines = br.lines();

    let mut scores: Vec<i64> = Vec::new();
    for line in lines {
        let l = line.unwrap();
        let mut stack: Vec<char> = Vec::new();

        let mut corrupted = false;
        for symbol in l.chars() {
            match symbol {
                '[' => {
                    stack.push(symbol);
                }
                '{' => {
                    stack.push(symbol);
                }
                '(' => {
                    stack.push(symbol);
                }
                '<' => {
                    stack.push(symbol);
                }
                ')' => {
                    if stack.pop().unwrap() != '(' {
                        corrupted = true;
                    }
                }
                ']' => {
                    if stack.pop().unwrap() != '[' {
                        corrupted = true;
                    }
                }
                '}' => {
                    if stack.pop().unwrap() != '{' {
                        corrupted = true;
                    }
                }
                '>' => {
                    if stack.pop().unwrap() != '<' {
                        corrupted = true;
                    }
                }
                _ => {}
            }
        }
        let mut local_score: i64 = 0;
        if !corrupted {
            stack.reverse();
            for symbol in stack {
                local_score *= 5;
                match symbol {
                    '(' => {
                        local_score += 1;
                    },
                    '[' => {
                        local_score += 2;
                    },
                    '{' => {
                        local_score += 3;
                    },
                    '<' => {
                        local_score += 4;
                    },
                    _ => {println!("error");}
                }
            }
            scores.push(local_score);
        }
    }
    scores.sort();
    println!("{:?}", scores[scores.len()/2]);
}
