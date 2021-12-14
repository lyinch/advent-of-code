use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
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

    let mut out = "".to_string();

    for iteration in 0..10 {
        if iteration > 0 {
            word = out.clone().chars().collect::<Vec<char>>();
        }
        out = "".to_string();
        for l in word.windows(2) {
            let tmp:String = l.iter().collect();

            let mut found = false;
            for (pat, res) in &data {
                if *pat == tmp {
                    out = format!("{}{}{}", out, l[0], res);
                    found = true;
                    break;
                }
            }

            if !found {
                out = format!("{}{}", out, tmp);
            }    
        }
        out = format!("{}{}", out, word.last().unwrap());
    }

    let mut counter = HashMap::new();
    for c in out.chars() {
        let e = counter.entry(c).or_insert(0);
        *e += 1;
    }
    
    let mut max = (' ', 0);
    let mut min= (' ', 99999999);

    for (k,v) in counter.iter() {
        if v > &max.1 {
            max = (*k,*v);
        } else if v < &min.1 {
            min = (*k,*v);
        }
    }

    println!("{}", max.1-min.1);
}
