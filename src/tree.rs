//#![feature(plugin)]
extern crate petgraph;
mod ProteinUtilities;
use petgraph::graph::{Graph, NodeIndex};
use std::collections::HashMap;
use petgraph::algo::connected_components;

fn main() {
    let mut node_count = 0_u32;
    let graphs = read_graph_file(".\\src\\tree.txt", &mut node_count);

    let mut graph_data = Graph::<String, usize>::new();
    let mut node_index_map = HashMap::<String, NodeIndex>::new();
    for node in graphs.iter() {
        // println!("Adding vertex {:?}", node);
        add_vertex(node, &mut node_index_map, &mut graph_data);
    }

    for i in 1..node_count + 1 {
        if !node_index_map.contains_key(&i.to_string()) {
            let temp_index = graph_data.add_node(i.to_string());
            node_index_map.insert(i.to_string(), temp_index);
        }
    }

    let connected_count = connected_components(&graph_data);
    println!("{}", connected_count - 1);
}

fn add_vertex(vertices: &(String, String),
              node_index_map: &mut HashMap<String, NodeIndex>,
              overlap_graph: &mut Graph<String, usize>) {

    let ith_index: NodeIndex;
    let jth_index: NodeIndex;
    if !node_index_map.contains_key(&vertices.0[..]) {
        ith_index = overlap_graph.add_node(vertices.0.clone());
        node_index_map.insert(vertices.0.clone(), ith_index);
    } else {
        ith_index = *node_index_map.get(&vertices.0[..]).unwrap();
    }

    if !node_index_map.contains_key(&vertices.1[..]) {
        jth_index = overlap_graph.add_node(vertices.1.clone());
        node_index_map.insert(vertices.1.clone(), jth_index);
    } else {
        jth_index = *node_index_map.get(&vertices.1[..]).unwrap();
    }
    overlap_graph.add_edge(ith_index, jth_index, 1);
}

pub fn read_graph_file(file_name: &'static str, node_count: &mut u32) -> Vec<(String, String)> {
    let mut s = String::new();
    ProteinUtilities::read_file_into_string(file_name, &mut s);
    let raw_lines: Vec<&str> = s.trim().split("\r\n").collect();
    let mut graph: Vec<(String, String)> = Vec::new();

    for temp in raw_lines.iter() {
        let nodes = temp.trim().split(" ").collect::<Vec<&str>>();
        // println!("{:?}",nodes);
        match nodes.len() {
            1 => *node_count = nodes[0].parse::<u32>().unwrap(),
            2 => graph.push((nodes[0].to_owned(), nodes[1].to_owned())),
            _ => unreachable!(),
        }
    }
    graph
}
