use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use petgraph::graph::{UnGraph, NodeIndex};
use petgraph::prelude::*;

fn main() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let lines = br.lines();

    let mut g: Graph<String, (), Undirected> = UnGraph::new_undirected();
    let mut weight_to_index: HashMap<String, NodeIndex> = HashMap::new();
    let mut start_index = NodeIndex::new(0);

    for line in lines {
        let l = line.unwrap();
        let input_nodes: Vec<String> = l.split('-').map(|x|x.to_owned()).collect();
        let edge = (input_nodes[0].clone(), input_nodes[1].clone());

        let start_node_index = {
            if weight_to_index.contains_key(&edge.0) {
                *weight_to_index.get(&edge.0).unwrap()
            } else {
                let ni = g.add_node(edge.0.clone());
                weight_to_index.insert(edge.0.clone(), ni);
                ni
            }
        };

        let end_node_index = {
            if weight_to_index.contains_key(&edge.1) {
                *weight_to_index.get(&edge.1).unwrap()
            } else {
                let ni = g.add_node(edge.1.clone());
                weight_to_index.insert(edge.1.clone(), ni);
                ni
            }
        };


        if edge.0 == "start" {
            start_index = start_node_index;
        }
        if edge.1 == "start" {
            start_index = end_node_index;
        }
        

        g.add_edge(start_node_index, end_node_index, ());
    }

    //let mut current_path: Vec<String> = Vec::new();
    let mut all_paths: Vec<Vec<String>> = Vec::new();
    let mut stack: Vec<(Vec<String>, NodeIndex, bool)> = Vec::new();    

    stack.push((Vec::new(),start_index, false));
    while let Some((mut current_path, ni, visited_twice)) = stack.pop() {
        let val = g.node_weight(ni).unwrap().clone();
        current_path.push(val.clone());
        
        if val == "end" {
            all_paths.push(current_path.clone());
            current_path.pop(); // remove end
            current_path.pop(); // remove last element

        } else {
            let neighbors = g.neighbors(ni);
            for neighbor in neighbors { 
                let nv = g.node_weight(neighbor).unwrap().clone();
                if nv != "start" {
                    if nv.to_uppercase() == nv {
                        stack.push((current_path.clone(), neighbor, visited_twice));
                    } else if current_path.contains(&nv) && !visited_twice {
                        stack.push((current_path.clone(), neighbor, true));
                    } else if !current_path.contains(&nv) {
                        stack.push((current_path.clone(), neighbor, visited_twice));
                    }
                        
                }
            }
        }
    }
   
    println!("{}", all_paths.len());
}
