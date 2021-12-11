use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let lines = br.lines();
    
    let mut data = Vec::new();
    for line in lines {
        let l = line.unwrap();
        let p: Vec<(i32, bool)> = l.chars().map(|x| (x.to_digit(10).unwrap() as i32, false)).collect();
        data.push(p);
    }

    let mut first_flash = 0;

    for t in 0..999999{
        let mut flashing = Vec::new();
        for y in 0..data.len() {
            for x in 0..data[0].len() {
                data[y][x].0 += 1;
                if data[y][x].0 > 9 {
                    flashing.push((x,y));
                }
            }
        }
        let mut local_flashes = 0; 
        while let Some((x,y)) = flashing.pop() {
            if  !data[y][x].1 {
                data[y][x].1 = true;
                local_flashes += 1;
                let mut directions:Vec<(i32, i32)> = vec![(-1,-1), (-1,0), (1,0), (0,1), (0,-1), (1,-1), (1,1), (-1,1)];
                if y == 0 {
                    directions = directions.into_iter().filter(|(_, y)| y >= &0).collect();
                } 
                if x == 0 {
                    directions = directions.into_iter().filter(|(x, _)| x >= &0).collect();
                }
                if y == data.len()-1 {
                    directions = directions.into_iter().filter(|(_, y)| y <= &0).collect();
                }
                if x == data[0].len()-1 {
                    directions = directions.into_iter().filter(|(x, _)| x <= &0).collect();
                }

                for (dx, dy) in directions {
                    let ny = (y as i32 + dy) as usize;
                    let nx = (x as i32 + dx) as usize;
                    data[ny][nx].0 += 1; 
                    if data[ny][nx].0 > 9 {
                        flashing.push((nx,ny));
                    }
                }
            }
        }

        if local_flashes == 100 {
            first_flash = t;
            break;
        }

        for y in 0..data.len() {
            for x in 0..data[0].len() {
                if data[y][x].0 > 9 {
                    data[y][x].0 = 0;
                }
                data[y][x].1 = false;
            }
        }
    }

    println!("{}", first_flash+1);
}
