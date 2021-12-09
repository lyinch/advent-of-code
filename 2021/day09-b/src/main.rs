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
    let mut low_points: Vec<(usize, usize)> = Vec::new(); // list of low points
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if y == 0 && x == 0 {
                let n = input[y][x];
                if n < input[y + 1][x] && n < input[y][x + 1] {
                    low_points.push((x,y));
                }
            } else if y == 0 && x == input[0].len() - 1 {
                let n = input[y][x];
                if n < input[y + 1][x] && n < input[y][x - 1] {
                    low_points.push((x,y));
                }
            } else if x == 0 && y == input.len()-1 {
                let n = input[y][x];
                if n < input[y - 1][x] && n < input[y][x + 1] {
                    low_points.push((x,y));
                }
            } else if y == input.len() - 1 && x == input[0].len() - 1 {
                let n = input[y][x];
                if n < input[y][x - 1] && n < input[y - 1][x] {
                    low_points.push((x,y));
                }
            }
            else if y == 0 {
                let n = input[y][x];
                if n < input[y][x - 1] && n < input[y + 1][x] && n < input[y][x + 1] {
                    low_points.push((x,y));
                }
            } 
            else if x == 0 {
                let n = input[y][x];
                if n < input[y - 1][x] && n < input[y + 1][x] && n < input[y][x + 1] {
                    low_points.push((x,y));
                }
            }  else if y == input.len() - 1 {
                let n = input[y][x];
                if n < input[y][x - 1] && n < input[y - 1][x] && n < input[y][x + 1] {
                    low_points.push((x,y));
                }
            } else if x == input[0].len() - 1 {
                let n = input[y][x];
                if n < input[y][x - 1] && n < input[y - 1][x] && n < input[y + 1][x] {
                    low_points.push((x,y));
                }
            } else {
                let n = input[y][x];
                if n < input[y][x - 1]
                    && n < input[y - 1][x]
                    && n < input[y + 1][x]
                    && n < input[y][x + 1]
                {
                    low_points.push((x,y));
                }
            }
        }
    }
    
    let mut largest_basins = (0,0,0);
    for (x_start,y_start) in low_points {
        let mut to_visit = vec![(x_start,y_start)];
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut basin_size = 0;
        while let Some((x, y)) = to_visit.pop() {
            if visited.contains(&(x,y)) {
                continue;
            }
            visited.insert((x,y));
            let n = input[y][x];
            if n == 9 {
                continue;
            }

            basin_size += 1;

            if y == 0 && x == 0 {
                if n < input[y][x + 1] {
                    to_visit.push((x+1,y));
                }
                if n  < input[y + 1][x] {
                    to_visit.push((x,y+1));
                }
            } else if y == 0 && x == input[0].len() - 1 {
                if n < input[y][x - 1] {
                    to_visit.push((x-1,y));
                }
                if n < input[y + 1][x] {
                    to_visit.push((x,y+1));
                }
            } else if x == 0 && y == input.len()-1 {
                if n < input[y][x + 1] {
                    to_visit.push((x+1,y));
                }
                if n < input[y - 1][x]  {
                    to_visit.push((x,y-1));
                }
            } else if y == input.len() - 1 && x == input[0].len() - 1 {
                if n < input[y - 1][x] {
                    to_visit.push((x,y-1));
                }
                if n < input[y][x - 1]  {
                    to_visit.push((x-1,y));
                }
            }
            else if y == 0 {
                if n < input[y + 1][x] {
                    to_visit.push((x,y+1));
                }
                if n < input[y][x + 1] {
                    to_visit.push((x+1,y));
                }
                if n < input[y][x - 1] {
                    to_visit.push((x-1,y));
                }
            } 
            else if x == 0 {
                if n < input[y + 1][x] {
                    to_visit.push((x,y+1));
                }
                if n < input[y][x + 1] {
                    to_visit.push((x+1,y));
                } 
                if n < input[y - 1][x] {
                    to_visit.push((x,y-1));
                }
            }  else if y == input.len() - 1 {
                if n < input[y - 1][x] {
                    to_visit.push((x,y-1));
                }
                if n < input[y][x + 1] {
                    to_visit.push((x+1,y));
                }
                if n < input[y][x - 1] {
                    to_visit.push((x-1,y));
                }
            } else if x == input[0].len() - 1 {
                if n < input[y - 1][x] {
                    to_visit.push((x,y-1));
                }
                if n < input[y + 1][x] {
                    to_visit.push((x,y+1));
                }
                if n < input[y][x - 1] {
                    to_visit.push((x-1,y));
                }
            } else {
                if n < input[y - 1][x] {
                    to_visit.push((x,y-1));
                }
                if n < input[y + 1][x] {
                    to_visit.push((x,y+1));
                }
                if  n < input[y][x + 1] {
                    to_visit.push((x+1,y));
                }
                if n < input[y][x - 1]
                {
                    to_visit.push((x-1,y));
                }
            }
        }

        if basin_size > largest_basins.0 {
            largest_basins.0 = basin_size;
        } else if basin_size > largest_basins.1 {
            largest_basins.1 = basin_size;
        } else if basin_size > largest_basins.2 {
            largest_basins.2 = basin_size;
        }
    }

    println!("{}", largest_basins.0*largest_basins.1*largest_basins.2);
}
