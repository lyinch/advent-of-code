use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use petgraph::Graph;
use petgraph::algo::dijkstra;
use petgraph::dot::{Dot, Config};

fn main() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let lines = br.lines();

    let mut g: Graph<u32, u32, _> = Graph::new();
    let mut lookup = HashMap::new();
    let mut end_node = (0,0);

    let mut mini_board = Vec::new();
    let mut board: Vec<Vec<u32>> = Vec::new();
    for (i, line) in lines.into_iter().enumerate() {
        let l = line.unwrap();
        let cells: Vec<u32> = l.chars().map(|x| x.to_digit(10).unwrap()).collect();
        mini_board.push(cells);
    }

    for i in 0..5 {
        for j in 0..5 {
            // columns
            for (r, row) in mini_board.iter().enumerate() {
                if j == 0 {
                    board.push(Vec::new());
                }
                let mut local_row = Vec::new();
                for cell in row {
                    let mut new_cell = cell + i + j;
                    if new_cell > 9 {
                        new_cell -= 9;
                    }
                    local_row.push(new_cell);
                }
                board[r + (i as usize)*100].append(&mut local_row);
            }
        }
    }

    for (i, row) in board.iter().enumerate() {
        if i > end_node.0 {
            end_node.0 = i;
        }
        for (j, cell) in row.iter().enumerate() {
            if j > end_node.1 {
                end_node.1 = j;
            }
            let ni = g.add_node(*cell);
            lookup.insert((i,j), ni);
            if j != 0 {
                g.add_edge(*lookup.get(&(i,j-1)).unwrap(), ni, *cell);
                let w = *g.node_weight(*lookup.get(&(i,j-1)).unwrap()).unwrap();
                g.add_edge(ni, *lookup.get(&(i,j-1)).unwrap(), w);
            }
            if i != 0 {
                g.add_edge(*lookup.get(&(i-1,j)).unwrap(), ni, *cell);
                let w = *g.node_weight(*lookup.get(&(i-1,j)).unwrap()).unwrap();
                g.add_edge(ni, *lookup.get(&(i-1,j)).unwrap(), w);
            }
        }
    } 

      
    let res = dijkstra(&g, *lookup.get(&(0,0)).unwrap(),Some(*lookup.get(&end_node).unwrap()), |x| *x.weight());
    println!("{}", res.get(lookup.get(&end_node).unwrap()).unwrap());
    /* 
    for (r, row)in board.iter().enumerate() {
        for (c, entry) in row.iter().enumerate() {
            //print!("{}", entry);
            print!("{}", entry);
        }
        println!();
    }
    */
}
