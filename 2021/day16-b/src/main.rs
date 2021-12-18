use std::fs::File;
use std::io::{BufRead, BufReader};

fn bin_to_dec(slice: &[i32]) -> i64 {
    let mut res = 0;
    for (pos, bit) in slice.iter().rev().enumerate() {
        res += 2i64.pow(pos as u32) * (*bit as i64);
    }
    res
}

fn parse_packet(data: &[i32], packet_start: usize) -> (i64, usize) {
    let mut packet_start = packet_start;
    let mut length_of_bits: Option<i32> = None;
    let mut num_of_subpackets = None;

    let v = bin_to_dec(&data[packet_start..packet_start + 3]);
    let t = bin_to_dec(&data[packet_start + 3..packet_start + 6]);
    packet_start += 6;
    let mut index = packet_start;

    let mut val: i64 = 0;
    if t == 1 {
        val = 1;
    }

    if t == 4 {
        // literal
        let mut num = Vec::new();
        loop {
            // loop until we encounter a chunk which starts with a 0
            let leading_bit = data[index];
            let n = &mut data[index + 1..index + 5].to_vec().clone();
            num.append(&mut n[0..4].to_vec().clone());
            index += 5;
            if leading_bit == 0 {
                break;
            }
        }
        val = bin_to_dec(num.as_slice()) as i64;
        packet_start = index;
    } else {
        let length_type = data[packet_start];
        packet_start += 1;
        if length_type == 0 {
            let total_length = bin_to_dec(&data[packet_start..packet_start + 15]);
            length_of_bits = Some(total_length as i32);
            packet_start += 15;
        } else {
            let total_length = bin_to_dec(&data[packet_start..packet_start + 11]);
            num_of_subpackets = Some(total_length);
            packet_start += 11;
        }
    }

    let mut subpacket_values = Vec::new();
    if let Some(bits) = length_of_bits {
        let old_start = packet_start;
        while packet_start - old_start < bits as usize {
            let (val_, packet_start_) = parse_packet(data, packet_start);
            subpacket_values.push(val_);
            packet_start = packet_start_;
        }
    } else if let Some(num) = num_of_subpackets {
        for _ in 0..num {
            let (val_, packet_start_) = parse_packet(data, packet_start);
            subpacket_values.push(val_);
            packet_start = packet_start_;
        }
    }

    if t == 0 {
        val += subpacket_values.iter().sum::<i64>();
    } else if t == 1 {
        val *= subpacket_values.iter().product::<i64>();
    } else if t == 2 {
        val = *subpacket_values.iter().min().unwrap();
    } else if t == 3 {
        val = *subpacket_values.iter().max().unwrap();
    } else if t == 5 {
        val = 0;
        if subpacket_values[0] > subpacket_values[1] {
            val = 1;
        }
    } else if t == 6 {
        val = 0;
        if subpacket_values[0] < subpacket_values[1] {
            val = 1;
        }
    } else if t == 7 {
        val = 0;
        if subpacket_values[0] == subpacket_values[1] {
            val = 1;
        }
        
    }

    (val, packet_start)
}

fn main() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let lines = br.lines();
    let mut data = Vec::new();
    for line in lines {
        let l = line.unwrap();
        for hex in l.chars() {
            let mut bin = match hex {
                '0' => {
                    vec![0, 0, 0, 0]
                }
                '1' => {
                    vec![0, 0, 0, 1]
                }
                '2' => {
                    vec![0, 0, 1, 0]
                }
                '3' => {
                    vec![0, 0, 1, 1]
                }
                '4' => {
                    vec![0, 1, 0, 0]
                }
                '5' => {
                    vec![0, 1, 0, 1]
                }
                '6' => {
                    vec![0, 1, 1, 0]
                }
                '7' => {
                    vec![0, 1, 1, 1]
                }
                '8' => {
                    vec![1, 0, 0, 0]
                }
                '9' => {
                    vec![1, 0, 0, 1]
                }
                'A' => {
                    vec![1, 0, 1, 0]
                }
                'B' => {
                    vec![1, 0, 1, 1]
                }
                'C' => {
                    vec![1, 1, 0, 0]
                }
                'D' => {
                    vec![1, 1, 0, 1]
                }
                'E' => {
                    vec![1, 1, 1, 0]
                }
                'F' => {
                    vec![1, 1, 1, 1]
                }
                _ => {
                    vec![0, 0, 0, 0]
                }
            };
            data.append(&mut bin);
        }
    }

    let out = parse_packet(data.as_slice(), 0);
    println!("{}", out.0);
}
