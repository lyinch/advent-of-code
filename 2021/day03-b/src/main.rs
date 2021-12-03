use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let mut input: Vec<String> = br.lines().collect::<Result<_, _>>().unwrap();
    let mut input_c: Vec<String> = input.clone();

    let num_of_digits = 12;

    for i in 0..num_of_digits {
        let mut current_count = 0;
        let mut total_lines = 0;
        for line in &mut input {
            total_lines += 1;
            if line.chars().nth(i).unwrap() == '1' {
                current_count += 1;
            }
        }

        let bit_to_keep = {
            if current_count*2 >= total_lines {
                '1'
            } else {
                '0'
            }
        };
        
        input = input.iter().filter(|x| x.chars().nth(i).unwrap() == bit_to_keep).map(|x| x.to_owned()).collect();

        if input.len() == 1 {
            break
        }
    }

    for i in 0..num_of_digits {
        let mut current_count = 0;
        let mut total_lines = 0;
        for line in &mut input_c {
            total_lines += 1;
            if line.chars().nth(i).unwrap() == '1' {
                current_count += 1;
            }
        }

        let bit_to_keep = {
            if current_count*2 < total_lines {
                '1'
            } else {
                '0'
            }
        };
        
        input_c = input_c.iter().filter(|x| x.chars().nth(i).unwrap() == bit_to_keep).map(|x| x.to_owned()).collect();

        if input_c.len() == 1 {
            break
        }
    }

    let oxy: Vec<i32> = input.get(0).unwrap().chars().map(|x| x.to_digit(10).unwrap() as i32).collect();
    let co: Vec<i32> = input_c.get(0).unwrap().chars().map(|x| x.to_digit(10).unwrap() as i32).collect();

    let mut oxy_dec = 0;
    let mut co_dec= 0;
    for (position, n) in oxy.iter().enumerate() {
       oxy_dec += n*(2_i32.pow(((num_of_digits-1)-position) as u32));
    }

    for (position, n) in co.iter().enumerate() {
        co_dec += n*(2_i32.pow(((num_of_digits-1)-position) as u32));
     }

     println!("{}", oxy_dec*co_dec);
     
     
}
