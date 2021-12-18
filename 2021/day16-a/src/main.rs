use std::fs::File;
use std::io::{BufRead, BufReader};


fn bin_to_dec(slice: &[i32]) -> i32{
    let mut res = 0;
    for (pos, bit) in slice.iter().rev().enumerate() {
        res += 2i32.pow(pos as u32)*bit;
    }
    res
}

fn parse_packet(data: &[i32], packet_start: usize, version: i32) -> (i32, usize) {
    let mut packet_start = packet_start;
    let mut length_of_bits: Option<i32>  = None;
    let mut num_of_subpackets = None;
    let mut version = version;

        let v = bin_to_dec(&data[packet_start..packet_start+3]);
        let t = bin_to_dec(&data[packet_start+3..packet_start+6]);
        packet_start += 6;
        let mut index = packet_start;

        if t == 4 {
            // literal
            let mut num = Vec::new();
            loop {
                // loop until we encounter a chunk which starts with a 0
                let leading_bit = data[index];
                let n = &mut data[index+1..index+5].to_vec().clone();
                num.append(&mut n[0..4].to_vec().clone());
                index += 5;
                if leading_bit == 0 {
                    break
                }
            }
            packet_start = index;
        } else {
            let length_type = data[packet_start];
            packet_start += 1;
            if length_type == 0 {
               let total_length = bin_to_dec(&data[packet_start..packet_start+15]);
               length_of_bits = Some(total_length);
               //println!("{}", total_length);
               //println!("{:?}", &data[packet_start..packet_start+15]);
               packet_start += 15;
            } else {
               let total_length = bin_to_dec(&data[packet_start..packet_start+11]);
               num_of_subpackets = Some(total_length);
               packet_start += 11;
            }
        }

        if let Some(bits) = length_of_bits {
            let old_start = packet_start;
            while packet_start-old_start < bits as usize{
               let (version_, packet_start_) = parse_packet(data, packet_start, version);
               version = version_;
               packet_start = packet_start_;
               //println!("{} --- {}", packet_start, packet_start-old_start);
            }
        } else if let Some(num) = num_of_subpackets {
            //println!("upsi");
           for _ in 0..num {
            let (version_, packet_start_)  = parse_packet(data, packet_start, version);
            version = version_;
            packet_start = packet_start_;
           }
        }
    (v+version, packet_start)
}

fn main() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let lines = br.lines();
    let mut data = Vec::new();
    for line in lines {
        let l = line.unwrap();
        for hex in l.chars() {
            let mut bin = match hex {
                '0' => {vec![0,0,0,0]},
                '1' => {vec![0,0,0,1]},
                '2' => {vec![0,0,1,0]},
                '3' => {vec![0,0,1,1]},
                '4' => {vec![0,1,0,0]},
                '5' => {vec![0,1,0,1]},
                '6' => {vec![0,1,1,0]},
                '7' => {vec![0,1,1,1]},
                '8' => {vec![1,0,0,0]},
                '9' => {vec![1,0,0,1]},
                'A' => {vec![1,0,1,0]},
                'B' => {vec![1,0,1,1]},
                'C' => {vec![1,1,0,0]},
                'D' => {vec![1,1,0,1]},
                'E' => {vec![1,1,1,0]},
                'F' => {vec![1,1,1,1]},
                _ => {vec![0,0,0,0]},
            };
            data.append(&mut bin);
        }
    }

    let out = parse_packet(data.as_slice(), 0, 0);
    println!("{}", out.0);
}
