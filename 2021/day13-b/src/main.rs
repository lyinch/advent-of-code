use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let lines = br.lines();

    let mut data = vec![vec![false; 2000]; 2000];
    let mut folds = Vec::new();

    let mut max_x = 0;
    let mut max_y = 0;
    for line in lines {
        let l = line.unwrap();
        if !l.is_empty() {
            if l.starts_with("fold") {
                let tmp = l.strip_prefix("fold along ").unwrap();
                let s: Vec<String> = tmp.split('=').map(|x| x.to_owned()).collect();
                folds.push((s[0].clone(), s[1].parse::<usize>().unwrap()));
            } else {
                let dot: Vec<usize> = l.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
                data[dot[1]][dot[0]] = true;
                max_x = max_x.max(dot[0]);
                max_y = max_y.max(dot[1]);
            }
        }
    }

    let mut buffer = data.clone();
    for current_fold in &folds {
        if current_fold.0 == "y" {
            for (y, row) in data.iter().enumerate() {
                if y > max_y {
                    continue;
                }
                for (x, e) in row.iter().enumerate() {
                    if x > max_x {
                        continue;
                    }
                    if y > current_fold.1 {
                        let new_y = current_fold.1 - (y - current_fold.1);
                        buffer[new_y][x] |= e;
                    }
                }
            }
            max_y = current_fold.1 - 1;
        } else {
            for (y, row) in data.iter().enumerate() {
                if y > max_y {
                    continue;
                }
                for (x, e) in row.iter().enumerate() {
                    if x > max_x {
                        continue;
                    }
                    if x > current_fold.1 {
                        let new_x = current_fold.1 - (x - current_fold.1);
                        buffer[y][new_x] |= e;
                    }
                }
            }
            max_x = current_fold.1 - 1;
        }
        data = buffer.clone();
    }

    for (y, row) in buffer.iter().enumerate() {
        if y > max_y {
            continue;
        }
        for (x, e) in row.iter().enumerate() {
            if x > max_x {
                continue;
            }
            if *e {
                print!("#");
            } else {
                print!(".");
            } 
        }
        println!();
    }
}
