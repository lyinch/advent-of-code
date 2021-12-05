use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp;

fn main() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let lines = br.lines();
    let mut segments: Vec<((i32,i32), (i32,i32))> = Vec::new();
    let mut field = vec![vec![0;1000]; 1000];

    for line in lines {
        let l = line.unwrap();
        let parts: Vec<&str> = l.split(' ').collect();
        let start: Vec<i32> = parts[0].split(',').map(|x| x.parse::<i32>().unwrap()).collect();
        let end: Vec<i32> = parts[2].split(',').map(|x| x.parse::<i32>().unwrap()).collect();
        segments.push(((start[0],start[1]),(end[0],end[1])));
    }

    for segment in segments {

        if segment.0.0 == segment.1.0 {
            // x is identical
            let start = cmp::min(segment.0.1, segment.1.1);
            let end= cmp::max(segment.0.1, segment.1.1);

            for i in start..=end {
                field[i as usize][segment.0.0 as usize] += 1;
            }
        } else if segment.0.1 == segment.1.1 {
            // y is identical
            let start = cmp::min(segment.0.0, segment.1.0);
            let end= cmp::max(segment.0.0, segment.1.0);

            for i in start..=end {
                field[segment.0.1 as usize][i as usize] += 1;
            } 
        } else {
            let mut x_change = 1;
            let mut y_change = 1;

            if segment.0.0 > segment.1.0 {
                x_change = -1;
            }
            if segment.0.1 > segment.1.1 {
                y_change = -1;
            }
            
            let start = cmp::min(segment.0.0, segment.1.0);
            let end= cmp::max(segment.0.0, segment.1.0);
            let mut starting_coord = segment.0;
            for _ in start..=end {
                field[starting_coord.1 as usize][starting_coord.0 as usize] += 1;
                starting_coord.0 += x_change;
                starting_coord.1 += y_change;
            } 
        }
    }

    

    let mut acc = 0;
    for row in field {
        //println!("{:?}", row);
        for entry in row {
            if entry >= 2 {
                acc += 1;
            }
        }
    }

    println!("{}", acc);
}
