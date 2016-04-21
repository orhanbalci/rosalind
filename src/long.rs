extern crate petgraph;
mod ProteinUtilities;
use petgraph::graph::{Graph, NodeIndex};
use std::collections::HashMap;
use petgraph::algo::toposort;

fn main() {
    let mut overlap_graph = petgraph::Graph::<String, usize>::new();
    let mut node_index_map = HashMap::<String, NodeIndex>::new();

    let dnas = ProteinUtilities::read_fasta(".\\src\\long.txt");
    for i in 0..dnas.len() {
        for j in i + 1..dnas.len() {
            add_overlapping_vertices(&dnas, i, j, &mut node_index_map, &mut overlap_graph);
            add_overlapping_vertices(&dnas, j, i, &mut node_index_map, &mut overlap_graph);
        }
    }

    let sorted_vertices = toposort(&overlap_graph);
    // sorted_vertices.iter()
    //               .inspect(|&&x| println!("{:?}", overlap_graph.node_weight(x).unwrap()))
    //               .count();
    let result = sorted_vertices.iter()
                                .map(|&x| overlap_graph.node_weight(x).unwrap())
                                .fold(String::new(), |mut accu, node_weight| {
                                    if accu.len() == 0 {
                                        accu.push_str(node_weight);
                                    } else {
                                        concat_overlapping(&mut accu, node_weight.clone());
                                    }
                                    accu
                                });
    println!("{}", result);
}

fn add_overlapping_vertices(dnas: &Vec<String>,
                            i: usize,
                            j: usize,
                            node_index_map: &mut HashMap<String, NodeIndex>,
                            overlap_graph: &mut Graph<String, usize>) {
    let overlap_size = overlaps(&dnas[i], &dnas[j]);
    if overlap_size >= dnas[j].len() / 2 {
        // println!("Overlaping pairs {}-{}", dnas[i], dnas[j]);
        let ith_index: NodeIndex;
        let jth_index: NodeIndex;
        if !node_index_map.contains_key(&dnas[i][..]) {
            ith_index = overlap_graph.add_node(dnas[i].clone());
            node_index_map.insert(dnas[i].clone(), ith_index);
        } else {
            ith_index = *node_index_map.get(&dnas[i][..]).unwrap();
        }

        if !node_index_map.contains_key(&dnas[j][..]) {
            jth_index = overlap_graph.add_node(dnas[j].clone());
            node_index_map.insert(dnas[j].clone(), jth_index);
        } else {
            jth_index = *node_index_map.get(&dnas[j][..]).unwrap();
        }
        overlap_graph.add_edge(ith_index, jth_index, overlap_size);
    }
}

fn concat_overlapping(first_sequence: &mut String, second_sequence: String) {
    let overlap_size = overlaps(&first_sequence, &second_sequence);
    first_sequence.push_str(&second_sequence[overlap_size..]);
}

fn overlaps(first_sequence: &str, second_sequence: &str) -> usize {
    for i in (0..second_sequence.len() + 1).rev() {
        if ends_with(first_sequence, &second_sequence[0..i]) {
            return i;
        }
    }
    0
}

fn ends_with(sequence: &str, ending_sequence: &str) -> bool {
    if ending_sequence.len() > sequence.len() {
        return false;
    } else {
        sequence[sequence.len() - ending_sequence.len()..sequence.len()] == ending_sequence[..]
    }
}

#[test]
fn test_ends_with() {
    assert!(ends_with("orhan", "han"))
}

#[test]
fn test_overlaps() {
    assert_eq!(overlaps("busra", "rakip"), 2)
}
