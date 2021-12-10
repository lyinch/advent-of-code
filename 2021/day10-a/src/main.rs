use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let lines = br.lines();

    let mut res = 0;
    for line in lines {
        let l = line.unwrap();
        let mut stack: Vec<char> = Vec::new();

        for symbol in l.chars() {
           match symbol {
               '[' => {
                stack.push(symbol);
               },
               '{' => {
                stack.push(symbol);
               },
               '(' => {
                stack.push(symbol);
               },
               '<' => {
                stack.push(symbol);
               },
                ')' => {
                    if stack.pop().unwrap() != '(' {
                        res += 3;
                        break;
                    }
                },
                ']' => {
                    if stack.pop().unwrap() != '[' {
                        res += 57;
                        break;
                    }
                },
                '}' => {
                    if stack.pop().unwrap() != '{' {
                        res += 1197;
                        break;
                    }
                } ,
                '>' => {
                    if stack.pop().unwrap() != '<' {
                        res += 25137;
                        break;
                    }
                }
                _ => {}
           } 
        }
    }
    println!("{}", res);
}
