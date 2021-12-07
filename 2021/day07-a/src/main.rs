use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let mut lines = br.lines();
    let input: Vec<i32> = lines.next().unwrap().unwrap().split(',').map(|x|  x.parse::<i32>().unwrap()).collect();

    let max = input.iter().max().unwrap();

    let mut grid = vec![vec![0;*max as usize]; input.len()];

    for crab in 0..input.len() {
        for position in 0..(*max as usize) {
            grid[crab][position] = (input[crab]-(position as i32)).abs();
        }
    }
   
    let mut best_pos = 9999999;
    for position in 0..(*max as usize) {
        let mut current_pos = 0;
        for crab in 0..grid.len() {
            current_pos += grid[crab][position];
        }
        best_pos = current_pos.min(best_pos);
    }


    println!("{}", best_pos);
    
}
