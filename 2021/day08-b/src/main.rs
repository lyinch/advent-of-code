use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::Chars;

fn main() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let lines = br.lines();
    let mut input = Vec::new();

    for line in lines {
        let l = line.unwrap().clone();
        let res: Vec<String> = l
            .split_whitespace()
            .filter(|x| x != &"|")
            .map(|x| x.to_owned())
            .collect(); //.filter(|x| x != &"|").collect();

        input.push(res);
    }

    let mut segments_to_digit: HashMap<String, &str> = HashMap::new();
    segments_to_digit.insert("abcefg".to_owned(), "0");
    segments_to_digit.insert("cf".to_owned(), "1");
    segments_to_digit.insert("acdeg".to_owned(), "2");
    segments_to_digit.insert("acdfg".to_owned(), "3");
    segments_to_digit.insert("bcdf".to_owned(), "4");
    segments_to_digit.insert("abdfg".to_owned(), "5");
    segments_to_digit.insert("abdefg".to_owned(), "6");
    segments_to_digit.insert("acf".to_owned(), "7");
    segments_to_digit.insert("abcdefg".to_owned(), "8");
    segments_to_digit.insert("abcdfg".to_owned(), "9");

    let mut result = 0;
    for entry in &input {
        let mut size_mapping: HashMap<i32, Vec<Vec<char>>> = HashMap::new();
        for v in entry {
            size_mapping
                .entry(v.len() as i32)
                .or_insert(Vec::new())
                .push(v.chars().collect());
        }
        let mut segment_mapping: HashMap<char, char> = HashMap::new();
        // find a
        let one = &size_mapping.get(&2).unwrap()[0];
        let seven = &size_mapping.get(&3).unwrap()[0];
        let diff: Vec<&char> = seven.iter().filter(|x| !one.contains(x)).collect();
        let a = diff[0];
        segment_mapping.insert(*a, 'a');
        
        // find out c, f
        let six_digits = &size_mapping.get(&6).unwrap();
        let one = &size_mapping.get(&2).unwrap()[0];

        let mut f = ' ';
        let mut f_c = Vec::new();
        for digit in six_digits.iter() {
            let segments = digit
                .iter()
                .filter(|x| one.contains(x))
                .collect::<Vec<&char>>();
            if segments.len() == 1 {
                f = *segments[0];
            } else {
                f_c = segments;
            }
        }
        let c = f_c.iter().filter(|x| **x != &f).next().unwrap();

        segment_mapping.insert(**c, 'c');
        segment_mapping.insert(f, 'f');

        // find g and e
        let six_digits = &size_mapping.get(&6).unwrap();
        let four = &size_mapping.get(&4).unwrap()[0];
        let mut g = ' ';
        let mut e_g = Vec::new();
        for digit in six_digits.iter() {
            let segments = digit
                .iter()
                .filter(|x| *x != a && !four.contains(x))
                .collect::<Vec<&char>>();
            if segments.len() == 1 {
                g = *segments[0];
            } else {
                e_g = segments;
            }
        }

        let e = e_g.iter().filter(|x| **x != &g).next().unwrap();

        segment_mapping.insert(g, 'g');
        segment_mapping.insert(**e, 'e');
        // find b, d
        let mut b = ' ';
        let mut b_d = Vec::new();
        for digit in six_digits.iter() {
            let segments = digit
                .iter()
                .filter(|x| *x != a && *x != &g && x != e && x != c && **x != f && four.contains(x))
                .collect::<Vec<&char>>();
            if segments.len() == 1 {
                b = *segments[0];
            } else {
                b_d = segments;
            }
        }

        let d = b_d.iter().filter(|x| **x != &b).next().unwrap();

        segment_mapping.insert(b, 'b');
        segment_mapping.insert(**d, 'd');
        
        let mut result_concat = "".to_string();
        for observation in &entry[10..=13] {
            let mut symbols: Vec<char> = observation 
                .chars()
                .map(|x| *segment_mapping.get(&x).unwrap())
                .collect::<Vec<char>>();
            symbols.sort();
            let segments_str: String = symbols.iter().collect();
            result_concat.push_str(segments_to_digit[&segments_str]);
        }

        result += result_concat.parse::<i32>().unwrap();
    }
    println!("{}", result);

}
