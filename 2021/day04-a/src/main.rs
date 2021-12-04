use std::fs::File;
use std::io::{BufRead, BufReader};

use std::fmt;

struct Board {
    board: Vec<Vec<(i32, bool)>>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        for row in 0..5 {
            for col in 0..5 {
                if self.board[row][col].1 {
                    write!(f, "*{:>2}, ", self.board[row][col].0.to_string())?;
                } else {
                    write!(f, "{:>2},", self.board[row][col].0.to_string())?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}




fn main() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let mut input = br.lines();
    let random_numbers = input.next().unwrap().unwrap();
    let random_numbers: Vec<i32> = random_numbers.split(',').map(|x| x.parse::<i32>().unwrap()).collect();

    input.next(); // remove first empty line
    let mut boards: Vec<Board> = vec![Board{board: Vec::new()}];

    let mut current_board = 0;
    for line in input {
        let l = line.unwrap();

        if !l.is_empty() {
            let r: Vec<(i32, bool)> = l.split(' ').filter(|x| !x.is_empty()).map(|x| (x.parse::<i32>().unwrap(), false) ).collect();
            boards[current_board].board.push(r);
        } else {
            boards.push(Board{board: Vec::new()});
            current_board += 1;
        }
    }

    let mut found = false;
    let mut result = 0;
    'outer: for num in random_numbers {
        for board in &mut boards {
            // mark numbers
            for row in &mut board.board {
                for entry in  row {
                    *entry = (entry.0, entry.1 || (entry.0 == num));
                }
                //row.iter_mut().map(|(n, state)| (n.clone(), *state || (n==&num) ));
            }

            // check for win
            let mut unseen_sum = 0;
            for row in 0..5 {
                let mut correct= 0;
                for col in 0..5 {
                    if board.board[row][col].1 {
                        correct += 1;
                    } else {
                        unseen_sum += board.board[row][col].0;
                    }
                }
                if correct == 5 {
                    found = true;
                }
            }

            if found {
                result = num * unseen_sum;
                break 'outer;
            }
  
            unseen_sum = 0;
            for col in 0..5 {
                let mut correct= 0;
                for row in 0..5 {
                    if board.board[row][col].1 {
                        correct += 1;
                    } else {
                        unseen_sum += board.board[row][col].0;
                    }
                }
                if correct == 5 {
                    found = true;
                }
            }

            if found {
                result = num * unseen_sum;
                break 'outer;
            }
            //println!("{}", board);
        }
    }

    println!("{}", result);


}
