extern crate petgraph;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::algo::is_cyclic_directed;


fn main() {
    println!("Hello, dag!");
    let graphs = read_graph_file(".\\src\\dag.txt");
    for graph in &graphs {
        if graph.is_empty() {
            continue;
        }

        let mut graph_data = Graph::<String, usize>::new();
        let mut node_index_map = HashMap::<String, NodeIndex>::new();
        for node in graph.iter().skip(1){
            //println!("Adding vertex {:?}", node);
            add_vertex(node,&mut node_index_map, &mut graph_data);
        }
        match is_cyclic_directed(&graph_data) {
            true => print!("-1 "),
            false => print!("1 ")
        }

    }
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

pub fn read_graph_file(file_name: &'static str) -> Vec<Vec<(String, String)>> {
    let mut s = String::new();
    read_file_into_string(file_name, &mut s);
    let raw_lines: Vec<&str> = s.split("\r\n").collect();
    let mut graphs: Vec<Vec<(String, String)>> = Vec::new();

    for temp in raw_lines.iter().skip(1) {
        match temp.len() {
            0 => graphs.push(Vec::new()),
            _ => {
                let nodes = temp.split(" ").collect::<Vec<&str>>();
                graphs.last_mut().unwrap().push((nodes[0].to_owned(), nodes[1].to_owned()));
            }
        }
    }
    graphs
}

pub fn read_file_into_string(file_name: &'static str, file_content: &mut String) {
    let path = Path::new(file_name);
    let display = path.display();


    let mut file = match File::open(&path) {
        Err(why) => panic!("Could not open {} : {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    match file.read_to_string(file_content) {
        Err(why) => panic!("Could not read {} : {}", display, Error::description(&why)),
        Ok(_) => {}//println!("File contains {}",file_content),
    };
}
