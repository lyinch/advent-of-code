use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let input: Vec<i64> = br.lines().map(|line| line.unwrap().parse().unwrap()).collect();
    let answer = input.windows(2).map(|e| e[0] < e[1]).fold(0, |mut a, c| { if c {a += 1} a});
    println!("{}", answer);
}
