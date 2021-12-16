use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::SystemTime;

fn main() {
    let now = SystemTime::now();
    let br = BufReader::new(File::open("input.txt").unwrap());
    let mut lines = br.lines();

    let mut word: Vec<char> = lines.next().unwrap().unwrap().chars().collect();


    lines.next(); // empty line

    let mut data = Vec::new();
    for line in lines {
        let l = line.unwrap();
        let tmp: Vec<String> = l.split(" -> ").map(|x| x.to_owned()).collect();
        data.push((tmp[0].clone(), tmp[1].clone()));
    }


    let mut polymer: HashMap<String, i128> = HashMap::new();

    for w in word.windows(2) {
        let t: String = w.iter().collect();        
        *polymer.entry(t).or_insert(0) += 1;
    }

    let mut tmp_polymer = HashMap::new();
    for _ in 0..40 {
        for (k, c) in polymer.iter() {
            for (pat, res) in &data {
                if pat == k{
                    let tmp_ = pat.clone();
                    let mut tmp = tmp_.chars();
                    let first = format!("{}{}", tmp.next().unwrap() ,res);
                    let second = format!("{}{}", res, tmp.next().unwrap() );
                    *tmp_polymer.entry(first.clone()).or_insert(0) += c;
                    *tmp_polymer.entry(second.clone()).or_insert(0) += c;
                    break;
                }
            }
        }
        polymer = tmp_polymer.clone();
        tmp_polymer = HashMap::new();
    }

    let mut counter = HashMap::new();
    for (k, v) in polymer.iter() {
        let tmp: Vec<char> = k.chars().collect();
        *counter.entry(tmp[0]).or_insert(0) += v;
        *counter.entry(tmp[1]).or_insert(0) += v;
    }

    let mut max = (' ', 0);
    let mut min = (' ', 99999999999999999999999999999999);

    for (k, v) in counter.iter() {
        if v > &max.1 {
            max = (*k, *v);
        } else if v < &min.1 {
            min = (*k, *v);
        }
    }

    println!("{}", (max.1 - min.1)/2);
    let end = SystemTime::now();
    println!("{:?}", end.duration_since(now).unwrap());
}